
use std::path::{ PathBuf };
use std::process;
use std::io;
use std::thread;

use utils::Rect;
use result::{ Result, Error };
use agent::Agent;

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
        -> Result<Self>
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

    pub fn launch_starcraft(&mut self) -> Result<()> {
        let exe_file = match self.settings.starcraft_exe {
            Some(ref file) => {
                if file.as_path().is_file() {
                    file.clone()
                }
                else {
                    return Err(
                        Error::ExeDoesNotExist(Some(file.clone()))
                    )
                }
            }
            None => return Err(Error::ExeNotSpecified)
        };

        self.launch_process(exe_file)
    }

    pub fn start_game(&mut self) -> Result<()> {
        Err(Error::Todo("start game"))
    }

    fn launch_process(&mut self, exe_file: PathBuf)
        -> Result<()>
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

                    println!("lalalsldasldasjakd");

                    child.wait()
                }
            )
        );

        Ok(())
    }
}
