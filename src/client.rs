use crate::types::thread_message::ThreadMessage;
use crate::types::{Button, PeekArgs, PokeArgs, SeqParam, Stick, StickMovement};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread;
use std::thread::JoinHandle;

pub struct SysBotClient {
    sender: SyncSender<ThreadMessage>,
    receiver: Receiver<Vec<u8>>,
    worker: Option<JoinHandle<()>>,
}

impl SysBotClient {
    pub fn connect(addr: &str, port: u16) -> Result<Self, &'static str> {
        let socket_addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::from_str(addr).map_err(|_| "Failed to convert ip address")?),
            port,
        );
        let (sender_in, receiver_in): (SyncSender<ThreadMessage>, Receiver<ThreadMessage>) =
            mpsc::sync_channel(0);
        let (sender_out, receiver_out): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let worker = Some(thread::spawn(move || {
            let mut tcp_stream =
                TcpStream::connect(socket_addr).expect("Failed to connect to address");
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
                        let mut buf = [0u8; 10000];
                        let size = tcp_stream
                            .read(&mut buf)
                            .expect("Failed to read from stream");
                        sender_out
                            .clone()
                            .send((&buf[..size]).to_vec())
                            .expect("Failed to send response over channel");
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

    pub fn consume(self) {
        self.send("".to_string(), false, true)
            .expect("Failed to send closing message");
        let worker = self.worker.expect("Worker was never created");
        worker.join().expect("Failed to join worker");
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

    fn send(&self, command: String, returns: bool, close: bool) -> Result<(), &'static str> {
        self.sender
            .send(ThreadMessage {
                message: command + "\r\n",
                returns,
                close,
            })
            .map_err(|_| "Failed to send command")
    }

    pub fn peek(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peek 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn peek_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let args = args
            .into_iter()
            .map(|a| format!("0x{:X} 0x{:X}", a.addr, a.size))
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekMulti {}", args);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn peek_absolute(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peekAbsolute 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn peek_absolute_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let args = args
            .into_iter()
            .map(|a| format!("0x{:X} 0x{:X}", a.addr, a.size))
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekAbsoluteMulti {}", args);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn peek_main(&self, args: PeekArgs) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let command = format!("peekMain 0x{:X} 0x{:X}", args.addr, args.size);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn peek_main_multi(&self, args: Vec<PeekArgs>) -> Result<Vec<u8>, &'static str> {
        self.check_connected()?;
        let args = args
            .into_iter()
            .map(|a| format!("0x{:X} 0x{:X}", a.addr, a.size))
            .collect::<Vec<String>>()
            .join(" ");
        let command = format!("peekMainMulti {}", args);
        self.send(command, true, false)?;
        self.receive()
    }

    pub fn poke(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("poke 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn poke_absolute(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("pokeAbsolute 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn poke_main(&self, args: PokeArgs) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("pokeMain 0x{:X} {}", args.addr, args.data.to_string());
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn click(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("click {}", button);
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn click_seq(&self, args: Vec<SeqParam>) -> Result<(), &'static str> {
        self.check_connected()?;
        let args = args
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let command = format!("clickSeq {}", args);
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn click_cancel(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "clickCancel".to_string();
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn press(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("press {}", button);
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn release(&self, button: Button) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!("release {}", button);
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn set_stick(&self, stick: Stick, movement: StickMovement) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = format!(
            "setStick {} {}",
            stick,
            movement.to_string().replace(',', " ")
        );
        self.send(command, false, false)?;
        Ok(())
    }

    pub fn detach_controller(&self) -> Result<(), &'static str> {
        self.check_connected()?;
        let command = "detachController".to_string();
        self.send(command, false, false)?;
        Ok(())
    }
}
