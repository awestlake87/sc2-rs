
use std::io;
use std::time;

use bytes::{ Buf, BufMut };
use futures::prelude::*;
use futures::sync::{ mpsc, oneshot };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::sc2api::{ Request, Response };
use tokio_core::{ reactor };
use tokio_timer::Timer;
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;

use super::{ Result, Error };

pub struct Client {
    reactor:        reactor::Handle,
    sender:         mpsc::Sender<tungstenite::Message>,
    receiver:       mpsc::Receiver<tungstenite::Message>,
}

impl Client {
    pub fn connect(reactor: reactor::Handle, url: Url)
        -> oneshot::Receiver<Self>
    {
        let (tx, rx) = oneshot::channel::<Self>();

        attempt_connect(reactor, url, tx);

        rx
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
    pub fn create_game(self) -> Result<Self> {
        let mut req = Request::new();

        req.mut_create_game().mut_local_map().set_map_path(
            "/home/najen/StarCraftII/Maps/AbyssalReefLE.SC2Map".to_string()
        );

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

fn attempt_connect(
    reactor: reactor::Handle, url: Url, tx: oneshot::Sender<Client>
) {
    reactor.clone().spawn(
        connect_async(url.clone(), reactor.remote().clone()).then(
            move |result| {
                match result {
                    Ok((ws_stream, _)) => {
                        println!("websocket handshake completed successfully");

                        let (sink, stream) = ws_stream.split();
                        let (req_tx, req_rx) = mpsc::channel(1);
                        let (rsp_tx, rsp_rx) = mpsc::channel(1);

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

                        tx.send(
                            Client {
                                reactor: client_reactor,
                                sender: req_tx,
                                receiver: rsp_rx
                            }
                        )
                    }
                    Err(e) => {
                        println!("websocket handshaked failed: {}", e);
                        println!("retrying...");

                        let timer = Timer::default();

                        reactor.clone().spawn(
                            timer.sleep(
                                time::Duration::from_millis(1000)
                            ).then(
                                move |_| {
                                    attempt_connect(reactor, url, tx);

                                    Ok(())
                                }
                            )
                        );

                        Ok(())
                    }
                }
            }
        ).then(|_| Ok(()))
    );
}
