
use std::io;
use std::mem;
use std::time;

use bytes::{ Buf, BufMut };
use futures::prelude::*;
use futures::future::{ Either };
use futures::stream::{ StreamFuture };
use futures::sync::{ mpsc };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::sc2api::{ Request, Response };
use tokio_core::{ reactor };
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;

use super::{ Result, Error };
use game::{ GameSettings, Map };
use player::{ Player };

pub struct Client {
    sender:         Option<mpsc::Sender<tungstenite::Message>>,
    receiver:       Option<StreamFuture<mpsc::Receiver<tungstenite::Message>>>,
}

impl Client {
    #[async]
    pub fn connect(reactor: reactor::Handle, url: Url) -> Result<Self> {
        match await!(connect_async(url.clone(), reactor.remote().clone())) {
            Ok((ws_stream, _)) => {
                println!("websocket connected");

                let (sink, stream) = ws_stream.split();
                let (req_tx, req_rx) = mpsc::channel(1);
                let (rsp_tx, rsp_rx) = mpsc::channel(1);

                // forward the requests to the websocket's sink
                reactor.spawn(
                    req_rx.map_err(
                        |_| tungstenite::Error::Io(
                            io::Error::new(
                                io::ErrorKind::Other,
                                "websocket failed to queue message"
                            )
                        )
                    ).forward(sink).then(
                        |_| Ok(())
                    )
                );

                // create a task to channel websocket messages to client
                reactor.clone().spawn(
                    stream.for_each(
                        move |msg| {
                            reactor.spawn(
                                rsp_tx.clone().send(msg).then(
                                    |_| Ok(())
                                )
                            );
                            Ok(())
                        }
                    ).then(|_| Ok(()))
                );

                Ok(
                    Client {
                        sender: Some(req_tx),
                        receiver: Some(rsp_rx.into_future())
                    }
                )
            },
            Err(e) => {
                println!("websocket handshaked failed: {}", e);

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

        let sender = match mem::replace(&mut self.sender, None) {
            Some(sender) => sender,
            None => return Err(Error::WebsockSendFailed)
        };

        let send_op = async_block! {
            match await!(
                sender.send(
                    tungstenite::Message::Binary(writer.into_inner())
                )
            ) {
                Ok(sender) =>  Ok(sender),
                Err(_) => Err(Error::WebsockSendFailed)
            }
        };

        match send_op.wait() {
            Ok(sender) => {
                mem::replace(&mut self.sender, Some(sender));
                Ok(())
            },
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }

    pub fn recv(&mut self) -> Result<Response> {
        let receiver = match mem::replace(&mut self.receiver, None) {
            Some(receiver) => receiver,
            None => return Err(Error::WebsockRecvFailed)
        };

        let recv_op = async_block! {
            let timer = Timer::default();

            match await!(
                receiver.select2(
                    timer.sleep(time::Duration::from_millis(3000))
                )
            ) {
                Ok(Either::A((result, _))) => match result {
                    (Some(tungstenite::Message::Binary(buf)), receiver) => {
                        let cursor = io::Cursor::new(buf);

                        match
                            parse_from_reader::<Response>(
                                &mut cursor.reader()
                            )
                        {
                            Ok(rsp) => Ok((rsp, Some(receiver.into_future()))),
                            Err(e) => {
                                eprintln!(
                                    "unable to parse response: {}", e
                                );

                                Err((
                                    Error::WebsockRecvFailed,
                                    Some(receiver.into_future())
                                ))
                            }
                        }
                    }
                    (Some(_), receiver) => {
                        eprintln!(
                            "unexpected non-binary message, retrying recv"
                        );
                        Err((
                            Error::WebsockRecvFailed,
                            Some(receiver.into_future())
                        ))
                    }
                    (None, receiver) => {
                        eprintln!("nothing received");
                        Err((
                            Error::WebsockRecvFailed,
                            Some(receiver.into_future())
                        ))
                    }
                },
                Ok(Either::B((result, stream_future))) => {
                    match result {
                        _ => println!("receive timed out")
                    };

                    Err((Error::WebsockRecvFailed, Some(stream_future)))
                }
                Err(_) => {
                    eprintln!("unable to receive message");
                    Err((Error::WebsockRecvFailed, None))
                }
            }
        };

        match recv_op.wait() {
            Ok((rsp, stream_future)) => {
                mem::replace(&mut self.receiver, stream_future);
                Ok(rsp)
            }
            Err((e, stream_future)) => {
                mem::replace(&mut self.receiver, stream_future);
                Err(e)
            }
        }
    }

    pub fn call(&mut self, req: Request) -> Result<Response> {
        self.send(req)?;
        self.recv()
    }

    pub fn quit(&mut self) -> Result<()> {
        let mut req = Request::new();

        req.mut_quit();

        self.send(req)
    }

    pub fn create_game(
        &mut self, settings: GameSettings, players: &Vec<Player>
    )
        -> Result<()>
    {
        let mut req = Request::new();

        match settings.map {
            Map::LocalMap(path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => return Err(
                            Error::Todo("invalid path string")
                        )
                    }
                );
            }
        };

        match self.call(req) {
            Ok(rsp) => {
                println!("parsed rsp {:#?}", rsp);

                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
