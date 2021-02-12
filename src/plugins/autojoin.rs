// autojoin room on invitation.
// basically all this code is from the autojoin example: https://github.com/matrix-org/matrix-rust-sdk/tree/master/matrix_sdk/examples

use super::Plugin;
use tokio::time::{sleep, Duration};
use matrix_sdk::{
    self,
    events::{
        StrippedStateEvent,
        room::member::{MemberEventContent},
    },
    Client, RoomState, async_trait
};

pub struct AutoJoinPlugin { }

#[async_trait]
impl Plugin for AutoJoinPlugin {
    async fn raw_stripped_state_member(
        &self,
        client: &Client,
        room: &RoomState,
        _: &StrippedStateEvent<MemberEventContent>,
        _: &Option<MemberEventContent>,
    ) {
        if let RoomState::Invited(room) = room {
            let room_id = room.room_id();
            println!("Autojoining room {}", room_id);
            let mut delay = 2;

            while let Err(err) = client.join_room_by_id(&room_id).await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                eprintln!(
                    "Failed to join room {} ({:?}), retrying in {}s",
                    room_id, err, delay
                );

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    eprintln!("Can't join room {} ({:?})", room_id, err);
                    break;
                }
            }
            println!("Successfully joined room {}", room_id);
        }
    }
}