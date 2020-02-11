use crate::create_buttplug_protocol;

create_buttplug_protocol!(
    // Protocol name,
    Youou,
    // Protocol members
    (
        (packet_id: Arc<Mutex<u8>> = Arc::new(Mutex::new(0)))
    ),
    (
        (VibrateCmd, {
            // TODO Convert to using generic command manager
    
            // Byte 2 seems to be a monotonically increasing packet id of some kind Speed seems to be
            // 0-247 or so. Anything above that sets a pattern which isn't what we want here.
            let max_value: f64 = 247.0;
            let speed: u8 = (msg.speeds[0].speed * max_value) as u8;
            let state: u8 = if speed > 0 { 1 } else { 0 };

            let mut data;
            {
                let mut packet_id = self.packet_id.lock().await;
                data = vec![0xaa, 0x55, *packet_id, 0x02, 0x03, 0x01, speed, state];
                *packet_id = packet_id.wrapping_add(1);
            }
            let mut crc: u8 = 0;
    
            // Simple XOR of everything up to the 9th byte for CRC.
            for b in data.clone() {
                crc = b ^ crc;
            }
    
            let mut data2 = vec![crc, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
            data.append(&mut data2);
    
            // Hopefully this will wrap back to 0 at 256
            // self.packet_id = self.packet_id.wrapping_add(1);
    
            let msg = DeviceWriteCmd::new(Endpoint::Tx, data, false);
            device.write_value(msg.into()).await?;
            Ok(ButtplugMessageUnion::Ok(messages::Ok::default()))
        })
    )
);

#[cfg(test)]
mod test {
    use crate::{
        core::messages::{VibrateCmd, VibrateSubcommand, StopDeviceCmd},
        test::{check_recv_value, TestDevice},
        device::{
            Endpoint,
            device::{DeviceImplCommand, DeviceWriteCmd},
        }
    };
    use async_std::task;
    
    #[test]
    pub fn test_youou_protocol() {
        task::block_on(async move {
            let (mut device, test_device) = TestDevice::new_bluetoothle_test_device("VX001_").await.unwrap();
            device.parse_message(&VibrateCmd::new(0, vec!(VibrateSubcommand::new(0, 0.5))).into()).await.unwrap();
            let (_, command_receiver) = test_device.get_endpoint_channel_clone(&Endpoint::Tx).await;
            check_recv_value(&command_receiver, 
                DeviceImplCommand::Write(
                        DeviceWriteCmd::new(Endpoint::Tx, 
                                            vec![0xaa, 0x55, 0x00, 0x02, 0x03, 0x01, (247.0f32 / 2.0f32) as u8, 0x01, 0x85, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 
                                            false))).await;
            // Test a cloned device to make sure we handle packet IDs across protocol clones correctly.
            let mut device2 = device.clone();
            device2.parse_message(&StopDeviceCmd::new(0).into()).await.unwrap();
            check_recv_value(&command_receiver, 
                DeviceImplCommand::Write(
                        DeviceWriteCmd::new(Endpoint::Tx, 
                                            vec![0xaa, 0x55, 0x01, 0x02, 0x03, 0x01, 0x00, 0x00, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 
                                            false))).await;
        });
    }
}