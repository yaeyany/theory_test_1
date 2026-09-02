use std::sync::mpsc::Receiver;

use crate::{commands::Commands, workers::WorkQueue};

pub fn server(receiver: Receiver<Commands>, mut work_queue: WorkQueue) {
    loop {
        match receiver.recv() {
            Ok(command) => match &command {
                Commands::Create { title, coordinates, state, charge, response_sender } => work_queue.add(command),
                Commands::Get { id, response_sender } => todo!(),
                Commands::Update { id, title, coordinates, state, charge, response_sender } => todo!(),
                Commands::List { response_sender } => todo!(),
                Commands::Delete { id, response_sender } => todo!(),
                Commands::Shutdown { response_sender } => todo!(),
            },
            Err(e) => todo!(),
        }
    }
}