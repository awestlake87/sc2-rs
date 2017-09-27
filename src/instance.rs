
use std::path::PathBuf;
use std::io;
use std::thread;
use std::time;
use std::process;

use futures::{ Future, Stream, Sink };
use futures::sync::{ oneshot };
use tokio_core::reactor;
use tokio_timer::Timer;
use tokio_tungstenite::connect_async;
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
        reactor.clone().spawn(
            connect_async(url.clone(), reactor.remote().clone()).and_then(
                |ws_stream| {
                    println!("websocket handshake completed successfully");

                    

                    Ok(())
                }
            ).map_err(
                move |e| {
                    println!("websocket handshaked failed: {}", e);
                    println!("retrying...");

                    let timer = Timer::default();

                    reactor.clone().spawn(
                        timer.sleep(time::Duration::from_millis(1000)).then(
                            move |_| {
                                Self::spawn_client(reactor, url);

                                Ok(())
                            }
                        )
                    );
                }
            )
        );
    }
}
