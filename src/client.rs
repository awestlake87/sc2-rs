
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;
use std::time;

use bytes::{ Buf, BufMut };
use organelle;
use organelle::{
    ResultExt, Handle, Cell, Protocol, Constraint
};
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

use super::{ Result, Error, ErrorKind, Message, Soma, Role };

/// keeps a record of a req/rsp transaction between the game instance
pub struct Transactor {
    client:         Handle,
    transaction:    Uuid,
    kind:           ClientMessageKind,
}

impl Transactor {
    /// send a client request to the client cell
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

    /// expect the result to contain the response expected by this transactor
    pub fn expect(self, src: Handle, result: ClientResult)
        -> Result<Response>
    {
        match result {
            ClientResult::Success(rsp) => {
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

                Ok(rsp.response)
            },
            ClientResult::Timeout(transaction) => {
                if self.transaction != transaction {
                    bail!("transaction id mismatch")
                }
                else {
                    bail!("transaction timed out")
                }
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ClientMessageKind {
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

/// a request to send to the game instance
#[derive(Debug)]
pub struct ClientRequest {
    transaction: Uuid,
    request: Request,
    timeout: time::Duration,
    kind: ClientMessageKind,
}

impl ClientRequest {
    /// create a new request with the default timeout
    pub fn new(request: Request) -> Self {
        Self::with_timeout(request, time::Duration::from_secs(5))
    }

    /// create a new request with a custom timeout
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

/// a successful response from the game instance
#[derive(Debug)]
pub struct ClientResponse {
    transaction: Uuid,
    response: Response,
    kind: ClientMessageKind,
}

/// the result of a transaction with the game instance
#[derive(Debug)]
pub enum ClientResult {
    /// transaction succeeded
    Success(ClientResponse),
    /// transaction timed out
    Timeout(Uuid),
}

impl ClientResult {
    fn success(transaction: Uuid, response: Response) -> Self {
        let kind = Self::get_kind(&response);

        ClientResult::Success(
            ClientResponse {
                transaction: transaction,
                response: response,
                kind: kind,
            }
        )
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

pub enum ClientCell {
    Init(Init),
    AwaitInstance(AwaitInstance),
    Connect(Connect),

    Open(Open),

    Disconnect(Disconnect),
}

impl ClientCell {
    pub fn new() -> Result<Self> {
        Ok(
            ClientCell::Init(
                Init {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::InstanceProvider),
                            Constraint::Variadic(Role::Client)
                        ],
                        vec![ ]
                    )?,
                }
            )
        )
    }
}

impl Cell for ClientCell {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> organelle::Result<Self> {
        match self {
            ClientCell::Init(state) => state.update(msg),
            ClientCell::AwaitInstance(state) => state.update(msg),
            ClientCell::Connect(state) => state.update(msg),
            ClientCell::Open(state) => state.update(msg),
            ClientCell::Disconnect(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::CellError
        )
    }
}

pub struct Init {
    soma:               Soma
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => self.start(),

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ClientCell::Init(self))
        }
    }

    fn start(self) -> Result<ClientCell> {
        AwaitInstance::await(self.soma)
    }
}

pub struct AwaitInstance {
    soma:               Soma,
}

impl AwaitInstance {
    fn await(soma: Soma) -> Result<ClientCell> {
        Ok(ClientCell::AwaitInstance(AwaitInstance { soma: soma }))
    }

    fn reset(soma: Soma) -> Result<ClientCell> {
        for c in soma.var_input(Role::Client)? {
            soma.effector()?.send(*c, Message::ClientClosed);
        }

        Self::await(soma)
    }

    fn reset_error(soma: Soma, e: Rc<Error>) -> Result<ClientCell> {
        for c in soma.var_input(Role::Client)? {
            soma.effector()?.send_in_order(
                *c,
                vec![
                    Message::ClientError(Rc::clone(&e)),
                    Message::ClientClosed
                ]
            );
        }

        Self::await(soma)
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(
                    src, Message::ProvideInstance(instance, url)
                ) => {
                    self.assign_instance(src, instance, url)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ClientCell::AwaitInstance(self))
        }
    }

    fn assign_instance(self, src: Handle, _: Uuid, url: Url)
        -> Result<ClientCell>
    {
        assert_eq!(src, self.soma.req_input(Role::InstanceProvider)?);

        Connect::connect(self.soma, url)
    }
}

pub struct Connect {
    soma:               Soma,

    timer:              Timer,
    retries:            u32,
}

impl Connect {
    fn connect(soma: Soma, url: Url) -> Result<ClientCell> {
        let this_cell = soma.effector()?.this_cell();
        soma.effector()?.send(
            this_cell, Message::ClientAttemptConnect(url)
        );

        Ok(
            ClientCell::Connect(
                Connect {
                    soma: soma,

                    timer: Timer::default(),
                    retries: NUM_RETRIES,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientAttemptConnect(url)) => {
                    self.attempt_connect(src, url)
                },
                Protocol::Message(src, Message::ClientConnected(sender)) => {
                    self.on_connected(src, sender)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ClientCell::Connect(self))
        }
    }

    fn attempt_connect(mut self, src: Handle, url: Url) -> Result<ClientCell> {
        assert_eq!(src, self.soma.effector()?.this_cell());

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
                        let this_cell = connected_effector.this_cell();

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
                                    this_cell, Message::ClientReceive(msg)
                                );

                                Ok(())
                            })
                                .and_then(move |_| {
                                    close_eff.send(
                                        this_cell, Message::ClientClosed
                                    );

                                    Ok(())
                                })
                                .or_else(move |e| {
                                    error_eff.send(
                                        this_cell,
                                        Message::ClientError(
                                            Rc::from(
                                                Error::with_chain(
                                                    e,
                                                    ErrorKind::ClientRecvFailed
                                                )
                                            )
                                        )
                                    );

                                    Ok(())
                                })
                        );
                        connected_effector.send(
                            this_cell,
                            Message::ClientConnected(send_tx)
                        );

