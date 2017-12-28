
use std::collections::VecDeque;
use std::io;
use std::time;

use bytes::{ Buf, BufMut };
use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };
use futures::prelude::*;
use futures::sync::{ oneshot, mpsc };
use protobuf;
use protobuf::{ Message as ProtobufMessage, parse_from_reader };
use sc2_proto::sc2api::{ Request, Response };
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;
use uuid::Uuid;

use super::super::{ Result, Error, ErrorKind };
use super::{ Message, Soma, Role };

pub type TransactionId = Uuid;

pub struct Transactor {
    client:         Handle,
    transaction:    TransactionId,
    kind:           ClientMessageKind,
}

impl Transactor {
    pub fn send(soma: &Soma, req: ClientRequest) -> Result<Self> {
        let transaction = req.transaction;
        let kind = req.kind;

        let client = soma.req_output(Role::Client)?;

        soma.effector()?.send(client, Message::ClientRequest(req));

        Ok(
            Self {
                client: client,
                transaction: transaction,
                kind: kind,
            }
        )
    }

    pub fn expect(self, src: Handle, rsp: ClientResponse)
        -> Result<ClientResponse>
    {
        if self.client != src {
            bail!("unexpected source for client response")
        }

        if self.transaction != rsp.transaction {
            bail!("transaction id mismatch")
        }

        if self.kind != rsp.kind {
            bail!("expected {:?} message, got {:?}", self.kind, rsp.kind)
        }

        if rsp.response.get_error().len() != 0 {
            bail!(
                ErrorKind::GameErrors(
                    rsp.response.get_error().iter()
                        .map(|e| e.clone())
                        .collect()
                )
            )
        }

        Ok(rsp)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ClientMessageKind {
    Unknown,
    CreateGame,
    JoinGame,
    RestartGame,
    StartReplay,
    LeaveGame,
    QuickSave,
    QuickLoad,
    Quit,
    GameInfo,
    Observation,
    Action,
    Step,
    Data,
    Query,
    SaveReplay,
    ReplayInfo,
    AvailableMaps,
    SaveMap,
    Ping,
    Debug
}

pub struct ClientRequest {
    pub transaction: TransactionId,
    pub request: Request,
    pub timeout: time::Duration,
    pub kind: ClientMessageKind,
}

impl ClientRequest {
    pub fn new(request: Request) -> Self {
        Self::with_timeout(request, time::Duration::from_secs(5))
    }

    pub fn with_timeout(request: Request, timeout: time::Duration)
        -> Self
    {
        let kind = Self::get_kind(&request);

        Self {
            transaction: Uuid::new_v4(),
            request: request,
            timeout: timeout,
            kind: kind
        }
    }

    fn get_kind(req: &Request) -> ClientMessageKind {
        if req.has_create_game() {
            ClientMessageKind::CreateGame
        }
        else if req.has_join_game() {
            ClientMessageKind::JoinGame
        }
        else if req.has_restart_game() {
            ClientMessageKind::RestartGame
        }
        else if req.has_start_replay() {
            ClientMessageKind::StartReplay
        }
        else if req.has_leave_game() {
            ClientMessageKind::LeaveGame
        }
        else if req.has_quick_save() {
            ClientMessageKind::QuickSave
        }
        else if req.has_quick_load() {
            ClientMessageKind::QuickLoad
        }
        else if req.has_quit() {
            ClientMessageKind::Quit
        }
        else if req.has_game_info() {
            ClientMessageKind::GameInfo
        }
        else if req.has_observation() {
            ClientMessageKind::Observation
        }
        else if req.has_action() {
            ClientMessageKind::Action
        }
        else if req.has_step() {
            ClientMessageKind::Step
        }
        else if req.has_data() {
            ClientMessageKind::Data
        }
        else if req.has_query() {
            ClientMessageKind::Query
        }
        else if req.has_save_replay() {
            ClientMessageKind::SaveReplay
        }
        else if req.has_replay_info() {
            ClientMessageKind::ReplayInfo
        }
        else if req.has_available_maps() {
            ClientMessageKind::AvailableMaps
        }
        else if req.has_save_map() {
            ClientMessageKind::SaveMap
        }
        else if req.has_ping() {
            ClientMessageKind::Ping
        }
        else if req.has_debug() {
            ClientMessageKind::Debug
        }
        else {
            ClientMessageKind::Unknown
        }
    }
}

pub struct ClientResponse {
    pub transaction: TransactionId,
    pub response: Response,
    pub kind: ClientMessageKind,
}

impl ClientResponse {
    fn new(transaction: TransactionId, response: Response) -> Self {
        let kind = Self::get_kind(&response);

        Self {
            transaction: transaction,
            response: response,
            kind: kind,
        }
    }

    fn get_kind(rsp: &Response) -> ClientMessageKind {
        if rsp.has_create_game() {
            ClientMessageKind::CreateGame
        }
        else if rsp.has_join_game() {
            ClientMessageKind::JoinGame
        }
        else if rsp.has_restart_game() {
            ClientMessageKind::RestartGame
        }
        else if rsp.has_start_replay() {
            ClientMessageKind::StartReplay
        }
        else if rsp.has_leave_game() {
            ClientMessageKind::LeaveGame
        }
        else if rsp.has_quick_save() {
            ClientMessageKind::QuickSave
        }
        else if rsp.has_quick_load() {
            ClientMessageKind::QuickLoad
        }
        else if rsp.has_quit() {
            ClientMessageKind::Quit
        }
        else if rsp.has_game_info() {
            ClientMessageKind::GameInfo
        }
        else if rsp.has_observation() {
            ClientMessageKind::Observation
        }
        else if rsp.has_action() {
            ClientMessageKind::Action
        }
        else if rsp.has_step() {
            ClientMessageKind::Step
        }
        else if rsp.has_data() {
            ClientMessageKind::Data
        }
        else if rsp.has_query() {
            ClientMessageKind::Query
        }
        else if rsp.has_save_replay() {
            ClientMessageKind::SaveReplay
        }
        else if rsp.has_replay_info() {
            ClientMessageKind::ReplayInfo
        }
        else if rsp.has_available_maps() {
            ClientMessageKind::AvailableMaps
        }
        else if rsp.has_save_map() {
            ClientMessageKind::SaveMap
        }
        else if rsp.has_ping() {
            ClientMessageKind::Ping
        }
        else if rsp.has_debug() {
            ClientMessageKind::Debug
        }
        else {
            ClientMessageKind::Unknown
        }
    }
}

const NUM_RETRIES: u32 = 10;

pub enum ClientLobe {
    Init(ClientInit),
    AwaitInstance(ClientAwaitInstance),
    Connect(ClientConnect),

    Open(ClientOpen),

    Closed(ClientClosed),
}

impl ClientLobe {
    pub fn new() -> Result<Self> {
        Ok(
            ClientLobe::Init(
                ClientInit {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::InstanceProvider),
                            Constraint::RequireOne(Role::Client)
                        ],
                        vec![ ]
                    )?,
                }
            )
        )
    }
}

