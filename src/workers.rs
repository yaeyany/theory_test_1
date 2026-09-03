use std::sync::{Arc, Mutex, mpsc::Sender};

use crate::commands::{CommandResponse, Commands};

pub fn worker(work_receiver_mutex: Arc<Mutex<std::sync::mpsc::Receiver<(Commands, Sender<CommandResponse>)>>>) {
    loop {
        let (command, response_sender)  = {
            match work_receiver_mutex.lock() {
                Ok(receiver) => match receiver.recv() {
                    Ok(work) => work,
                    Err(_) => todo!(),
                },
                Err(_) => todo!(),
            }
        };

        match command {
            Commands::Create { 
                title, 
                coordinates, 
                state, 
                charge } => {

            }
            Commands::Get { 
                id } => todo!(),
            Commands::Update { 
                id, 
                title, 
                coordinates, 
                state, 
                charge } => todo!(),
            Commands::List => todo!(),
            Commands::Delete { 
                id } => todo!(),
            Commands::Shutdown => todo!(),
        }
    }
}

