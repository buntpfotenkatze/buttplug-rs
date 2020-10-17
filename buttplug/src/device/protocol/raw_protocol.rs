use super::{ButtplugProtocol, ButtplugProtocolCommandHandler};
use crate::{
  core::messages::{ButtplugDeviceCommandMessageUnion, MessageAttributesMap},
  device::protocol::ButtplugProtocolProperties,
};

#[derive(ButtplugProtocolProperties)]
pub struct RawProtocol {
  name: String,
  message_attributes: MessageAttributesMap,
  stop_commands: Vec<ButtplugDeviceCommandMessageUnion>,
}

impl ButtplugProtocol for RawProtocol {
  fn new_protocol(
    name: &str,
    message_attributes: MessageAttributesMap,
  ) -> Box<dyn ButtplugProtocol> {
    Box::new(Self {
      name: name.to_owned(),
      message_attributes,
      stop_commands: vec![],
    })
  }
}

impl ButtplugProtocolCommandHandler for RawProtocol {
}

// TODO Write tests
