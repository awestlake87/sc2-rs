
use std::io;
use std::time;

use bytes::{ BufMut };
use futures::{ Future, Stream, Sink };
use futures::sync::{ oneshot, mpsc };
use protobuf::{ CodedOutputStream, Message };
use sc2_proto::sc2api::{ Request };
use tokio_core::{ reactor };
use tokio_timer::Timer;
use tokio_tungstenite::{ connect_async };
use tungstenite;
use url::Url;

use super::{ Result, Error };

pub struct Client {
    sender:         mpsc::Sender<tungstenite::Message>,
}

impl Client {
    pub fn connect(reactor: reactor::Handle, url: Url)
        -> oneshot::Receiver<Self>
    {
        let (tx, rx) = oneshot::channel::<Self>();

        attempt_connect(reactor, url, tx);

        rx
    }

    pub fn send(&mut self, req: Request) -> Result<()> {
        let buf = Vec::new();
        let mut writer = buf.writer();

        {
            let mut cos = CodedOutputStream::new(&mut writer);

            req.write_to(&mut cos).unwrap();
            cos.flush().unwrap();
        }

        match
            self.sender.clone().send(
                tungstenite::Message::Binary(writer.into_inner())
            ).wait()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }

    pub fn quit(&mut self) -> Result<()> {
        let mut req = Request::new();

        req.mut_quit();

        self.send(req)
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

                        let (sink, _) = ws_stream.split();
                        let (ws_tx, ws_rx) = mpsc::channel(0);

                        reactor.spawn(
                            ws_rx.map_err(
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

                        tx.send(Client { sender: ws_tx })
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
