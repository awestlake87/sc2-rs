
mod agent;
mod launcher;
mod melee;
mod observer;

pub use self::launcher::{ LauncherLobe };
pub use self::melee::{ MeleeSuite, MeleeSettings, MeleeLobe };
pub use self::observer::{ ObserverLobe };

use cortical::{ Handle };
use uuid::Uuid;

use super::{ Error };

pub enum Message {
    LaunchInstance,
    AvailableInstances(Vec<Uuid>),

    RequestMelee {
        transaction: Uuid,
        player1: Handle,
        player2: Handle
    },
    AssignInstance(Uuid),

    InvalidRequest {
        transaction: Uuid,
        error: Box<Error>,
    }
}
