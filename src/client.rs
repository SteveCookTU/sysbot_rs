use crate::types::thread_message::ThreadMessage;
use crate::types::{
    Button, ConfigureOption, PeekArgs, PokeArgs, PokeData, SeqParam, Stick, StickMovement,
};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread;
use std::thread::JoinHandle;

/// A client that sends and receives data from a sys-botbase server
///
/// The client is created and established with [`connect`] and will be all messages will be sent
/// and cleaned up when the client is dropped
///
/// [`connect`]: fn@crate::SysBotClient::connect
pub struct SysBotClient {
    sender: SyncSender<ThreadMessage>,
    receiver: Receiver<Vec<u8>>,
    worker: Option<JoinHandle<()>>,
}

impl SysBotClient {
    /// Creates and connects a SysBotClient to a TcpStream in a concurrent thread.
    ///
    /// # Arguments
    ///
    /// * `addr` - A string slice representing an IPv4 address
    /// * `port` - A port number for the specified address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysbot_rs::SysBotClient;
    /// match SysBotClient::connect("0.0.0.0", 6000) {
    ///     Ok(client) => {
    ///         // Do something with the client
    ///     }
    ///     Err(err) => {
    ///         panic!("{}", err);
    ///     }
    /// }
    /// ```
    pub fn connect(addr: &str, port: u16) -> Result<Self, &'static str> {
        let socket_addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::from_str(addr).map_err(|_| "Failed to convert ip address")?),
            port,
        );
        let (sender_in, receiver_in): (SyncSender<ThreadMessage>, Receiver<ThreadMessage>) =
            mpsc::sync_channel(0);
        let (sender_out, receiver_out): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let tcp_stream = TcpStream::connect(socket_addr).expect("Failed to connect to address");
        let worker = Some(thread::spawn(move || {
            let mut tcp_stream = tcp_stream;
            let sender_out = sender_out;
            let receiver_in = receiver_in;
            for message in receiver_in.iter() {
                if message.close {
                    return;
                } else {
                    let _ = tcp_stream
                        .write(message.message.as_bytes())
                        .expect("Failed to write to stream");
                    tcp_stream.flush().expect("Failed to flush stream");

                    if message.returns {
                        if message.size == 0 {
                            let mut buf = vec![0; 100];
                            tcp_stream
                                .read(&mut buf)
                                .expect("Failed to read from stream");
                            sender_out
                                .clone()
                                .send(buf)
                                .expect("Failed to send response over channel");
                        } else {
                            let mut buf = vec![0u8; message.size];
                            tcp_stream
                                .read_exact(&mut buf)
                                .expect("Failed to read from stream");
                            sender_out
                                .clone()
                                .send(buf)
                                .expect("Failed to send response over channel");
                        }
                    }
                }
            }
        }));

        Ok(Self {
            sender: sender_in,
            receiver: receiver_out,
            worker,
        })
    }

    fn receive(&self) -> Result<Vec<u8>, &'static str> {
        self.receiver
            .recv()
            .map_err(|_| "Failed to receive a response")
    }

    fn check_connected(&self) -> Result<(), &'static str> {
        if self.worker.is_none() {
            Err("SysBotClient not connected")
        } else {
            Ok(())
        }
    }

    fn send(
        &self,
        command: String,
        returns: bool,
        close: bool,
        size: usize,
    ) -> Result<(), &'static str> {
        self.sender
            .send(ThreadMessage {
                message: command + "\r\n",
                returns,
                close,
                size,
            })
            .map_err(|_| "Failed to send command")
    }

    fn hex_string_to_vec(string_bytes: Vec<u8>) -> Vec<u8> {
        string_bytes
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    u8::from_str_radix(&String::from_utf8_lossy(chunk), 16)
                        .map_err(|_| println!("{:?}", chunk))
                        .unwrap()
                } else {
                    0xa
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn peek(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peek 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false, args.size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn peek_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let mut total_size = 0;
        let args = args
            .into_iter()
            .map(|a| {
                total_size += a.size;
                format!("0x{:X} 0x{:X}", a.addr, a.size)
            })
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekMulti {}", args);
        self.send(command, true, false, total_size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn peek_absolute(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peekAbsolute 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false, args.size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn peek_absolute_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let mut total_size = 0;
        let args = args
            .into_iter()
            .map(|a| {
                total_size += a.size;
                format!("0x{:X} 0x{:X}", a.addr, a.size)
            })
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekAbsoluteMulti {}", args);
        self.send(command, true, false, total_size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn peek_main(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peekMain 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false, args.size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn peek_main_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let mut total_size = 0;
        let args = args
            .into_iter()
            .map(|a| {
                total_size += a.size;
                format!("0x{:X} 0x{:X}", a.addr, a.size)
            })
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekMainMulti {}", args);
        self.send(command, true, false, total_size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn poke(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("poke 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false, 0)
    }

    pub fn poke_absolute(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("pokeAbsolute 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false, 0)
    }

    pub fn poke_main(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("pokeMain 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false, 0)
    }

    pub fn click(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("click {}", button);
        self.send(command, false, false, 0)
    }

    pub fn click_seq(&self, args: Vec<SeqParam>) -> Result<(), &'static str> {
        self.check_connected()?;
        let args = args
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let command = format!("clickSeq {}", args);
        self.send(command, false, false, 0)
    }

    pub fn click_cancel(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "clickCancel".to_string();
        self.send(command, false, false, 0)
    }

    pub fn press(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("press {}", button);
        self.send(command, false, false, 0)
    }

    pub fn release(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("release {}", button);
        self.send(command, false, false, 0)
    }

    pub fn set_stick(&self, stick: Stick, movement: StickMovement) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!(
            "setStick {} {}",
            stick,
            movement.to_string().replace(',', " ")
        );
        self.send(command, false, false, 0)
    }

    pub fn detach_controller(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "detachController".to_string();
        self.send(command, false, false, 0)
    }

    pub fn configure(&self, option: ConfigureOption) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("configure {}", option);
        self.send(command, false, false, 0)
    }

    pub fn get_title_id(&self) -> Result<u64, &'static str> {
        self.check_connected()?;
        let command = "getTitleID".to_string();
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u32")?,
        ))
    }

    pub fn get_system_language(&self) -> Result<u8, &'static str> {
        self.check_connected()?;
        let command = "getSystemLanguage".to_string();
        self.send(command, true, false, 0)?;
        let string = String::from_utf8(self.receive()?).unwrap();
        let string = string.replace('\u{0000}', "");
        u8::from_str(string.trim()).map_err(|_| "Failed to parse string to u8")
    }

    pub fn get_main_nso_base(&self) -> Result<u64, &'static str> {
        self.check_connected()?;
        let command = "getMainNsoBase".to_string();
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn get_build_id(&self) -> Result<u64, &'static str> {
        self.check_connected()?;
        let command = "getBuildID".to_string();
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn get_heap_base(&self) -> Result<u64, &'static str> {
        self.check_connected()?;
        let command = "getHeapBase".to_string();
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn is_program_running(&self) -> Result<bool, &'static str> {
        self.check_connected()?;
        let command = "getHeapBase".to_string();
        self.send(command, true, false, 2)?;
        let string_bytes = self.receive()?;
        Ok(string_bytes[0] != 0)
    }

    pub fn get_version(&self) -> Result<String, &'static str> {
        self.check_connected()?;
        let command = "getVersion".to_string();
        self.send(command, true, false, 4)?;
        let string_bytes = self.receive()?;
        Ok(String::from_utf8(string_bytes)
            .map_err(|_| "Failed to parse response to string")?
            .trim()
            .to_string())
    }

    pub fn pointer(&self, jumps: &[u64]) -> Result<u64, &'static str> {
        self.check_connected()?;
        let mut command = "pointer".to_string();
        for jump in jumps {
            command = format!("{} 0x{:X}", command, jump)
        }
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn pointer_all(&self, jumps: &[u64]) -> Result<u64, &'static str> {
        self.check_connected()?;
        let mut command = "pointerAll".to_string();
        for jump in jumps {
            command = format!("{} 0x{:X}", command, jump)
        }
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn pointer_relative(&self, jumps: &[u64]) -> Result<u64, &'static str> {
        self.check_connected()?;
        let mut command = "pointerAll".to_string();
        for jump in jumps {
            command = format!("{} 0x{:X}", command, jump)
        }
        self.send(command, true, false, 17)?;
        let bytes = SysBotClient::hex_string_to_vec(self.receive()?);
        Ok(u64::from_be_bytes(
            (&bytes[0..8])
                .try_into()
                .map_err(|_| "Failed to parse bytes to u64")?,
        ))
    }

    pub fn pointer_peek(&self, jumps: &[u64], size: usize) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let mut command = format!("pointerPeek 0x{:X}", size);
        for jump in jumps {
            command = format!("{} 0x{:X}", command, jump);
        }
        self.send(command, true, false, size * 2 + 1)?;
        Ok(SysBotClient::hex_string_to_vec(self.receive()?))
    }

    pub fn pointer_poke(&self, jumps: &[u64], data: PokeData) -> Result<(), &'static str> {
        self.check_connected()?;
        let mut command = format!("pointerPoke {}", data.to_string());
        for jump in jumps {
            command = format!("{} 0x{:X}", command, jump);
        }
        self.send(command, false, false, 0)
    }

    pub fn freeze(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("freeze 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false, 0)
    }

    pub fn unfreeze(&self, addr: u64) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("unFreeze 0x{:X}", addr);
        self.send(command, false, false, 0)
    }

    pub fn freeze_clear(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "freezeClear".to_string();
        self.send(command, false, false, 0)
    }

    pub fn freeze_pause(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "freezePause".to_string();
        self.send(command, false, false, 0)
    }

    pub fn freeze_unpause(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "freezeUnpause".to_string();
        self.send(command, false, false, 0)
    }
}

impl Drop for SysBotClient {
    fn drop(&mut self) {
        self.send("".to_string(), false, true, 0)
            .expect("Failed to send closing message");
        self.worker.take().unwrap().join().unwrap();
    }
}
