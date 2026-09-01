use std::sync::mpsc::channel;

use crate::{client::client, commands::Commands, jobs::JobStore, server::server};


mod client;
mod commands;
mod errors;
mod jobs;
mod server;

fn main() {
    let mut jobs = JobStore::new();
    let (sender, receiver) = channel::<Commands>();
    client(sender);
    server(receiver, &mut jobs);
}
