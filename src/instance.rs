
use std::path::PathBuf;
use std::io;
use std::thread;
use std::time;
use std::process;

use bytes::{ Buf, BufMut };
use futures::{ Future, Stream, Sink };
use futures::sync::{ oneshot };
use protobuf::{ CodedOutputStream, Message };
use sc2_proto::sc2api::{ Request };
use tokio_core::reactor;
use tokio_timer::Timer;
use tokio_tungstenite::{ connect_async };
use tungstenite::{ Message as WebsockMessage };
use url::Url;

use super::{ Result, Error };
use utils::Rect;
use client::{ Client };

#[derive(Clone)]
pub struct InstanceSettings {
    pub reactor:            reactor::Handle,
    pub starcraft_exe:      PathBuf,
    pub port:               u16,
    pub window_rect:        Rect<u32>
}

pub struct Instance {
    settings:           InstanceSettings,
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Result<Self> {
        if settings.starcraft_exe.as_path().is_file() {
            Ok(Self { settings: settings })
        }
        else {
            Err(Error::ExeDoesNotExist(settings.starcraft_exe))
        }
    }

    pub fn run(&self) -> oneshot::Receiver<()> {
        let exe = self.settings.starcraft_exe.clone();
        let port = self.settings.port;
        let window = self.settings.window_rect;

        let (tx, rx) = oneshot::channel::<()>();

        thread::spawn(
            move || {
                let mut child = process::Command::new(exe)
                    .arg("-listen").arg("127.0.0.1")
                    .arg("-port").arg(port.to_string())
                    .arg("-displayMode").arg("0")

                    .arg("-windowx").arg(window.x.to_string())
                    .arg("-windowy").arg(window.y.to_string())
                    .arg("-windowWidth").arg(window.w.to_string())
                    .arg("-windowHeight").arg(window.h.to_string())

                    .spawn()
                    .unwrap()
                ;

                child.wait();

                tx.send(()).unwrap();
            }
        );

        let url = Url::parse(
            &format!("ws://localhost:{}/sc2api", self.settings.port)[..]
        ).expect("somehow I fucked up the URL");

        println!("attempting connection to {:?}", url);

        Self::spawn_client(self.settings.reactor.clone(), url);

        rx
    }

    pub fn spawn_client(reactor: reactor::Handle, url: Url) {
        let ok_reactor = reactor.clone();
        let err_reactor = reactor.clone();

        reactor.spawn(
            connect_async(url.clone(), reactor.remote().clone()).and_then(
                move |(ws_stream, _)| {
                    println!("websocket handshake completed successfully");

                    let mut req = Request::new();

                    req.mut_quit();

                    let buf = Vec::new();
                    let mut writer = buf.writer();

                    {
                        let mut cos = CodedOutputStream::new(&mut writer);

                        req.write_to(&mut cos).unwrap();
                        cos.flush().unwrap();
                    }

                    ok_reactor.spawn(
                        ws_stream.send(
                            WebsockMessage::Binary(writer.into_inner())
                        ).and_then(
                            |_| {
                                println!("yay!");

                                Ok(())
                            }
                        ).map_err(
                            |e| {
                                println!("websocket send failed: {}", e);
                            }
                        )
                    );

                    Ok(())
                }
            ).map_err(
                move |e| {
                    println!("websocket handshaked failed: {}", e);
                    println!("retrying...");

                    let timer = Timer::default();

                    err_reactor.clone().spawn(
                        timer.sleep(time::Duration::from_millis(1000)).then(
                            move |_| {
                                Self::spawn_client(err_reactor, url);

                                Ok(())
                            }
                        )
                    );
                }
            )
        );
    }
}
