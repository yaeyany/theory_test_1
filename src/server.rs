use std::sync::mpsc::Receiver;

use crate::{commands::Commands, jobs::JobStore};


pub fn server(receiver: Receiver<Commands>, jobs: &mut JobStore) {
    loop {
        match receiver.recv() {
            Ok(command) => match command {
                Commands::Create { title, response_sender } => {todo!()
                },
                Commands::Get { id, response_sender } => todo!(),
                Commands::Update { id, title, state, response_sender } => todo!(),
                Commands::List { response_sender } => todo!(),
                Commands::Delete { id, response_sender } => todo!(),
                Commands::Shutdown { response_sender } => todo!(),
                            },
            Err(e) => todo!(),
        }
    }
}