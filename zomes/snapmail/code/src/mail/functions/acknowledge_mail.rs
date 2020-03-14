// use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    AgentAddress,
    protocol::{DirectMessageProtocol, AckMessage},
    mail::{
        entries::{
            InMail, PendingAck, OutAck,
        }
    },
};

/// Zome function
/// Return address of newly created OutAck
pub fn acknowledge_mail(inmail_address: Address) -> ZomeApiResult<Address> {
    //  1. Make sure its an InMail
    let inmail = hdk::utils::get_as_type::<InMail>(inmail_address)?;
    //  2. Make sure it has not already been acknowledged
    let res = hdk::get_links_count(&inmail_address, LinkMatch::Exactly("acknowledgment"), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged".to_string()));
    }
    // 3. Write OutAck
    let outack = OutAck::new();
    let outack_entry = Entry::App("outack".into(), outack.into());
    let outack_address = hdk::commit_entry(&outack_entry)?;
    let _ = hdk::link_entries(&inmail_address, &outack_address, "acknowledgment", "")?;
    // 4. Try Direct sharing of Acknowledgment
    let res = acknowledge_mail_direct(&inmail.outmail_address, &inmail.from);
    if res.is_ok() {
        return Ok(outack_address);
    }
    let err = res.err().unwrap();
    hdk::debug(format!("Direct sharing of Acknowledgment failed: {}", err));
    // 5. Otherwise share Acknowledgement via DHT
    let _ = acknowledge_mail_pending(&outack_address, &inmail.outmail_address, &inmail.from);
    Ok(outack_address)
}

/// Try sending directly to other Agent if Online
fn acknowledge_mail_direct(outmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<()> {
    //   a. Create DM
    let msg = AckMessage {
        outmail_address: outmail_address.clone(),
    };
    let payload = serde_json::to_string(&DirectMessageProtocol::Ack(msg)).unwrap();
    //   b. Send DM
    let result = hdk::send(
        from.clone(),
        payload,
        crate::DIRECT_SEND_TIMEOUT_MS.into(),
    );
    if let Err(err) = result {
        return Err(err);
    }
    //   c. Check Response
    let response = result.unwrap();
    hdk::debug(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(err) = maybe_msg {
        return Err(ZomeApiError::Internal(format!("{}", err)));
    }
    match maybe_msg.unwrap() {
        DirectMessageProtocol::Success(_) => Ok(()),
        _ => Err(ZomeApiError::Internal("Failed".to_string())),
    }
}

/// Create & Commit PendingAck
/// Return address of newly created PendingAck
/// Return PendingAck's address
fn acknowledge_mail_pending(outack_address: &Address, outmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    let pending_ack = PendingAck::new(outmail_address.clone());
    let pending_ack_entry = Entry::App("pending_ack".into(), pending_ack.into());
    let pending_ack_address = hdk::commit_entry(&pending_ack_entry)?;
    let _ = hdk::link_entries(&outack_address, &pending_ack_address, "pending", "")?;
    let _ = hdk::link_entries(&from, &pending_ack_address, "ack_inbox", &*hdk::AGENT_ADDRESS.to_string())?;
    Ok(pending_ack_address)
}
