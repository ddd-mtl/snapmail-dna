// use hdk::prelude::*;

use hdk::{
    holochain_core_types::entry::Entry,
    holochain_json_api::json::JsonString,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{entry_kind, signal_protocol::*, mail::{
    self,
    entries::{InMail, MailItem, MailState},
}, AgentAddress, DirectMessageProtocol, MailMessage, AckMessage, ReceivedAck, snapmail_now};
use std::convert::TryInto;
use crate::mail::entries::InMailState;


pub fn receive(from: Address, msg_json: JsonString) -> String {
    hdk::debug(format!("Received from: {:?}", from)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = msg_json.try_into();
    if let Err(err) = maybe_msg {
        return format!("error: {}", err);
    }
    let message = match maybe_msg.unwrap() {
        DirectMessageProtocol::Mail(mail) => {
            mail::receive_direct_mail(from, mail)
        },
        DirectMessageProtocol::Ack(ack) => {
            mail::receive_direct_ack(from, ack)
        }
        DirectMessageProtocol::Ping => {
            DirectMessageProtocol::Success(String::new())
        }
        _ => {
            let response = DirectMessageProtocol::Failure("Unexpected protocol".to_owned());
            return serde_json::to_string(&response).expect("Should stringify");
        },
    };
    let msg_json = serde_json::to_string(&message).expect("Should stringify");
     msg_json
}


/// Handle a MailMessage.
/// Emits `received_mail` signal.
/// Returns Success or Failure.
pub fn receive_direct_mail(from: AgentAddress, mail_msg: MailMessage) -> DirectMessageProtocol {
    // Create InMail
    let inmail = InMail::from_direct(from.clone(), mail_msg.clone());
    let inmail_entry = Entry::App(entry_kind::InMail.into(), inmail.into());
    let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
    if let Err(err) = maybe_inmail_address {
        let response_str = "Failed committing InMail";
        hdk::debug(format!("{}: {}", response_str, err)).ok();
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let inmail_address =  maybe_inmail_address.unwrap();
    hdk::debug(format!("inmail_address: {}", inmail_address)).ok();

    // Emit signal
    let item = MailItem {
        address: inmail_address,
        author: from.clone(),
        mail: mail_msg.mail.clone(),
        state: MailState::In(InMailState::Arrived),
        bcc: Vec::new(),
        date: snapmail_now() as i64, // FIXME
    };
    let signal = SignalProtocol::ReceivedMail(item);
    let signal_json = serde_json::to_string(&signal).expect("Should stringify");
    let res = hdk::emit_signal("received_mail", JsonString::from_json(&signal_json));
    if let Err(err) = res {
        hdk::debug(format!("Emit signal failed: {}", err)).ok();
    }
    // Return Success response
    return DirectMessageProtocol::Success(String::new());
}

/// Handle a AckMessage.
/// Emits `received_ack` signal.
/// Returns Success or Failure.
pub fn receive_direct_ack(from: AgentAddress, ack_msg: AckMessage) -> DirectMessageProtocol {
    // Create InAck
    let res = mail::create_and_commit_inack(&ack_msg.outmail_address, &from);
    if let Err(err) = res {
        let response_str = "Failed committing InAck";
        hdk::debug(format!("{}: {}", response_str, err)).ok();
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    // Emit Signal
    let signal = SignalProtocol::ReceivedAck(ReceivedAck {
        from: from.clone(),
        for_mail: ack_msg.outmail_address.clone(),
    });
    let signal_json = serde_json::to_string(&signal).expect("Should stringify");
    let res = hdk::emit_signal("received_ack", JsonString::from_json(&signal_json));
    if let Err(err) = res {
        hdk::debug(format!("Emit signal failed: {}", err)).ok();
    }
    // Return Success response
    return DirectMessageProtocol::Success(String::new());
}
