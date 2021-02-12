/**
 * Plugin Registration
 */

mod autojoin;
mod bacronym;
mod party;
mod woop;

fn register_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(autojoin::AutoJoinPlugin {}),
        Box::new(bacronym::BacronymPlugin::new()),
        Box::new(party::PartyPlugin {}),
        Box::new(woop::WoopPlugin {}),
    ]
}

/**
 * Plugin infrastructure (e.g. definition of the Plugin trait) 
 */

use matrix_sdk::{
    async_trait,
    events::{
        AnyMessageEventContent, StrippedStateEvent, SyncMessageEvent,
        room::member::MemberEventContent,
        room::message::{MessageEventContent, TextMessageEventContent},
    },
    Client, EventEmitter, RoomState
};

/// Plugins can implement this trait and receive callbacks for various matrix events.
/// Right now it exposes a subset of the callbacks from EventEmitter (https://docs.rs/matrix-sdk/0.2.0/matrix_sdk/trait.EventEmitter.html)
#[async_trait]
pub trait Plugin : Send + Sync {
    // raw functions, directly exposed from EventEmitter
    async fn raw_room_message(&self, _: &Client, _: &RoomState, _: &SyncMessageEvent<MessageEventContent>) {}
    async fn raw_stripped_state_member(&self, _: &Client, _: &RoomState, _: &StrippedStateEvent<MemberEventContent>, _: &Option<MemberEventContent>) {}

    // helper functions that provide a higher-level API for plugins
    async fn room_message(&self, _: &Client, _: &RoomState, _: &str) {}
    async fn send_message(&self, client: &Client, room: &RoomState, message: &str) {
        if message.trim().is_empty() {
            return;
        }

        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
            message
        ));
        if let RoomState::Joined(room) = room {
            let room_id = room.room_id();

            let response = client.room_send(&room_id, content, None).await;
            if let Err(err) = response {
                println!("Warning: failed to send message. {}", err);
            }
        }
    }
}

// The matrix-sdk only allows for a single EventEmitter to be registered, but we want to
// support multiple plugins. This EventForwarder simply forwards the callbacks to all plugins.
pub struct EventForwarder {
    client: Client,
    plugins: Vec<Box<dyn Plugin>>
}

impl EventForwarder {
    pub fn new(client: Client) -> Self {
        Self { client, plugins: register_plugins() }
    }
}

#[async_trait]
impl EventEmitter for EventForwarder {
    async fn on_room_message(&self, room: RoomState, event: &SyncMessageEvent<MessageEventContent>) {
        for plugin in self.plugins.iter() {
            plugin.raw_room_message(&self.client, &room, event).await;
        }
        if let RoomState::Joined(_) = room {
            let msg_body = if let SyncMessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent { body: msg_body, .. }),
                ..
            } = event
            {
                msg_body.clone()
            } else {
                String::new()
            };

            for plugin in self.plugins.iter() {
                plugin.room_message(&self.client, &room, &msg_body).await;
            }
        }
    }
    async fn on_stripped_state_member(&self, room: RoomState, room_member: &StrippedStateEvent<MemberEventContent>, member_event: Option<MemberEventContent>) {
        if let Some(user_id) = self.client.user_id().await {
            if room_member.state_key != user_id {
                return;
            }

            for plugin in self.plugins.iter() {
                plugin.raw_stripped_state_member(&self.client, &room, room_member, &member_event).await;
            }
        }
    }
}