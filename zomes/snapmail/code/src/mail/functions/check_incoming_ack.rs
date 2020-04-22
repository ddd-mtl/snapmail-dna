//use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::mail;
use crate::link_kind;

/// Return list of outMail addresses for which we succesfully linked a new InAck out of PendingAcks
pub fn check_incoming_ack() -> ZomeApiResult<Vec<Address>> {
    let maybe_my_handle_address = crate::handle::get_my_handle_entry();
    if let None = maybe_my_handle_address {
        return Err(ZomeApiError::Internal("This agent does not have a Handle set up".to_string()));
    }
    let my_handle_address = maybe_my_handle_address.unwrap().0;
    // Lookup `ack_inbox` links on my agentId
    let links_result = hdk::get_links(
        // &*hdk::AGENT_ADDRESS,
        &my_handle_address,
        LinkMatch::Exactly(link_kind::AckInbox),
        LinkMatch::Any)?;
    // For each link
    let mut updated_outmails = Vec::new();
    for pending_ack_address in &links_result.addresses() {
        //  - Get entry on the DHT
        let maybe_pending_ack = mail::get_pending_ack(pending_ack_address);
        if let Err(err) = maybe_pending_ack {
            hdk::debug(format!("Getting PendingAck from DHT failed: {}", err)).ok();
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        // Create InAck
        let maybe_inack_address = mail::create_and_commit_inack(&pending_ack.outmail_address, &author);
        if let Err(err) = maybe_inack_address {
            hdk::debug(format!("Creating InAck from PendignAck failed: {}", err)).ok();
            continue;
        }
        //  - Delete link from my agentId
        let res = hdk::remove_link(
            // *hdk::AGENT_ADDRESS,
            &my_handle_address,
            &pending_ack_address,
            link_kind::AckInbox,
            "",
        );
        if let Err(err) = res {
            hdk::debug("Remove ``ack_inbox`` link failed:").ok();
            hdk::debug(err).ok();
            continue;
        }
        // Delete PendingAck
        let res = hdk::remove_entry(pending_ack_address);
        if let Err(err) = res {
            hdk::debug(format!("Delete PendignAck failed: {}", err)).ok();
        }
        // Add to return list
        updated_outmails.push(pending_ack.outmail_address.clone());
    }
    Ok(updated_outmails)
}