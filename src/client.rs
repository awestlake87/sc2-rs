use std::{io, time};

use bytes::{Buf, BufMut};
use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use protobuf::{self, parse_from_reader, Message};
use sc2_proto::sc2api::{Request, Response};
use tokio_core::reactor;
use tokio_timer::Timer;
use tokio_tungstenite::connect_async;
use tungstenite;
use url::Url;

use super::{Error, Result};
use synapses::{Dendrite, Synapse};

fn stream_next<T: Stream>(
    stream: T,
) -> impl Future<Item = (Option<T::Item>, T), Error = (T::Error, T)> {
    stream
        .and_then(|item| Ok(item))
        .into_future()
        .map(|(item, and_then)| (item, and_then.into_inner()))
        .map_err(|(e, and_then)| (e, and_then.into_inner()))
}

#[derive(Debug)]
enum ProtoRequest {
    Connect(Url, oneshot::Sender<()>),
    Request(Request, oneshot::Sender<Result<Response>>),
}

pub struct ProtoClientBuilder {
    tx: mpsc::Sender<ProtoRequest>,
    rx: mpsc::Receiver<ProtoRequest>,
}

impl ProtoClientBuilder {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(5);

        Self { tx: tx, rx: rx }
    }

    pub fn fork(&self) -> ProtoClient {
        ProtoClient {
            tx: self.tx.clone(),
        }
    }

    pub fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        let proto_handle = handle.clone();

        handle.spawn(listen(self.rx, proto_handle).map(|_| ()).map_err(|_| ()));

        Ok(())
    }
}

/// sender for the client soma
#[derive(Debug, Clone)]
pub struct ProtoClient {
    tx: mpsc::Sender<ProtoRequest>,
}

impl ProtoClient {
    /// connect to the game instance
    pub fn connect(&self, url: Url) -> impl Future<Item = (), Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ProtoRequest::Connect(url, tx))
                    .map_err(|_| Error::from("unable to send connect"))
            )?;

            await!(rx.map_err(|_| Error::from("unable to receive connect ack")))
        }
    }

    /// send a request to the game instance
    pub fn request(
        &self,
        req: Request,
    ) -> impl Future<Item = Response, Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ProtoRequest::Request(req, tx))
                    .map_err(|_| Error::from("unable to send request"))
            )?;

            await!(rx.map_err(|_| Error::from("unable to receive response")))?
        }
    }
}

/// receiver for the client soma
#[derive(Debug)]
pub struct ClientDendrite {
    rx: mpsc::Receiver<ProtoRequest>,
}

/// create a client synapse
pub fn synapse() -> (ProtoClient, ClientDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (ProtoClient { tx: tx }, ClientDendrite { rx: rx })
}

#[async]
fn listen(
    mut rx: mpsc::Receiver<ProtoRequest>,
    handle: reactor::Handle,
) -> Result<()> {
    loop {
        let (req, stream) = await!(
            stream_next(rx)
                .map_err(|_| Error::from("unable to receive next item"))
        )?;

        rx = match req {
            Some(ProtoRequest::Connect(url, tx)) => {
                await!(connect(url, handle.clone(), tx, stream))?
            },
            None => break,
            _ => bail!("unexpected req {:?}", req),
        }
    }

    Ok(())
}

#[async]
fn connect(
    url: Url,
    handle: reactor::Handle,
    tx: oneshot::Sender<()>,
    rx: mpsc::Receiver<ProtoRequest>,
) -> Result<mpsc::Receiver<ProtoRequest>> {
    const NUM_RETRIES: u32 = 10;

    let timer = Timer::default();

    let mut client = None;

    for i in 0..NUM_RETRIES {
        let url = url.clone();
        let handle = handle.clone();

        println!(
            "attempting to connect to instance {} - retries {}",
            url,
            (NUM_RETRIES - 1) - i
        );

        match await!(
            timer
                .sleep(time::Duration::from_secs(5))
                .map_err(|e| e.into())
                .and_then(|_| attempt_connect(url, handle))
        ) {
            Err(_) => println!("connect failed. retrying in 5s"),
            Ok((send, recv)) => {
                client = Some((send, recv));
                break;
            },
        }
    }

    if let Some((send, recv)) = client {
        tx.send(())
            .map_err(|_| Error::from("unable to send connect ack"))?;
        await!(run_client(rx, send, recv))
    } else {
        bail!("unable to connect")
    }
}

#[async]
fn attempt_connect(
    url: Url,
    handle: reactor::Handle,
) -> Result<
    (
        mpsc::Sender<tungstenite::Message>,
        mpsc::Sender<oneshot::Sender<Result<Response>>>,
    ),
> {
    let (send_tx, send_rx) = mpsc::channel(10);
    let (recv_tx, recv_rx) =
        mpsc::channel::<oneshot::Sender<Result<Response>>>(10);

    let handle = handle.clone();

    await!(connect_async(url, handle.remote().clone()).and_then(
        move |(ws_stream, _)| {
            let (sink, stream) = ws_stream.split();

            handle.spawn(
                sink.send_all(
                    send_rx
                        .map_err(|_| -> tungstenite::Error { unreachable!() }),
                ).map(|_| ())
                    .map_err(|_| ()),
            );
            handle.spawn(
                stream
                    .map_err(|e| panic!("client error {:?}", e))
                    .zip(recv_rx)
                    .for_each(|(msg, tx)| {
                        if let tungstenite::Message::Binary(buf) = msg {
                            let cursor = io::Cursor::new(buf);

                            match parse_from_reader::<Response>(&mut cursor.reader())
                            {
                                Err(e) => {
                                    if let Err(_) = tx.send(Err(e.into())) {
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
    )).map_err(|e| Error::from(e))?;

    Ok((send_tx, recv_tx))
}

#[async]
fn run_client(
    mut rx: mpsc::Receiver<ProtoRequest>,
    send: mpsc::Sender<tungstenite::Message>,
    recv: mpsc::Sender<oneshot::Sender<Result<Response>>>,
) -> Result<mpsc::Receiver<ProtoRequest>> {
    loop {
        let (req, stream) = await!(
            stream_next(rx)
                .map_err(|_| Error::from("unable to receive next item"))
        )?;

        rx = stream;

        match req {
            Some(ProtoRequest::Request(req, tx)) => {
                let buf = vec![];
                let mut writer = buf.writer();

                {
                    let mut cos = protobuf::CodedOutputStream::new(&mut writer);

                    req.write_to(&mut cos)?;
                    cos.flush()?;
                }
                await!(
                    send.clone()
                        .send(tungstenite::Message::Binary(writer.into_inner()))
                        .map_err(|_| Error::from("unable to send request"))
                )?;
                await!(
                    recv.clone()
                        .send(tx)
                        .map_err(|_| Error::from("unable to send responder"))
                )?;
            },

            None => break,

            _ => bail!("unexpected request"),
        }
    }

    Ok(rx)
}