impl Lobe for ClientLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> cortical::Result<Self> {
        match self {
            ClientLobe::Init(state) => state.update(msg),
            ClientLobe::AwaitInstance(state) => state.update(msg),
            ClientLobe::Connect(state) => state.update(msg),
            ClientLobe::Open(state) => state.update(msg),
            ClientLobe::Closed(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct ClientInit {
    soma:               Soma
}

impl ClientInit {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Start => self.start(),

            _ => Ok(ClientLobe::Init(self))
        }
    }

    fn start(self) -> Result<ClientLobe> {
        Ok(ClientLobe::AwaitInstance(ClientAwaitInstance { soma: self.soma }))
    }
}

pub struct ClientAwaitInstance {
    soma:               Soma,
}

impl ClientAwaitInstance {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(
                src, Message::ProvideInstance(instance, url)
            ) => {
                self.assign_instance(src, instance, url)
            },

            _ => Ok(ClientLobe::AwaitInstance(self))
        }
    }

    fn assign_instance(self, src: Handle, _: Uuid, url: Url)
        -> Result<ClientLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::InstanceProvider)?);

        let this_lobe = self.soma.effector()?.this_lobe();
        self.soma.effector()?.send(
            this_lobe, Message::AttemptConnect(url)
        );

        Ok(
            ClientLobe::Connect(
                ClientConnect {
                    soma: self.soma,

                    timer: Timer::default(),
                    retries: NUM_RETRIES,
                }
            )
        )
    }
}

