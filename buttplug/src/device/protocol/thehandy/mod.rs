use super::{ButtplugDeviceResultFuture, ButtplugProtocol, ButtplugProtocolCommandHandler};
use crate::{
  core::{
    errors::ButtplugError,
    messages::{self, ButtplugDeviceCommandMessageUnion, DeviceMessageAttributesMap},
  },
  device::{
    protocol::{generic_command_manager::GenericCommandManager, ButtplugProtocolProperties},
    DeviceImpl,
    DeviceWriteCmd,
    DeviceReadCmd,
    Endpoint,
  },
};
use futures::future::{self, BoxFuture};
use messages::RequestServerInfo;
use std::sync::Arc;
use tokio::sync::Mutex;
use prost::Message;

mod protocomm {
  include!(concat!(env!("OUT_DIR"), "/protocomm.rs"));
}

mod handyplug {
  include!(concat!(env!("OUT_DIR"), "/handyplug.rs"));
}

#[derive(ButtplugProtocolProperties)]
pub struct TheHandy {
  name: String,
  message_attributes: DeviceMessageAttributesMap,
  manager: Arc<Mutex<GenericCommandManager>>,
  stop_commands: Vec<ButtplugDeviceCommandMessageUnion>,
}


impl ButtplugProtocol for TheHandy {
  fn new_protocol(
    name: &str,
    message_attributes: DeviceMessageAttributesMap,
  ) -> Box<dyn ButtplugProtocol>
  where
    Self: Sized,
  {
    let manager = GenericCommandManager::new(&message_attributes);

    Box::new(Self {
      name: name.to_owned(),
      message_attributes,
      stop_commands: manager.get_stop_commands(),
      manager: Arc::new(Mutex::new(manager)),
    })
  }

  fn initialize(device_impl: Arc<DeviceImpl>) -> BoxFuture<'static, Result<Option<String>, ButtplugError>>
  where
      Self: Sized, 
  {
    Box::pin(async move {
      // Ok, here we go. This is a fucking nightmare but apparently "protocomm
      // makes the firmware easier".
      //
      // I will remember this, Maike.
      //
      // This code is mostly my translation of the Handy Python POC. If they ever
      // change anything, I quit.
  
      // First we need to set up a session with The Handy. This will require
      // sending the "security initializer" to basically say we're sending
      // plaintext. Due to pb3 making everything optional, we have some Option<T>
      // wrappers here.
      let payload = protocomm::Sec0Payload {
        msg: protocomm::Sec0MsgType::S0SessionCommand as i32,
        payload: Some(protocomm::sec0_payload::Payload::Sc(protocomm::S0SessionCmd {}))
      };
      let proto = Some(protocomm::session_data::Proto::Sec0(payload));
      let session_req = protocomm::SessionData {
        sec_ver: protocomm::SecSchemeVersion::SecScheme0 as i32,
        proto
      };
  
      // We need to shove this at what we're calling the "firmware" endpoint but
      // what is actually the "prov-session" endpoint. These names are stored in
      // characteristic descriptors, which is new and novel. However I don't
      // have to do characteristic descriptor lookups for the other 140+ pieces
      // of hardware this library supports so I'm damn well not doing it now.
      let mut sec_buf = vec![];
      session_req.encode(&mut sec_buf).unwrap();
      info!("Writing security packet");
      device_impl.write_value(DeviceWriteCmd::new(Endpoint::Firmware, sec_buf, false));
      info!("Reading back security info");
      let _ = device_impl.read_value(DeviceReadCmd::new(Endpoint::Firmware, 100, 500));

      
      // TODO We should read the reply from this just to make sure we've
      // established correctly.
      let rsi_payload = handyplug::RequestServerInfo {
        // You know when IDs are important? When you have a protocol that
        // handles multiple asynchronous commands. You know what doesn't handle
        // multiple asynchronous commands? The handyplug protocol.
        //
        // Do you know where you'd pack those? In the top level container, as
        // they should then be separate from the message context, in order to
        // allow multiple sorters. Do you know what doesn't need multiple
        // sorters? The handyplug protocol.
        //
        // Please do not cargo cult protocols.
        id: 1,
        client_name: "".to_owned(),
        message_version: 0
      };
      let rsi_message = handyplug::Message {
        message: Some(handyplug::message::Message::RequestServerInfo(rsi_payload))
      };
      let mut rsi_buf = vec!();
      rsi_message.encode(&mut rsi_buf).unwrap();
      info!("Writing RSI");
      device_impl.write_value(DeviceWriteCmd::new(Endpoint::Tx, rsi_buf, false)).await.unwrap();

      // Ok, now we read back from the same endpoint. Ignore the weirdness of
      // reading from Tx. This is a cloaca endpoint.
      info!("Trying to read SI");
      if let Ok(data) = device_impl.read_value(DeviceReadCmd::new(Endpoint::Tx, 256, 500)).await {
        // There is probably a better way to do this cast
        let si_msg = handyplug::Message::decode(&data.data()[0..data.data().len()]).unwrap();
        info!("SI cast worked?");
        if let handyplug::message::Message::ServerInfo(si) = si_msg.message.unwrap() {
          info!("Got handy info back: {:?}", si);
        } else {
          error!("Not a server info message!");
        }
      } else {
        error!("Can't read value!");
      }

      Ok(Some("The Handy".to_string()))
    })
  }
}

impl ButtplugProtocolCommandHandler for TheHandy {
}