
use std::io;
use std::mem;
use std::time;

use bytes::{ Buf, BufMut };
use futures::prelude::*;
use futures::future::{ Either };
use futures::stream::{ StreamFuture };
use futures::sync::{ mpsc };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::common;
use sc2_proto::sc2api;
use sc2_proto::sc2api::{ Request, Response };
use tokio_core::{ reactor };
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;

use super::{ Result, Error };
use super::game::{ GameSettings, Map };
use super::player::{ Player, PlayerKind, Race, Difficulty };

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

    #[async]
    pub fn send(mut self, req: Request) -> Result<Self> {
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

        match await!(
            sender.send(
                tungstenite::Message::Binary(writer.into_inner())
            )
        ) {
            Ok(sender) =>  {
                mem::replace(&mut self.sender, Some(sender));
                Ok(self)
            },
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }

    #[async(boxed)]
    pub fn recv(mut self) -> Result<(Response, Self)> {
        let receiver = match mem::replace(&mut self.receiver, None) {
            Some(receiver) => receiver,
            None => return Err(Error::WebsockRecvFailed)
        };

        let timer = Timer::default();

        let (result, stream_future) = match await!(
            receiver.select2(
                timer.sleep(time::Duration::from_millis(3000))
            )
        ) {
            Ok(Either::A((result, _))) => match result {
                (Some(tungstenite::Message::Binary(buf)), receiver) => {
                    let cursor = io::Cursor::new(buf);

                    match parse_from_reader::<Response>(
                            &mut cursor.reader()
                        )
                    {
                        Ok(rsp) => (Ok(rsp), Some(receiver.into_future())),
                        Err(e) => {
                            eprintln!(
                                "unable to parse response: {}", e
                            );

                            (
                                Err(Error::WebsockRecvFailed),
                                Some(receiver.into_future())
                            )
                        }
                    }
                }
                (Some(_), receiver) => {
                    eprintln!(
                        "unexpected non-binary message, retrying recv"
                    );
                    (
                        Err(Error::WebsockRecvFailed),
                        Some(receiver.into_future())
                    )
                }
                (None, receiver) => {
                    eprintln!("nothing received");
                    (
                        Err(Error::WebsockRecvFailed),
                        Some(receiver.into_future())
                    )
                }
            },
            Ok(Either::B((result, stream_future))) => {
                match result {
                    _ => println!("receive timed out")
                };

                (Err(Error::WebsockRecvFailed), Some(stream_future))
            }
            Err(_) => {
                eprintln!("unable to receive message");
                (Err(Error::WebsockRecvFailed), None)
            }
        };
        mem::replace(&mut self.receiver, stream_future);

        match result {
            Ok(rsp) => Ok((rsp, self)),
            Err(e) => Err(e)
        }
    }

    #[async]
    pub fn call(mut self, req: Request) -> Result<(Response, Self)> {
        match await!(self.send(req)) {
            Ok(client) => self = client,
            Err(e) => return Err(e)
        };

        await!(self.recv())
    }

    #[async]
    pub fn quit(self) -> Result<Self> {
        let mut req = sc2api::Request::new();

        req.mut_quit();

        await!(self.send(req))
    }

    #[async]
    pub fn create_game(
        self, settings: GameSettings, players: Vec<Player>
    )
        -> Result<(Response, Self)>
    {
        let mut req = sc2api::Request::new();

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

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player.kind {
                PlayerKind::Computer => {
                    let difficulty = match player.difficulty {
                        Some(difficulty) => difficulty,
                        None => return Err(
                            Error::Todo("computer must have difficulty")
                        )
                    };

                    use sc2_proto::sc2api::Difficulty as Diff;

                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(
                        match difficulty {
                            Difficulty::VeryEasy        => Diff::VeryEasy,
                            Difficulty::Easy            => Diff::Easy,
                            Difficulty::Medium          => Diff::Medium,
                            Difficulty::MediumHard      => Diff::MediumHard,
                            Difficulty::Hard            => Diff::Hard,
                            Difficulty::Harder          => Diff::Harder,
                            Difficulty::VeryHard        => Diff::VeryHard,
                            Difficulty::CheatVision     => Diff::CheatVision,
                            Difficulty::CheatMoney      => Diff::CheatMoney,
                            Difficulty::CheatInsane     => Diff::CheatInsane
                        }
                    );
                },
                PlayerKind::Participant => {
                    setup.set_field_type(sc2api::PlayerType::Participant);
                },
                PlayerKind::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            match player.race {
                Some(race) => setup.set_race(
                    match race {
                        Race::Zerg      => common::Race::Zerg,
                        Race::Terran    => common::Race::Terran,
                        Race::Protoss   => common::Race::Protoss
                    }
                ),
                None => ()
            };

            req.mut_create_game().mut_player_setup().push(setup);
        }

        match await!(self.call(req)) {
            Ok((rsp, client)) => {
                println!("parsed rsp {:#?}", rsp);

                Ok((rsp, client))
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
