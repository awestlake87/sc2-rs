use std::{io, mem, time};

use bytes::{Buf, BufMut};
use futures::prelude::*;
use futures::prelude::await;
use futures::unsync::{mpsc, oneshot};
use protobuf::{self, parse_from_reader, Message};
use sc2_proto::sc2api::{Request, Response};
use tokio_core::reactor;
use tokio_timer::Timer;
use tokio_tungstenite::connect_async;
use tungstenite;
use url::Url;

use constants::{info_tag, sc2_bug_tag};
use {Error, ErrorKind, Result};

#[derive(Debug)]
enum ClientRequest {
    Connect(Url, oneshot::Sender<()>),
    Request(Request, oneshot::Sender<Result<Response>>),
    Disconnect(oneshot::Sender<()>),
}

/// Sender for the client.
#[derive(Debug, Clone)]
pub struct ProtoClient {
    tx: mpsc::Sender<ClientRequest>,
}

impl ProtoClient {
    /// Connect to the game instance.
    pub fn connect(&self, url: Url) -> impl Future<Item = (), Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ClientRequest::Connect(url, tx))
                    .map_err(|_| -> Error {
                        unreachable!("{}: Connect req failed", sc2_bug_tag())
                    })
            )?;

            await!(rx.map_err(|_| -> Error {
                unreachable!("{}: Connect ack failed", sc2_bug_tag())
            }))
        }
    }

    /// Send a request to the game instance.
    pub fn request(
        &self,
        req: Request,
    ) -> impl Future<Item = Response, Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ClientRequest::Request(req, tx))
                    .map_err(|_| -> Error {
                        unreachable!("{}: Request req failed", sc2_bug_tag())
                    })
            )?;

            await!(rx.map_err(|_| -> Error {
                unreachable!("{}: Request ack failed", sc2_bug_tag())
            }))?
        }
    }

    pub fn disconnect(&self) -> impl Future<Item = (), Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ClientRequest::Disconnect(tx))
                    .map_err(|_| -> Error {
                        unreachable!("{}: Disconnect req failed", sc2_bug_tag())
                    })
            )?;

            await!(rx.map_err(|_| -> Error {
                unreachable!("{}: Disconnect ack failed", sc2_bug_tag())
            }))
        }
    }
}

/// Websocket client used to communicate with the game instance.
pub struct ProtoClientBuilder {
    tx: mpsc::Sender<ClientRequest>,
    rx: mpsc::Receiver<ClientRequest>,
}

impl ProtoClientBuilder {
    /// Create a new client.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10);

        Self { tx: tx, rx: rx }
    }

    pub fn add_client(&self) -> ProtoClient {
        ProtoClient {
            tx: self.tx.clone(),
        }
    }

    pub fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        ClientService::new(self.rx, handle.clone()).spawn(handle)
    }
}

pub struct ClientService {
    rx: Option<mpsc::Receiver<ClientRequest>>,
    handle: reactor::Handle,
}

impl ClientService {
    fn new(rx: mpsc::Receiver<ClientRequest>, handle: reactor::Handle) -> Self {
        Self {
            rx: Some(rx),
            handle: handle,
        }
    }

    fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        handle.spawn(self.run().map_err(|e| {
            panic!(
                "{}: Client exited unexpectedly - {:#?}",
                sc2_bug_tag(),
                e
            )
        }));

        Ok(())
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let rx = mem::replace(&mut self.rx, None).unwrap();
        let mut connection = None;

        #[async]
        for req in rx.map_err(|_| -> Error { unreachable!() }) {
            match req {
                ClientRequest::Connect(url, tx) => {
                    let (client, conn) = await!(self.connect(url))?;
                    self = client;

                    connection = Some(conn);

                    tx.send(()).unwrap();
                },
                ClientRequest::Request(req, tx) => match connection {
                    Some(conn) => {
                        await!(conn.request(req, tx))?;
                        connection = Some(conn)
                    },
                    None => tx.send(Err(ErrorKind::ClientSendFailed(
                        "No connection to game".to_string(),
                    ).into()))
                        .unwrap(),
                },
                ClientRequest::Disconnect(tx) => {
                    connection = None;
                    tx.send(()).unwrap()
                },
            }
        }

