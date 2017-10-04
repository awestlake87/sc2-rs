
pub mod control;

use std::io;

use bytes::{ Buf, BufMut };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::sc2api::{ Request, Response };
use tungstenite::{ connect, Message as WebSocketMessage, WebSocket };
use tungstenite::client::{ AutoStream };
use url::Url;

use super::{ Result, Error };

pub struct Client {
    socket: WebSocket<AutoStream>
}

impl Client {
    pub fn connect(url: Url) -> Result<Self> {
        match connect(url) {
            Ok((socket, _)) => Ok(Self { socket: socket }),
            Err(e) => {
                eprintln!("open failed: {}", e);
                Err(Error::WebsockOpenFailed)
            }
        }
    }

    pub fn send(&mut self, req: Request) -> Result<()> {
        let buf = Vec::new();
        let mut writer = buf.writer();

        {
            let mut cos = CodedOutputStream::new(&mut writer);

            req.write_to(&mut cos).unwrap();
            cos.flush().unwrap();
        }

        match self.socket.write_message(
            WebSocketMessage::Binary(writer.into_inner())
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("send failed: {}", e);
                Err(Error::WebsockSendFailed)
            }
        }
    }

    pub fn close(&mut self) -> Result<()> {
        match self.socket.close(None) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("close failed: {}", e);
                Err(Error::Todo("close failed"))
            }
        }
    }

    pub fn recv(&mut self) -> Result<Response> {
        match self.socket.read_message() {
            Ok(WebSocketMessage::Binary(buf)) => {
                let cursor = io::Cursor::new(buf);

                match parse_from_reader::<Response>(&mut cursor.reader()) {
                    Ok(rsp) => Ok(rsp),
                    Err(e) => {
                        eprintln!("unable to parse response: {}", e);

                        Err(Error::WebsockRecvFailed)
                    }
                }
            }
            Ok(_) => {
                eprintln!("unexpected non-binary message");
                Err(Error::WebsockRecvFailed)
            }
            Err(e) => {
                eprintln!("recv failed: {}", e);
                Err(Error::WebsockRecvFailed)
            }
        }
    }

    pub fn call(&mut self, req: Request) -> Result<Response> {
        self.send(req)?;
        self.recv()
    }
}
