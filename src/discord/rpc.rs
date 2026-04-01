use discord_rpc_client::{Client};

pub fn discord_rpc() -> Client {
    let state_message = "Crab Rave";

    // Create the client
    let mut drpc = Client::new(1484565447046074459);

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| act.state(state_message))
        .expect("Failed to set activity");

    return drpc;
}