        Ok(())
    }

    #[async]
    fn connect(self, url: Url) -> Result<(Self, Connection)> {
        const NUM_RETRIES: u32 = 10;

        let timer = Timer::default();

        for i in 0..NUM_RETRIES {
            println!(
                "{}: Attempting to connect to instance {} - retries {}",
                info_tag(),
                url,
                (NUM_RETRIES - 1) - i
            );

            let connect_future = self.attempt_connect(&url);

            match await!(
                timer
                    .sleep(time::Duration::from_secs(5))
                    .map_err(|e| e.into())
                    .and_then(|_| connect_future)
            ) {
                Ok(conn) => {
                    return Ok((self, conn));
                },
                Err(e) => {
                    // if no retries left
                    if NUM_RETRIES - i == 1 {
                        return Err(e);
                    } else {
                        println!(
                            "{}: Unable to connect, retrying...",
                            info_tag()
                        );
                        continue;
                    }
                },
            };
        }

        unreachable!();
    }

    fn attempt_connect(
        &self,
        url: &Url,
    ) -> impl Future<Item = Connection, Error = Error> {
        let (send_tx, send_rx) = mpsc::channel(10);
        let (recv_tx, recv_rx) =
            mpsc::channel::<oneshot::Sender<Result<Response>>>(10);

        let handle = self.handle.clone();
        let url = url.clone();

        async_block! {
            await!(
                connect_async(url, handle.remote().clone()).and_then(
                    move |(ws_stream, _)| {
                        let (sink, stream) = ws_stream.split();

                        handle.spawn(
                            sink.send_all(send_rx.map_err(
                                |_| -> tungstenite::Error { unreachable!() },
                            )).map(|_| ())
                                .map_err(|_| ()),
                        );
                        handle.spawn(
                            stream
                                .map_err(|_| -> () { unreachable!() })
                                .zip(recv_rx)
                                .for_each(|(msg, tx)| {
                                    if let tungstenite::Message::Binary(buf) = msg {
                                        let cursor = io::Cursor::new(buf);

                                        match parse_from_reader::<Response>(
                                            &mut cursor.reader(),
                                        ) {
                                            Err(e) => {
                                                if let Err(_) =
                                                    tx.send(Err(e.into()))
                                                {
                                                    // keep going
                                                }
                                            },
                                            Ok(rsp) => {
                                                if let Err(_) = tx.send(Ok(rsp)) {
                                                    // keep going
                                                }
                                            },
                                        }
                                    }

                                    Ok(())
                                }),
                        );

                        Ok(())
                    }
                ).map_err(|e| Error::with_chain(e, ErrorKind::ClientOpenFailed(
                    "Unable to connect".to_string()
                )))
            )?;

            Ok(
                Connection {
                    send: send_tx,
                    recv: recv_tx,
                },
            )
        }
    }
}

struct Connection {
    send: mpsc::Sender<tungstenite::Message>,
    recv: mpsc::Sender<oneshot::Sender<Result<Response>>>,
}

impl Connection {
    fn request(
        &self,
        req: Request,
        tx: oneshot::Sender<Result<Response>>,
    ) -> impl Future<Item = (), Error = Error> {
        let sender = self.send.clone();
        let receiver = self.recv.clone();

        async_block!{
            let buf = vec![];
            let mut writer = buf.writer();

            {
                let mut cos = protobuf::CodedOutputStream::new(&mut writer);

                req.write_to(&mut cos)?;
                cos.flush()?;
            }

            let msg = tungstenite::Message::Binary(writer.into_inner());

            await!(
                sender.send(msg)
                    .map_err(|_| Error::from(ErrorKind::ClientSendFailed(
                        "Send sink has closed".to_string()
                    )))
            )?;

            await!(
                receiver.send(tx)
                    .map_err(|_| Error::from(ErrorKind::ClientRecvFailed(
                        "Recv stream has closed".to_string()
                    )))
            )?;

            Ok(())
        }
    }
}
