
use std::collections::{ VecDeque };
use std::io;
use std::mem;
use std::sync::{ Arc, Mutex, Condvar };
use std::thread;
use std::time::Duration;

use bytes::{ Buf, BufMut };
use futures::{ Future, Stream, Sink };
use futures::sync::{ oneshot, mpsc };
use protobuf::{ CodedOutputStream, Message, parse_from_reader  };
use sc2_proto::sc2api::{ Request, Response };
use tungstenite::{ Message as WebSocketMessage };
use tokio_core::reactor;
use tokio_tungstenite::{ connect_async };
use url::Url;

use super::{ Result, Error };

pub struct Client {
    core_remote: reactor::Remote,
    sender: Option<mpsc::Sender<WebSocketMessage>>,
    queue: Arc<(Mutex<VecDeque<WebSocketMessage>>, Condvar)>,
}

impl Client {
    pub fn connect(url: Url) -> Result<Self> {
        let (tx, rx) = oneshot::channel();

        let guard = thread::spawn(
            move || {
                let mut core = reactor::Core::new().unwrap();
                let hdl = core.handle();

                let client = connect_async(url, hdl.remote().clone()).and_then(
                    |(ws_stream, _)| {
                        let (sink, stream) = ws_stream.split();
                        let (send_tx, send_rx) = mpsc::channel(0);

                        // queue messages on this hdl
                        let q_tx = Arc::new(
                            (Mutex::new(VecDeque::new()), Condvar::new())
                        );

                        // recv messages on this hdl
                        let q_rx = Arc::clone(&q_tx);

                        match tx.send(
                            Self {
                                core_remote: hdl.remote().clone(),
                                sender: Some(send_tx),
                                queue: q_rx
                            }
                        ) {
                            Ok(_) => (),
                            Err(_) => panic!("unable to complete oneshot!")
                        }

                        // pushes rsps onto q_tx
                        stream.for_each(
                            move |msg| {
                                q_tx.0.lock().unwrap().push_back(msg);
                                q_tx.1.notify_one();

                                Ok(())
                            }
                        ).map(|_| ()).select(
                            // forward msgs to the sink
                            sink.send_all(
                                send_rx.map_err(
                                    |_| io::Error::new(
                                        io::ErrorKind::BrokenPipe,
                                        "closed"
                                    )
                                )
                            ).map(|_| ())
                        ).then(|_| Ok(()))
                    }
                );

                match core.run(client) {
                    Ok(_) => (),
                    Err(e) => eprintln!("client error: {}", e)
                }
            }
        );

        match rx.wait() {
            Ok(client) => Ok(client),
            Err(e) => {
                eprintln!("unable to wait for result: {}", e);

                match guard.join() {
                    Ok(_) => (),
                    Err(e) => eprintln!("unable to join client: {:?}", e)
                }

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

        let (tx, rx) = oneshot::channel();

        self.core_remote.spawn(
            move |handle| {
                handle.spawn(
                    sender.send(
                        WebSocketMessage::Binary(writer.into_inner())
                    ).and_then(
                        |sender| match tx.send(sender) {
                            Ok(_) => Ok(()),
                            _ => panic!("unable to complete oneshot!")
                        }
                    ).map_err(|e| eprintln!("send failed: {}", e))
                );
                Ok(())
            }
        );

        match rx.wait() {
            Ok(sender) => {
                self.sender = Some(sender);
                Ok(())
            },
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }

    pub fn close(&mut self) -> Result<()> {
        match self.sender {
            Some(ref mut sender) => match sender.close() {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("close failed: {}", e);
                    Err(Error::WebsockSendFailed)
                },
            },
            None => {
                eprintln!("sender does not exist");
                Err(Error::WebsockSendFailed)
            }
        }
    }

    pub fn recv(&mut self, timeout: Duration) -> Result<Response> {
        let rsp = {
            let mut q = self.queue.0.lock().unwrap();

            if q.is_empty() {
                let (mut q, result) = self.queue.1.wait_timeout(q, timeout)
                    .unwrap()
                ;

                if !result.timed_out() {
                    q.pop_front()
                }
                else {
                    return Err(Error::WebsockRecvFailed)
                }
            }
            else {
                q.pop_front()
            }
        };

        match rsp {
            Some(WebSocketMessage::Binary(buf)) => {
                let cursor = io::Cursor::new(buf);

                match parse_from_reader::<Response>(&mut cursor.reader()) {
                    Ok(rsp) => Ok(rsp),
                    Err(e) => {
                        eprintln!("unable to parse response: {}", e);

                        Err(Error::WebsockRecvFailed)
                    }
                }
            }
            Some(_) => {
                eprintln!("unexpected non-binary message");
                Err(Error::WebsockRecvFailed)
            },
            None => {
                eprintln!("no item in queue?!");
                Err(Error::WebsockRecvFailed)
            }
        }
    }
}
