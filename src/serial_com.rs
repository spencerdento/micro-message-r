// use lettre::error;
use serial::{prelude::*, windows::COMPort, PortSettings};
use std::{
    ffi::OsStr,
    io::{Read, Write},
};

const STX: u8 = 0x02;
const ETX: u8 = 0x03;

pub fn init_serial() -> anyhow::Result<COMPort> {
    let mut port = serial::open(OsStr::new("COM5"))?;
    const SETTINGS: PortSettings = PortSettings {
        baud_rate: serial::BaudRate::Baud110,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&SETTINGS)?;

    Ok(port)
}

pub fn _write_one_message(port: &mut COMPort, message: &[u8]) -> anyhow::Result<usize> {
    let tx_len = port.write(message)?;
    assert_eq!(tx_len, message.len());
    port.flush()?;
    Ok(tx_len)
}

pub fn read_one_message_old(port: &mut COMPort) -> (anyhow::Result<Vec<u8>>, i32) {
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
                    }
                    Err(_) => {
                        error_count += 1;
                        continue;
                    }
                };
            }
            None => break,
        };
    }
    (
        Err(anyhow::Error::msg(error_count.to_string())),
        error_count,
    )
}

pub fn _read_one_message(
    port: &mut COMPort,
    message_length: i32,
) -> (anyhow::Result<Vec<char>>, i32) {
    let mut error_count = 0;
    let mut character_count = 1;
    let mut buf: Vec<char> = Vec::new();
    let iter = port.bytes();
    let mut stx_flag: bool = false;

    for components in iter {
        if let Err(_) = components {
            println!("Starting over");
            stx_flag = false;
            error_count += 1;
            buf = Vec::new();
            character_count = 0;
        } else if let Ok(hex_in) = components {
            if hex_in == STX {
                if stx_flag == true {
                    buf = Vec::new();
                    character_count = 1;
                } else {
                    stx_flag = true;
                }
            } else if hex_in == ETX {
                if character_count == message_length {
                    return (Ok(buf), error_count);
                } else {
                    println!("End: {}", character_count);
                    stx_flag = false;
                    error_count += 1;
                    buf = Vec::new();
                    character_count = 1;
                }
            } else {
                character_count += 1;
                buf.push(hex_in as char);
            }
        }
    }

    (
        Err(anyhow::Error::msg(error_count.to_string())),
        error_count,
    )
}

pub fn _read_one_message_exact(port: &mut COMPort) -> anyhow::Result<String> {
    let mut buf: [u8; 32] = [0; 32];
    let mut count = 0;
    let mut message = String::new();

    port.read_exact(&mut buf)?;

    let mut data = buf.get(count).unwrap_or(&b'*');
    count += 1;
    while *data != STX && count <= 32 {
        data = buf.get(count).unwrap_or(&b'*');
        count += 1;
    }

    data = buf.get(count).unwrap_or(&b'*');
    count += 1;

    while *data != ETX && count <= 32 {
        //need to check for STX, could be invalid read
        message.push(*data as char);
        data = buf.get(count).unwrap_or(&b'*');
        count += 1;
    }

    Ok(message)
}

pub fn _read_port(port: &mut COMPort, mimimum_command_length: usize) -> anyhow::Result<String> {
    let mut buf: [u8; 100] = [0; 100];
    let mut stx_index: i32 = -1;

    let num_bytes_read = port.read(&mut buf)?;

    if num_bytes_read < (mimimum_command_length + 2) {
        return Err(anyhow::Error::msg(String::from("Not enough info")));
    }

    //find STX
    let mut i = 0;
    for octet in &buf[..num_bytes_read] {
        if *octet == STX {
            stx_index = i;
            break;
        }
        i += 1;
    }
    //stx_index == -1 if not found
    if stx_index == -1 {
        return Err(anyhow::Error::msg(String::from("STX not found")));
    }
    let stx_index: usize = stx_index as usize;

    //get the stuff until the ETX -- if end is reached first an error will be returned
    let mut message = String::new();
    for octet in &buf[(stx_index + 1)..num_bytes_read] {
        if *octet != ETX {
            message.push(*octet as char);
        } else {
            return Ok(message);
        }
    }
    Err(anyhow::Error::msg("ETX not found"))
}
