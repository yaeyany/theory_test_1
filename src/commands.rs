use std::sync::mpsc::Sender;

use crate::bots::{BotCharge, BotCoordinates, BotId, BotState, BotTitle};

pub enum Commands {
    Create {
        title: BotTitle,
        coordinates: BotCoordinates,
        state: BotState,
        charge: BotCharge,
        response_sender: Sender<CommandResponse>
    },
    Get {
        id: BotId,
        response_sender: Sender<CommandResponse>
    },
    Update {
        id: BotId,
        title: Option<BotTitle>,
        coordinates: Option<BotCoordinates>,
        state: Option<BotState>,
        charge: Option<BotCharge>,
        response_sender: Sender<CommandResponse>
    },
    List {
        response_sender: Sender<CommandResponse>
    },
    Delete {
        id: BotId,
        response_sender: Sender<CommandResponse>
    },
    Shutdown {
        response_sender: Sender<CommandResponse>
    }
}

pub enum CommandResponse {
    UnknownCommand,
    CreatedBot,
    InvalidTItle,
    InvalidId,
    InvalidCoordinates,
    InvalidState,
    InvalidCharge,
    GetBot,
    UpdatedBot,
    BotList,
    DeletedBot,
    Shutdown
}