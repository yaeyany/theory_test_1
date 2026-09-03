use std::sync::{Arc, Mutex, mpsc::{Sender, channel}};

use crate::{bots::BotStore, commands::{CommandResponse, Commands}, server::server};

mod client;
mod commands;
mod errors;
mod bots;
mod server;
mod workers;

fn main() {
    let mut bots = BotStore::new();
    let (sender, receiver) = channel::<(Commands, Sender<CommandResponse>)>();
    let (work_sender, work_receiver) = channel::<(Commands, Sender<CommandResponse>)>();
    let work_receiver_mutex = Arc::new(Mutex::new(work_receiver));
    server(receiver, work_sender);


}
