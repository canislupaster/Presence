extern crate discord_rpc_sdk;

use std::time::SystemTime;
use discord_rpc_sdk::{EventHandlers, JoinRequest, JoinRequestReply, RichPresence, RPC};

const APPLICATION_ID: &'static str = "378906438590005272";

struct Handlers;

impl EventHandlers for Handlers {
    fn ready() {
        println!("We're ready!");
    }

    fn errored(errcode: i32, message: &str) {
        println!("Error {}: {}", errcode, message);
    }

    fn disconnected(errcode: i32, message: &str) {
        println!("Disconnected {}: {}", errcode, message);
    }

    fn join_game(secret: &str) {
        println!("Joining {}", secret);
    }

    fn spectate_game(secret: &str) {
        println!("Spectating {}", secret);
    }

    fn join_request<R: FnOnce(JoinRequestReply)>(request: JoinRequest, respond: R) {
        println!("Join request from {:?}", request);
        respond(JoinRequestReply::Yes);
    }
}

fn main() {
    let rpc = RPC::init::<Handlers>(APPLICATION_ID, true, None).unwrap();

    let presence = RichPresence {
        details: Some("Details".to_string()),
        state: Some("State".to_string()),
        start_time: Some(SystemTime::now()),
        large_image_key: Some("large_image".to_string()),
        large_image_text: Some("large image".to_string()),
        small_image_key: Some("small_image".to_string()),
        small_image_text: Some("small image".to_string()),
        party_size: Some(4),
        party_max: Some(13),
        spectate_secret: Some("spectate".to_string()),
        join_secret: Some("join".to_string()),
        ..Default::default()
    };
    rpc.update_presence(presence).unwrap();

    loop {
        rpc.run_callbacks();
    }
}
