use std::sync::mpsc::{Receiver, Sender};

use crate::{commands::{CommandResponse, Commands}, errors::{CustomErrors, handle_error}};

pub fn server(receiver: Receiver<(Commands, Sender<CommandResponse>)>, work_sender: Sender<(Commands, Sender<CommandResponse>)>) {
    loop {
        match receiver.recv() {
            Ok((command, response_sender)) => match &command {
                Commands::Shutdown => todo!(),
                _ => work_sender.send((command, response_sender)).map_err(|_|handle_error(CustomErrors::WorkersUnavailable))
            },
            Err(e) => todo!(),
        };
    }
}