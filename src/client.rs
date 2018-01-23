use std::{self, mem, time};

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::sc2api::{Request, Response};
use tokio_core::reactor;
use tokio_timer::Timer;
use tokio_tungstenite::connect_async;
use url::Url;

use super::{Dendrite, Error, Result, Synapse};

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
enum ClientRequest {
    Connect(Url, oneshot::Sender<()>),
}

#[derive(Debug, Clone)]
pub struct ClientTerminal {
    tx: mpsc::Sender<ClientRequest>,
}

impl ClientTerminal {
    #[async]
    pub fn connect(self, url: Url) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ClientRequest::Connect(url, tx))
                .map_err(|_| Error::from("unable to send client request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to receive connect ack")))
    }
}

#[derive(Debug)]
pub struct ClientDendrite {
    rx: mpsc::Receiver<ClientRequest>,
}

impl ClientDendrite {
    #[async]
    fn listen(self, handle: reactor::Handle) -> Result<()> {
        let mut rx = self.rx;

        loop {
            let (req, stream) = await!(
                stream_next(rx)
                    .map_err(|_| Error::from("unable to receive next item"))
            )?;

            rx = match req {
                Some(ClientRequest::Connect(url, tx)) => {
                    await!(run_client(url, handle.clone(), tx, stream))?
                },
                None => break,
                _ => bail!("unexpected req {:?}", req),
            }
        }

        Ok(())
    }
}

pub struct ClientSoma {
    dendrite: Option<ClientDendrite>,
}

impl ClientSoma {
    pub fn synapse() -> (ClientTerminal, ClientDendrite) {
        let (tx, rx) = mpsc::channel(10);

        (ClientTerminal { tx: tx }, ClientDendrite { rx: rx })
    }

    pub fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self { dendrite: None },
            vec![Constraint::One(Synapse::Client)],
            vec![],
        ))
    }
}

impl Soma for ClientSoma {
    type Synapse = Synapse;
    type Error = Error;
    type Future = Box<Future<Item = Self, Error = Self::Error>>;

