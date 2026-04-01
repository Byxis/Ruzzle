use discord_rpc_client::Client;
use std::thread;

pub fn discord_rpc() -> Option<()> {
    thread::spawn(|| {
        let state_message = "Crab Rave";

        // Create the client
        let mut drpc = Client::new(1484565447046074459);

        // Register event handlers with the corresponding methods
        drpc.on_ready(|_ctx| {
            println!("ready?");
        });

        // Start up the client connection
        drpc.start();

        // Set the activity - silently ignore if it fails
        let _ = drpc.set_activity(|act| act.state(state_message));
    });

    None
}
