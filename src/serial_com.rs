use serial::{PortSettings, prelude::*, windows::COMPort};
use std::{ffi::OsStr, io:: Write};


pub fn init_serial() -> anyhow::Result<COMPort> {
    let mut port = serial::open(OsStr::new("COM5"))?;
    let settings = PortSettings {
        baud_rate: serial::BaudRate::Baud115200,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&settings)?;

    Ok(port)
}

pub fn write_one_message(port: &mut COMPort, message: &[u8]) -> anyhow::Result<usize> {
    let tx_len = port.write(message)?;
    assert_eq!(tx_len, message.len());
    port.flush()?;
    Ok(tx_len)
}