    #[async(boxed)]
    fn update(mut self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(Synapse::Client, Dendrite::Client(rx)) => {
                self.dendrite = Some(rx);

                Ok(self)
            },
            Impulse::Start(tx, handle) => {
                assert!(self.dendrite.is_some());

                let listen_handle = handle.clone();

                handle.spawn(
                    mem::replace(&mut self.dendrite, None)
                        .unwrap()
                        .listen(listen_handle)
                        .or_else(move |e| {
                            tx.send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(self)
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

#[async]
fn run_client(
    url: Url,
    handle: reactor::Handle,
    tx: oneshot::Sender<()>,
    rx: mpsc::Receiver<ClientRequest>,
) -> Result<mpsc::Receiver<ClientRequest>> {
    const NUM_RETRIES: u32 = 10;

    let timer = Timer::default();

    for i in 0..NUM_RETRIES {
        let url = url.clone();
        let handle = handle.clone();

        println!(
            "attempting to connect to instance {} - retries {}",
            url,
            (NUM_RETRIES - 1) - i
        );

        if let Err(_) = await!(
            timer
                .sleep(time::Duration::from_secs(5))
                .map_err(|e| e.into())
                .and_then(|_| attempt_connect(url, handle))
        ) {
            println!("connect failed. retrying in 5s");
        } else {
            tx.send(()).map_err(|_| Error::from("unable to send"))?;
            break;
        }
    }

    Ok(rx)
}

#[async]
fn attempt_connect(url: Url, handle: reactor::Handle) -> Result<()> {
    await!(connect_async(url, handle.remote().clone()).and_then(
        move |(ws_stream, _)| {
            let (sink, stream) = ws_stream.split();

            Ok(())
        }
    )).map_err(|e| e.into())
}

// #[derive(PartialEq, Copy, Clone, Debug)]
// enum ClientMessageKind {
//     Unknown,
//     CreateGame,
//     JoinGame,
//     RestartGame,
//     StartReplay,
//     LeaveGame,
//     QuickSave,
//     QuickLoad,
//     Quit,
//     GameInfo,
//     Observation,
//     Action,
//     Step,
//     Data,
//     Query,
//     SaveReplay,
//     ReplayInfo,
//     AvailableMaps,
//     SaveMap,
//     Ping,
//     Debug,
// }

// /// a request to send to the game instance
// #[derive(Debug)]
// pub struct ClientRequest {
//     transaction: Uuid,
//     request: Request,
//     timeout: time::Duration,
//     kind: ClientMessageKind,
// }

// impl ClientRequest {
//     /// create a new request with the default timeout
//     pub fn new(request: Request) -> Self {
//         Self::with_timeout(request, time::Duration::from_secs(5))
//     }

//     /// create a new request with a custom timeout
//     pub fn with_timeout(request: Request, timeout: time::Duration) -> Self {
//         let kind = Self::get_kind(&request);

//         Self {
//             transaction: Uuid::new_v4(),
//             request: request,
//             timeout: timeout,
//             kind: kind,
//         }
//     }

//     fn get_kind(req: &Request) -> ClientMessageKind {
//         if req.has_create_game() {
//             ClientMessageKind::CreateGame
//         } else if req.has_join_game() {
//             ClientMessageKind::JoinGame
//         } else if req.has_restart_game() {
//             ClientMessageKind::RestartGame
//         } else if req.has_start_replay() {
//             ClientMessageKind::StartReplay
//         } else if req.has_leave_game() {
//             ClientMessageKind::LeaveGame
//         } else if req.has_quick_save() {
//             ClientMessageKind::QuickSave
//         } else if req.has_quick_load() {
//             ClientMessageKind::QuickLoad
//         } else if req.has_quit() {
//             ClientMessageKind::Quit
//         } else if req.has_game_info() {
//             ClientMessageKind::GameInfo
//         } else if req.has_observation() {
//             ClientMessageKind::Observation
//         } else if req.has_action() {
//             ClientMessageKind::Action
//         } else if req.has_step() {
//             ClientMessageKind::Step
//         } else if req.has_data() {
//             ClientMessageKind::Data
//         } else if req.has_query() {
//             ClientMessageKind::Query
//         } else if req.has_save_replay() {
//             ClientMessageKind::SaveReplay
//         } else if req.has_replay_info() {
//             ClientMessageKind::ReplayInfo
//         } else if req.has_available_maps() {
//             ClientMessageKind::AvailableMaps
//         } else if req.has_save_map() {
//             ClientMessageKind::SaveMap
//         } else if req.has_ping() {
//             ClientMessageKind::Ping
//         } else if req.has_debug() {
//             ClientMessageKind::Debug
//         } else {
//             ClientMessageKind::Unknown
//         }
//     }
// }

// /// a successful response from the game instance
// #[derive(Debug)]
// pub struct ClientResponse {
//     transaction: Uuid,
//     response: Response,
//     kind: ClientMessageKind,
// }

// /// the result of a transaction with the game instance
// #[derive(Debug)]
// pub enum ClientResult {
//     /// transaction succeeded
//     Success(ClientResponse),
//     /// transaction timed out
//     Timeout(Uuid),
// }

// impl ClientResult {
//     fn success(transaction: Uuid, response: Response) -> Self {
//         let kind = Self::get_kind(&response);

//         ClientResult::Success(ClientResponse {
//             transaction: transaction,
//             response: response,
//             kind: kind,
//         })
//     }

//     fn get_kind(rsp: &Response) -> ClientMessageKind {
//         if rsp.has_create_game() {
//             ClientMessageKind::CreateGame
//         } else if rsp.has_join_game() {
//             ClientMessageKind::JoinGame
//         } else if rsp.has_restart_game() {
//             ClientMessageKind::RestartGame
//         } else if rsp.has_start_replay() {
//             ClientMessageKind::StartReplay
//         } else if rsp.has_leave_game() {
//             ClientMessageKind::LeaveGame
//         } else if rsp.has_quick_save() {
//             ClientMessageKind::QuickSave
//         } else if rsp.has_quick_load() {
//             ClientMessageKind::QuickLoad
//         } else if rsp.has_quit() {
//             ClientMessageKind::Quit
//         } else if rsp.has_game_info() {
//             ClientMessageKind::GameInfo
//         } else if rsp.has_observation() {
//             ClientMessageKind::Observation
//         } else if rsp.has_action() {
//             ClientMessageKind::Action
//         } else if rsp.has_step() {
//             ClientMessageKind::Step
//         } else if rsp.has_data() {
//             ClientMessageKind::Data
//         } else if rsp.has_query() {
//             ClientMessageKind::Query
//         } else if rsp.has_save_replay() {
//             ClientMessageKind::SaveReplay
//         } else if rsp.has_replay_info() {
//             ClientMessageKind::ReplayInfo
//         } else if rsp.has_available_maps() {
//             ClientMessageKind::AvailableMaps
//         } else if rsp.has_save_map() {
//             ClientMessageKind::SaveMap
//         } else if rsp.has_ping() {
//             ClientMessageKind::Ping
//         } else if rsp.has_debug() {
//             ClientMessageKind::Debug
//         } else {
//             ClientMessageKind::Unknown
//         }
//     }
// }

// const NUM_RETRIES: u32 = 10;

// pub enum ClientSoma {
//     Init(Init),
//     AwaitInstance(AwaitInstance),
//     Connect(Connect),

//     Open(Open),

//     Disconnect(Disconnect),
// }

// impl ClientSoma {
//     pub fn sheath() -> Result<Sheath<ClientSoma>> {
//         Ok(Sheath::new(
//             ClientSoma::Init(Init {}),
//             vec![
//                 Dendrite::RequireOne(Synapse::InstanceProvider),
//                 Dendrite::Variadic(Synapse::Client),
//             ],
//             vec![],
//         )?)
//     }
// }

// impl Neuron for ClientSoma {
//     type Signal = Signal;
//     type Synapse = Synapse;

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> organelle::Result<Self> {
//         match self {
//             ClientSoma::Init(state) => state.update(axon, msg),
//             ClientSoma::AwaitInstance(state) => state.update(axon, msg),
//             ClientSoma::Connect(state) => state.update(axon, msg),
//             ClientSoma::Open(state) => state.update(axon, msg),
//             ClientSoma::Disconnect(state) => state.update(axon, msg),
//         }.chain_err(|| organelle::ErrorKind::SomaError)
//     }
// }

// pub struct Init {}

// impl Init {
//     fn update(
//         self,
//         _: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ClientSoma> {
//         match msg {
//             Impulse::Start => self.start(),

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn start(self) -> Result<ClientSoma> {
//         AwaitInstance::await()
//     }
// }

// pub struct AwaitInstance {}

// impl AwaitInstance {
//     fn await() -> Result<ClientSoma> {
//         Ok(ClientSoma::AwaitInstance(AwaitInstance {}))
//     }

//     fn reset(axon: &Axon) -> Result<ClientSoma> {
//         for c in axon.var_input(Synapse::Client)? {
//             axon.effector()?.send(*c, Signal::ClientClosed);
//         }

//         Self::await()
//     }

//     fn reset_error(axon: &Axon, e: Rc<Error>) -> Result<ClientSoma> {
//         for c in axon.var_input(Synapse::Client)? {
//             axon.effector()?.send_in_order(
//                 *c,
// vec![Signal::ClientError(Rc::clone(&e)),
// Signal::ClientClosed],             );
//         }

//         Self::await()
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ClientSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ProvideInstance(instance, url)) => {
//                 self.assign_instance(axon, src, instance, url)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn assign_instance(
//         self,
//         axon: &Axon,
//         src: Handle,
//         _: Uuid,
//         url: Url,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.req_input(Synapse::InstanceProvider)?);

//         Connect::connect(axon, url)
//     }
// }

// pub struct Connect {
//     timer: Timer,
//     retries: u32,
// }

// impl Connect {
//     fn connect(axon: &Axon, url: Url) -> Result<ClientSoma> {
//         let this_soma = axon.effector()?.this_soma();
//         axon.effector()?
//             .send(this_soma, Signal::ClientAttemptConnect(url));

//         Ok(ClientSoma::Connect(Connect {
//             timer: Timer::default(),
//             retries: NUM_RETRIES,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ClientSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientAttemptConnect(url)) => {
//                 self.attempt_connect(axon, src, url)
//             },
//             Impulse::Signal(src, Signal::ClientConnected(sender)) => {
//                 self.on_connected(axon, src, sender)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn attempt_connect(
//         mut self,
//         axon: &Axon,
//         src: Handle,
//         url: Url,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         let connected_effector = axon.effector()?.clone();
//         let retry_effector = axon.effector()?.clone();
//         let timer_effector = axon.effector()?.clone();

//         let client_remote = axon.effector()?.remote();

//         if self.retries == 0 {
//             bail!("unable to connect to instance")
//         } else {
//             println!(
//                 "attempting to connect to instance {} - retries {}",
//                 url, self.retries
//             );

//             self.retries -= 1;
//         }

//         let retry_url = url.clone();

//         axon.effector()?.spawn(
//             self.timer
//                 .sleep(time::Duration::from_secs(5))
//                 .and_then(move |_| {
//                     connect_async(url, client_remote)
//                         .and_then(move |(ws_stream, _)| {
//                             let this_soma = connected_effector.this_soma();

//                             let (send_tx, send_rx) = mpsc::channel(10);

//                             let (sink, stream) = ws_stream.split();

//                             connected_effector.spawn(sink.send_all(
//                                 send_rx.map_err(|_| {
//                                     tungstenite::Error::Io(
//                                         io::ErrorKind::BrokenPipe.into(),
//                                     )
//                                 }),
//                             ).then(|_| Ok(())));

//                             let recv_eff = connected_effector.clone();
//                             let close_eff = connected_effector.clone();
//                             let error_eff = connected_effector.clone();

//                             connected_effector.spawn(
//                                 stream
//                                     .for_each(move |msg| {
//                                         recv_eff.send(
//                                             this_soma,
//                                             Signal::ClientReceive(msg),
//                                         );

//                                         Ok(())
//                                     })
//                                     .and_then(move |_| {
//                                         close_eff.send(
//                                             this_soma,
//                                             Signal::ClientClosed,
//                                         );

//                                         Ok(())
//                                     })
//                                     .or_else(move |e| {
//                                         error_eff.send(
//                                             this_soma,
//                                             Signal::ClientError(Rc::from(
//                                                 Error::with_chain(
//                                                     e,
//
// ErrorKind::ClientRecvFailed,
// ),                                             )),
//                                         );

//                                         Ok(())
//                                     }),
//                             );
//                             connected_effector.send(
//                                 this_soma,
//                                 Signal::ClientConnected(send_tx),
//                             );

//                             Ok(())
//                         })
//                         .or_else(move |_| {
//                             let this_soma = retry_effector.this_soma();
//                             retry_effector.send(
//                                 this_soma,
//                                 Signal::ClientAttemptConnect(retry_url),
//                             );

//                             Ok(())
//                         })
//                 })
//                 .or_else(move |e| {
//                     timer_effector.error(organelle::Error::with_chain(
//                         e,
//                         organelle::ErrorKind::SomaError,
//                     ));

//                     Ok(())
//                 }),
//         );

//         Ok(ClientSoma::Connect(self))
//     }

//     fn on_connected(
//         self,
//         axon: &Axon,
//         src: Handle,
//         sender: mpsc::Sender<tungstenite::Message>,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         Open::open(axon, sender, self.timer)
//     }
// }

// pub struct Open {
//     sender: mpsc::Sender<tungstenite::Message>,
//     timer: Timer,

//     transactions: VecDeque<(Uuid, Handle, oneshot::Sender<()>)>,
// }

// impl Open {
//     fn open(
//         axon: &Axon,
//         sender: mpsc::Sender<tungstenite::Message>,
//         timer: Timer,
//     ) -> Result<ClientSoma> {
//         for c in axon.var_input(Synapse::Client)? {
//             axon.effector()?.send(*c, Signal::Ready);
//         }

//         Ok(ClientSoma::Open(Open {
//             sender: sender,
//             timer: timer,

//             transactions: VecDeque::new(),
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ClientSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientRequest(req)) => {
//                 self.send(axon, src, req)
//             },
//             Impulse::Signal(src, Signal::ClientReceive(msg)) => {
//                 self.recv(axon, src, msg)
//             },
//             Impulse::Signal(src, Signal::ClientTimeout(transaction)) => {
//                 self.on_timeout(axon, src, transaction)
//             },
//             Impulse::Signal(_, Signal::ClientDisconnect) => {
//                 Disconnect::disconnect()
//             },
//             Impulse::Signal(src, Signal::ClientClosed) => {
//                 self.on_close(axon, src)
//             },
//             Impulse::Signal(src, Signal::ClientError(e)) => {
//                 self.on_error(axon, src, e)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn send(
//         mut self,
//         axon: &Axon,
//         src: Handle,
//         req: ClientRequest,
//     ) -> Result<ClientSoma> {
//         let buf = Vec::new();
//         let mut writer = buf.writer();

//         let (tx, rx) = oneshot::channel();
//         let transaction = req.transaction;

//         self.transactions.push_back((transaction, src, tx));

//         {
//             let mut cos = protobuf::CodedOutputStream::new(&mut writer);

//             req.request.write_to(&mut cos)?;
//             cos.flush()?;
//         }

//         let timeout_effector = axon.effector()?.clone();

//         axon.effector()?.spawn(
//             self.timer
//                 .timeout(
//                     self.sender
//                         .clone()
//
// .send(tungstenite::Message::Binary(writer.into_inner()))
// .map_err(|_| ())                         .and_then(|_| rx.map_err(|_| ())),
//                     req.timeout,
//                 )
//                 .and_then(|_| Ok(()))
//                 .or_else(move |_| {
//                     let this_soma = timeout_effector.this_soma();

//                     timeout_effector
//                         .send(this_soma, Signal::ClientTimeout(transaction));

//                     Ok(())
//                 }),
//         );

//         Ok(ClientSoma::Open(self))
//     }

//     fn recv(
//         mut self,
//         axon: &Axon,
//         src: Handle,
//         msg: tungstenite::Message,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         let rsp = match msg {
//             tungstenite::Message::Binary(buf) => {
//                 let cursor = io::Cursor::new(buf);

//                 parse_from_reader::<Response>(&mut cursor.reader())?
//             },
//             _ => bail!("unexpected non-binary message"),
//         };

//         let (transaction, dest, tx) = match self.transactions.pop_front() {
//             Some(transaction) => transaction,
//             None => bail!("no pending transactions for this response"),
//         };

//         if let Err(_) = tx.send(()) {
//             // rx must be closed
//         }

//         axon.effector()?.send(
//             dest,
//             Signal::ClientResult(ClientResult::success(transaction, rsp)),
//         );

//         Ok(ClientSoma::Open(self))
//     }

//     fn on_timeout(
//         mut self,
//         axon: &Axon,
//         src: Handle,
//         transaction: Uuid,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         if let Some(i) = self.transactions
//             .iter()
//             .position(|&(ref t, _, _)| *t == transaction)
//         {
//             let dest = self.transactions[i].1;

//             self.transactions.remove(i);
//             axon.effector()?
//                 .send(dest, Signal::ClientTimeout(transaction));
//         }

//         Ok(ClientSoma::Open(self))
//     }

//     fn on_close(self, axon: &Axon, src: Handle) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         AwaitInstance::reset(axon)
//     }

//     fn on_error(
//         self,
//         axon: &Axon,
//         src: Handle,
//         e: Rc<Error>,
//     ) -> Result<ClientSoma> {
//         assert_eq!(src, axon.effector()?.this_soma());

//         AwaitInstance::reset_error(axon, e)
//     }
// }

// pub struct Disconnect {}

// impl Disconnect {
//     fn disconnect() -> Result<ClientSoma> {
//         Ok(ClientSoma::Disconnect(Disconnect {}))
//     }
//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<ClientSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::ClientClosed) => {
//                 AwaitInstance::reset(axon)
//             },
//             Impulse::Signal(_, Signal::ClientError(e)) => {
//                 AwaitInstance::reset_error(axon, e)
//             },

//             Impulse::Signal(_, msg) => bail!("unexpected msg {:#?}", msg),
//             _ => bail!("unexpected protocol message"),
//         }
//     }
// }
