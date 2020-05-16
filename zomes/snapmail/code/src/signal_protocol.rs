use hdk::prelude::*;
use hdk::holochain_persistence_api::cas::content::Address;

use crate::{
    AgentAddress,
    mail::entries::MailItem,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum SignalProtocol {
    ReceivedMail(MailItem),
    ReceivedAck(ReceivedAck),
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct ReceivedMail {
//     pub item: MailItem,
// }

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ReceivedAck {
    pub from: AgentAddress,
    pub for_mail: Address,
}