                        Ok(())
                    })
                    .or_else(move |_| {
                        let this_cell = retry_effector.this_cell();
                        retry_effector.send(
                            this_cell,
                            Message::ClientAttemptConnect(retry_url)
                        );

                        Ok(())
                    })
                )
                .or_else(move |e| {
                    timer_effector.error(
                        organelle::Error::with_chain(
                            e, organelle::ErrorKind::CellError
                        )
                    );

                    Ok(())
                })
        );

        Ok(ClientCell::Connect(self))
    }

    fn on_connected(
        self,
        src: Handle,
        sender: mpsc::Sender<tungstenite::Message>
    )
        -> Result<ClientCell>
    {
        assert_eq!(src, self.soma.effector()?.this_cell());

        Open::open(self.soma, sender, self.timer)
    }
}

pub struct Open {
    soma:           Soma,
    sender:         mpsc::Sender<tungstenite::Message>,
    timer:          Timer,

    transactions:   VecDeque<
                        (Uuid, Handle, oneshot::Sender<()>)
                    >,
}

impl Open {
    fn open(
        soma: Soma, sender: mpsc::Sender<tungstenite::Message>, timer: Timer
    )
        -> Result<ClientCell>
    {
        for c in soma.var_input(Role::Client)? {
            soma.effector()?.send(*c, Message::Ready);
        }

        Ok(
            ClientCell::Open(
                Open {
                    soma: soma,
                    sender: sender,
                    timer: timer,

                    transactions: VecDeque::new(),
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientRequest(req)) => {
                    self.send(src, req)
                },
                Protocol::Message(src, Message::ClientReceive(msg)) => {
                    self.recv(src, msg)
                },
                Protocol::Message(
                    src, Message::ClientTimeout(transaction)
                ) => {
                    self.on_timeout(src, transaction)
                },
                Protocol::Message(_, Message::ClientDisconnect) => {
                    Disconnect::disconnect(self.soma)
                },
                Protocol::Message(src, Message::ClientClosed) => {
                    self.on_close(src)
                },
                Protocol::Message(src, Message::ClientError(e)) => {
                    self.on_error(src, e)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ClientCell::Open(self))
        }
    }

    fn send(mut self, src: Handle, req: ClientRequest) -> Result<ClientCell> {
        let buf = Vec::new();
        let mut writer = buf.writer();


        let (tx, rx) = oneshot::channel();
        let transaction = req.transaction;

        self.transactions.push_back((transaction, src, tx));

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
                let this_cell = timeout_effector.this_cell();

                timeout_effector.send(
                    this_cell, Message::ClientTimeout(transaction)
                );

                Ok(())
            })
        );

        Ok(ClientCell::Open(self))
    }

    fn recv(mut self, src: Handle, msg: tungstenite::Message)
        -> Result<ClientCell>
    {
        assert_eq!(src, self.soma.effector()?.this_cell());

        let rsp = match msg {
            tungstenite::Message::Binary(buf) => {
                let cursor = io::Cursor::new(buf);

                parse_from_reader::<Response>(&mut cursor.reader())?
            }
            _ => bail!("unexpected non-binary message"),
        };

        let (transaction, dest, tx) = match self.transactions.pop_front() {
            Some(transaction) => transaction,
            None => bail!("no pending transactions for this response"),
        };

        if let Err(_) = tx.send(()) {
            // rx must be closed
        }

        self.soma.effector()?.send(
            dest,
            Message::ClientResult(ClientResult::success(transaction, rsp))
        );

        Ok(ClientCell::Open(self))
    }

    fn on_timeout(mut self, src: Handle, transaction: Uuid)
        -> Result<ClientCell>
    {
        assert_eq!(src, self.soma.effector()?.this_cell());

        if let Some(i) = self.transactions.iter()
            .position(|&(ref t, _, _)| *t == transaction)
        {
            let dest = self.transactions[i].1;

            self.transactions.remove(i);
            self.soma.effector()?.send(
                dest, Message::ClientTimeout(transaction)
            );
        }

        Ok(ClientCell::Open(self))
    }

    fn on_close(self, src: Handle) -> Result<ClientCell> {
        assert_eq!(src, self.soma.effector()?.this_cell());

        AwaitInstance::reset(self.soma)
    }

    fn on_error(self, src: Handle, e: Rc<Error>) -> Result<ClientCell> {
        assert_eq!(src, self.soma.effector()?.this_cell());

        AwaitInstance::reset_error(self.soma, e)
    }
}

pub struct Disconnect {
    soma:           Soma,
}

impl Disconnect {
    fn disconnect(soma: Soma) -> Result<ClientCell> {
        Ok(ClientCell::Disconnect(Disconnect { soma: soma }))
    }
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::ClientClosed) => {
                    AwaitInstance::reset(self.soma)
                },
                Protocol::Message(_, Message::ClientError(e)) => {
                    AwaitInstance::reset_error(self.soma, e)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected msg {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ClientCell::Disconnect(self))
        }
    }
}