pub struct ClientConnect {
    soma:               Soma,

    timer:              Timer,
    retries:            u32,
}

impl ClientConnect {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::AttemptConnect(url)) => {
                self.attempt_connect(src, url)
            },
            Protocol::Message(src, Message::Connected(sender)) => {
                self.on_connected(src, sender)
            },

            _ => Ok(ClientLobe::Connect(self))
        }
    }

    fn attempt_connect(mut self, src: Handle, url: Url) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        let connected_effector = self.soma.effector()?.clone();
        let retry_effector = self.soma.effector()?.clone();
        let timer_effector = self.soma.effector()?.clone();

        let client_remote = self.soma.effector()?.remote();

        if self.retries == 0 {
            bail!("unable to connect to instance")
        }
        else {
            println!(
                "attempting to connect to instance {} - retries {}",
                url,
                self.retries
            );

            self.retries -= 1;
        }

        let retry_url = url.clone();

        self.soma.effector()?.spawn(
            self.timer.sleep(time::Duration::from_secs(5))
                .and_then(move |_| connect_async(url, client_remote)
                    .and_then(move |(ws_stream, _)| {
                        let this_lobe = connected_effector.this_lobe();

                        let (send_tx, send_rx) = mpsc::channel(10);

                        let (sink, stream) = ws_stream.split();

                        connected_effector.spawn(
                            sink.send_all(
                                send_rx.map_err(
                                    |_| tungstenite::Error::Io(
                                        io::ErrorKind::BrokenPipe
                                            .into()
                                    )
                                )
                            )
                                .then(|_| Ok(()))
                        );

                        let recv_eff = connected_effector.clone();
                        let close_eff = connected_effector.clone();
                        let error_eff = connected_effector.clone();

                        connected_effector.spawn(
                            stream.for_each(move |msg| {
                                recv_eff.send(
                                    this_lobe, Message::ClientReceive(msg)
                                );

                                Ok(())
                            })
                                .and_then(move |_| {
                                    close_eff.send(
                                        this_lobe, Message::ClientClosed
                                    );

                                    Ok(())
                                })
                                .or_else(move |e| {
                                    error_eff.send(
                                        this_lobe,
                                        Message::ClientError(
                                            Error::with_chain(
                                                e,
                                                ErrorKind::ClientRecvFailed
                                            )
                                        )
                                    );

                                    Ok(())
                                })
                        );
                        connected_effector.send(
                            this_lobe,
                            Message::Connected(send_tx)
                        );

                        Ok(())
                    })
                    .or_else(move |_| {
                        let this_lobe = retry_effector.this_lobe();
                        retry_effector.send(
                            this_lobe,
                            Message::AttemptConnect(retry_url)
                        );

                        Ok(())
                    })
                )
                .or_else(move |e| {
                    timer_effector.error(
                        cortical::Error::with_chain(
                            e, cortical::ErrorKind::LobeError
                        )
                    );

                    Ok(())
                })
        );

        Ok(ClientLobe::Connect(self))
    }

    fn on_connected(
        self,
        src: Handle,
        sender: mpsc::Sender<tungstenite::Message>
    )
        -> Result<ClientLobe>
    {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        self.soma.send_req_input(Role::Client, Message::Ready)?;

        Ok(
            ClientLobe::Open(
                ClientOpen {
                    soma: self.soma,
                    sender: sender,
                    timer: self.timer,

                    transactions: VecDeque::new(),
                }
            )
        )
    }
}

