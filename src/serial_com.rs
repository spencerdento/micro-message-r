use serial::{PortSettings, prelude::*, windows::COMPort};
use std::io;
use std::time::Duration;

pub fn init_serial() -> anyhow::Result<COMPort> {
    let mut port = serial::open("COM4")?;
    let settings = PortSettings {
        baud_rate: serial::BaudRate::Baud1200,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&settings)?;

    Ok(port)
}

// pub fn write_one_message (port: &COMPort) {
    
// }
