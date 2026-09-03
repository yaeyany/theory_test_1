use std::{fmt::Result, sync::mpsc::{Receiver, Sender, channel}};

use crate::{commands::{CommandResponse, Commands}, errors::{CustomErrors, handle_error}};


pub fn client(command: Commands, sender: Sender<(Commands, Sender<CommandResponse>)>) {
    let (response_sender, response_receiver) = channel::<CommandResponse>();
    match sender.send((command, response_sender)) {
        Ok(_) => match_response(response_receiver),
        Err(_) => handle_error(CustomErrors::ServerUnavailable),
    }
}

pub fn match_response(response_receiver: Receiver<CommandResponse>) {
    match response_receiver.recv() {
        Ok(response) => println!("{}", response),
        Err(_) => handle_error(CustomErrors::ServerUnavailable),
    }
}