pub struct ClientOpen {
    soma:           Soma,
    sender:         mpsc::Sender<tungstenite::Message>,
    timer:          Timer,

    transactions:   VecDeque<
        (TransactionId, oneshot::Sender<()>)
    >,
}

impl ClientOpen {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientRequest(req)) => {
                self.send(src, req)
            },
            Protocol::Message(src, Message::ClientReceive(msg)) => {
                self.recv(src, msg)
            },
            Protocol::Message(src, Message::ClientTimeout(transaction)) => {
                self.on_timeout(src, transaction)
            }
            Protocol::Message(src, Message::ClientClosed) => {
                self.on_close(src)
            },
            Protocol::Message(src, Message::ClientError(e)) => {
                self.on_error(src, e)
            }
            _ => Ok(ClientLobe::Open(self))
        }
    }

    fn send(mut self, src: Handle, req: ClientRequest) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.req_input(Role::Client)?);

        let buf = Vec::new();
        let mut writer = buf.writer();


        let (tx, rx) = oneshot::channel();
        let transaction = req.transaction;

        self.transactions.push_back((transaction, tx));

        {
            let mut cos = protobuf::CodedOutputStream::new(&mut writer);

            req.request.write_to(&mut cos)?;
            cos.flush()?;
        }

        let timeout_effector = self.soma.effector()?.clone();

        self.soma.effector()?.spawn(
            self.timer.timeout(
                self.sender.clone().send(
                    tungstenite::Message::Binary(writer.into_inner())
                )
                    .map_err(|_| ())
                    .and_then(|_| rx.map_err(|_| ())),
                req.timeout
            )
            .and_then(|_| Ok(()))
            .or_else(move |_| {
                let this_lobe = timeout_effector.this_lobe();

                timeout_effector.send(
                    this_lobe, Message::ClientTimeout(transaction)
                );

                Ok(())
            })
        );

        Ok(ClientLobe::Open(self))
    }

    fn recv(mut self, src: Handle, msg: tungstenite::Message)
        -> Result<ClientLobe>
    {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        let rsp = match msg {
            tungstenite::Message::Binary(buf) => {
                let cursor = io::Cursor::new(buf);

                parse_from_reader::<Response>(&mut cursor.reader())?
            }
            _ => bail!("unexpected non-binary message"),
        };

        let (transaction, tx) = match self.transactions.pop_front() {
            Some(transaction) => transaction,
            None => bail!("no pending transactions for this response"),
        };

        if let Err(_) = tx.send(()) {
            // rx must be closed
        }

        self.soma.send_req_input(
            Role::Client,
            Message::ClientResponse(ClientResponse::new(transaction, rsp))
        )?;

        Ok(ClientLobe::Open(self))
    }

    fn on_timeout(mut self, src: Handle, transaction: TransactionId)
        -> Result<ClientLobe>
    {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        if let Some(i) = self.transactions.iter()
            .position(|&(ref t, _)| *t == transaction)
        {
            self.transactions.remove(i);
            self.soma.send_req_input(
                Role::Client, Message::ClientTimeout(transaction)
            )?;
        }

        Ok(ClientLobe::Open(self))
    }

    fn on_close(self, src: Handle) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        self.soma.send_req_input(Role::Client, Message::ClientClosed)?;

        Ok(ClientLobe::Closed(ClientClosed { soma: self.soma }))
    }

    fn on_error(self, src: Handle, e: Error) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        self.soma.send_req_input(Role::Client, Message::ClientError(e))?;
        self.soma.send_req_input(Role::Client, Message::ClientClosed)?;

        Ok(ClientLobe::Closed(ClientClosed { soma: self.soma }))
    }
}

pub struct ClientClosed {
    soma:           Soma,
}

impl ClientClosed {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        Ok(ClientLobe::Closed(self))
    }
}
