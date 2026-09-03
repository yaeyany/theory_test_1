use std::fmt::{self, Display, Formatter};

use crate::bots::{BotCharge, BotCoordinates, BotId, BotState, BotTitle};

pub enum Commands {
    Create {
        title: BotTitle,
        coordinates: BotCoordinates,
        state: BotState,
        charge: BotCharge
    },
    Get {
        id: BotId
    },
    Update {
        id: BotId,
        title: Option<BotTitle>,
        coordinates: Option<BotCoordinates>,
        state: Option<BotState>,
        charge: Option<BotCharge>
    },
    List,
    Delete {
        id: BotId,
    },
    Shutdown
}

pub enum CommandResponse {
    UnknownCommand,
    CreatedBot {
        id: BotId
    },
    InvalidTItle,
    InvalidId,
    InvalidCoordinates,
    InvalidState,
    InvalidCharge,
    GetBot {
        title: BotTitle,
        coordinates: BotCoordinates,
        state: BotState,
        charge: BotCharge
    },
    UpdatedBot,
    BotList {
        bots: Vec<(BotId, BotTitle)>
    },
    DeletedBot,
    Shutdown
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandResponse::UnknownCommand => write!(f, 
                "Unknown command"
            ),
            CommandResponse::CreatedBot { id }=> write!(f, 
                "Bot created. Id: {}", 
                id
            ),
            CommandResponse::InvalidTItle => write!(f, 
                "Invalid title"
            ),
            CommandResponse::InvalidId => write!(f, 
                "Invalid ID"
            ),
            CommandResponse::InvalidCoordinates => write!(f, 
                "Invalid coordinates"
            ),
            CommandResponse::InvalidState => write!(f, 
                "Invalid state"
            ),
            CommandResponse::InvalidCharge => write!(f, 
                "Invalid charge"
            ),
            CommandResponse::GetBot { 
                title, 
                coordinates, 
                state, 
                charge } => write!(f, 
                    "Bot info:\n
                    Title: {}\n
                    Coordinates: {}\n
                    State: {}\n
                    Charge: {}", 
                    title, coordinates, state, charge
                ),
            CommandResponse::UpdatedBot => write!(f, "Bot updated"),
            CommandResponse::BotList { bots } => {
                if bots.is_empty() {
                    let _ = write!(f, "Bot list empty");
                } else {
                    let _ = write!(f, "Bot list:\n");
                    for (id, title) in bots {
                        let _ = write!(f, "{}, {}\n", id, title);
                    }
                }
                Ok(())
            },
            CommandResponse::DeletedBot => write!(f, 
                "Bot deleted"
            ),
            CommandResponse::Shutdown => write!(f, 
                "Server shutting down"
            ),
        }
    }
}