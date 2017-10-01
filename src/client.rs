
use std::io;

use bytes::{ Buf, BufMut };
use futures::prelude::*;
use futures::sync::{ mpsc };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::sc2api::{ Request, Response };
use tokio_core::{ reactor };
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;

use super::{ Result, Error };
use game::{ GameSettings, Map };
use player::{ Player };

pub struct Client {
    reactor:        reactor::Handle,
    sender:         mpsc::Sender<tungstenite::Message>,
    receiver:       mpsc::Receiver<tungstenite::Message>,
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

                let client_reactor = reactor.clone();

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
                        reactor: client_reactor,
                        sender: req_tx,
                        receiver: rsp_rx
                    }
                )
            },
            Err(e) => {
                println!("websocket handshaked failed: {}", e);

                Err(Error::WebsockOpenFailed)
            }
        }
    }

    #[async]
    pub fn send(self, req: Request) -> Result<Self> {
        let buf = Vec::new();
        let mut writer = buf.writer();

        {
            let mut cos = CodedOutputStream::new(&mut writer);

            req.write_to(&mut cos).unwrap();
            cos.flush().unwrap();
        }

        let Self { reactor, sender, receiver } = self;

        match
            await!(
                sender.send(
                    tungstenite::Message::Binary(writer.into_inner())
                )
            )
        {
            Ok(sender) => Ok(
                Self {
                    reactor: reactor,
                    sender: sender,
                    receiver: receiver
                }
            ),
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }

    #[async]
    pub fn recv(self) -> Result<(Response, Self)> {
        let Self { reactor, sender, receiver } = self;

        match await!(receiver.into_future()) {
            Ok((Some(tungstenite::Message::Binary(buf)), receiver)) => {
                let cursor = io::Cursor::new(buf);

                match
                    parse_from_reader::<Response>(
                        &mut cursor.reader()
                    )
                {
                    Ok(rsp) => Ok((
                        rsp,
                        Self {
                            sender: sender,
                            reactor: reactor,
                            receiver: receiver
                        }
                    )),
                    Err(e) => {
                        eprintln!(
                            "unable to parse response: {}", e
                        );

                        Err(Error::WebsockRecvFailed)
                    }
                }
            }
            Ok((Some(_), _)) => {
                eprintln!("unexpected non-binary message, retrying recv");
                Err(Error::WebsockRecvFailed)
            }
            Ok((None, _)) => {
                eprintln!("nothing received");
                Err(Error::WebsockRecvFailed)
            }
            Err(_) => {
                eprintln!("unable to receive message");
                Err(Error::WebsockRecvFailed)
            }
        }
    }

    #[async]
    pub fn call(self, req: Request) -> Result<(Response, Self)>
    {
        match await!(self.send(req)) {
            Ok(client) => await!(client.recv()),
            Err(e) => Err(e)
        }
    }

    #[async]
    pub fn quit(self) -> Result<Self> {
        let mut req = Request::new();

        req.mut_quit();

        await!(self.send(req))
    }

    #[async]
    pub fn create_game(self, settings: GameSettings, players: &Vec<Player>)
        -> Result<Self>
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

        match await!(self.call(req)) {
            Ok((rsp, client)) => {
                println!("parsed rsp {:#?}", rsp);

                await!(client.quit())
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
