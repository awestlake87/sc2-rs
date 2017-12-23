
mod launcher;
mod melee;
mod observer;

pub use self::launcher::{ LauncherLobe };
pub use self::melee::{ MeleeLobe };
pub use self::observer::{ ObserverLobe };

use uuid::Uuid;

pub enum Message {
    LaunchInstance,
    AvailableInstances(Vec<Uuid>),
}
