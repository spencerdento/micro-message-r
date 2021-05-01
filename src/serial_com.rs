use serial::{PortSettings, prelude::*, windows::COMPort};
use std::{ffi::OsStr, io::{Read, Write}};


pub fn init_serial() -> anyhow::Result<COMPort> {
    let mut port = serial::open(OsStr::new("COM5"))?;
    const SETTINGS: PortSettings = PortSettings {
        baud_rate: serial::BaudRate::Baud1200,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&SETTINGS)?;

    Ok(port)
}

pub fn write_one_message(port: &mut COMPort, message: &[u8]) -> anyhow::Result<usize> {
    let tx_len = port.write(message)?;
    assert_eq!(tx_len, message.len());
    port.flush()?;
    Ok(tx_len)
}

pub fn read_one_message(port: &mut COMPort) -> (anyhow::Result<Vec<u8>>, i32) {
    let mut error_count = 0;
    let mut buf = Vec::new();
    let mut iter = port.bytes();
    let mut stx_semaphore: bool = false;

    loop {
        let next_byte = iter.next();

        match next_byte {
            Some(byte) => {
                match byte {
                    Ok(character) => {
                        if character == 0x02 {
                            stx_semaphore = true;
                            continue;
                        } else if character == 0x03 {
                            if stx_semaphore == false {
                                buf = Vec::new();
                                continue;
                            } else {
                                return (Ok(buf), error_count);
                            }
                        } else if stx_semaphore == true {
                            buf.push(character);
                            continue;
                        }
                    },
                    Err(_) => {
                        error_count+=1;
                        continue;
                    },
                };
            },
            None => break,
        };
    }
    (Err(anyhow::Error::msg(error_count.to_string())), error_count)
}
