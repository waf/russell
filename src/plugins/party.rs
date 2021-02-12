// triggers the element.io client party animation

use super::Plugin;
use matrix_sdk::{
    Client, RoomState, async_trait
};

pub struct PartyPlugin { }

#[async_trait]
impl Plugin for PartyPlugin {
    async fn room_message(&self, client: &Client, room: &RoomState, msg_body: &str) {
        if msg_body == ".party" {
            let content = "🎉🎊🥳 let's PARTY!! 🥳🎊🎉";
            self.send_message(client, room, content).await;
        }
    }
}