
use std::fmt;
use std::path::{ PathBuf };
use std::process;
use std::io;
use std::thread;
use std::time;

use ws::{ connect };

use utils::Rect;

use agent::Agent;
use client::Client;

pub struct CoordinatorSettings {
    pub starcraft_exe:      Option<PathBuf>,
    pub port:               Option<u16>,
    pub window_rect:        Rect<u32>
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            starcraft_exe: None,
            port: None,
            window_rect: Rect::<u32> {
                x: 100, y: 200, w: 1024, h: 768
            },
        }
    }
}

pub enum CoordinatorErr {
    ExeDoesNotExist(Option<PathBuf>),
    ExeNotSpecified,

    UnableToStartProcess(io::Error),

    Todo(&'static str),
}

impl fmt::Debug for CoordinatorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoordinatorErr::ExeDoesNotExist(ref path) => write!(
                f, "starcraft exe {:?} does not exist", path
            ),
            CoordinatorErr::ExeNotSpecified => write!(
                f, "starcraft exe not specified"
            ),
            CoordinatorErr::UnableToStartProcess(ref err) => write!(
                f, "unable to start process {:?}", *err
            ),
            CoordinatorErr::Todo(ref msg) => write!(f, "todo {:?}", *msg)
        }
    }
}

type AgentList = Vec<Box<Agent>>;

pub struct Coordinator {
    settings:           CoordinatorSettings,
    participants:       AgentList,
    starcraft_thread:   Option<
        thread::JoinHandle<io::Result<process::ExitStatus>>
    >,
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings)
        -> Result<Self, CoordinatorErr>
    {
        Ok(
            Self {
                settings: settings,
                participants: AgentList::new(),
                starcraft_thread: None
            }
        )
    }

    pub fn set_participants(&mut self, participants: AgentList) {
        self.participants = participants;
    }

    pub fn launch_starcraft(&mut self) -> Result<(), CoordinatorErr> {
        let exe_file = match self.settings.starcraft_exe {
            Some(ref file) => {
                if file.as_path().is_file() {
                    file.clone()
                }
                else {
                    return Err(
                        CoordinatorErr::ExeDoesNotExist(Some(file.clone()))
                    )
                }
            }
            None => return Err(CoordinatorErr::ExeNotSpecified)
        };

        self.launch_process(exe_file)
    }

    pub fn start_game(&mut self) -> Result<(), CoordinatorErr> {
        Err(CoordinatorErr::Todo("start game"))
    }

    fn launch_process(&mut self, exe_file: PathBuf)
        -> Result<(), CoordinatorErr>
    {
        let port = match self.settings.port {
            Some(port) => port,
            None => 9168
        };
        let window = self.settings.window_rect;

        self.starcraft_thread = Some(
            thread::spawn(
                move || {
                    let mut child = match
                        process::Command::new(exe_file)
                            .arg("-listen").arg("127.0.0.1")
                            .arg("-port").arg(port.to_string())
                            .arg("-displayMode").arg("0")

                            .arg("-windowx").arg(window.x.to_string())
                            .arg("-windowy").arg(window.y.to_string())
                            .arg("-windowWidth").arg(window.w.to_string())
                            .arg("-windowHeight").arg(window.h.to_string())

                            .spawn()
                    {
                        Ok(child) => child,
                        Err(e) => return Err(e)
                    };

                    child.wait()
                }
            )
        );

        thread::sleep(time::Duration::from_millis(5000));

        let result = match
            connect(
                format!("ws://localhost:{}/{}", port, "sc2api"),
                |out| Client { out: out }
            )
        {
            Ok(_) => Ok(()),
            Err(_) => Err(
                CoordinatorErr::Todo("unable to open websocket")
            )
        };

        self.starcraft_thread.take().unwrap().join().unwrap();
        self.starcraft_thread = None;

        result
    }
}
