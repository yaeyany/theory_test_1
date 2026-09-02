use std::sync::mpsc::channel;

use crate::{bots::BotStore, client::client, commands::Commands, server::server, workers::WorkQueue};

mod client;
mod commands;
mod errors;
mod bots;
mod server;
mod workers;

fn main() {
    let mut bots = BotStore::new();
    let mut work_queue = WorkQueue::new();
    let (sender, receiver) = channel::<Commands>();
    client(sender);
    server(receiver, work_queue);

}
