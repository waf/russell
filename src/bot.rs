use matrix_sdk::{Client, SyncSettings, Session};
use anyhow::{Context, Result};
use super::{
    plugins::EventForwarder,
    config::TokenAuthentication
};

pub struct Bot {
    client: Client // matrix api client
}

impl Bot {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_new_session(&mut self, user_name: &str, password: &str, display_name: &str) -> Result<TokenAuthentication> {

        let response = self.client
            .login(user_name, password, None, Some(display_name))
            .await?;

        println!("logged in as {}", user_name);

        self.install_plugins().await?;

        Ok(TokenAuthentication {
            user_id: response.user_id,
            device_id: response.device_id,
            access_token: response.access_token
        })
    }

    pub async fn restore_session(&mut self, token: &TokenAuthentication) -> Result<()> {
        let session = Session {
            access_token: token.access_token.clone(),
            user_id: token.user_id.clone(),
            device_id: token.device_id.clone(),
        };
        self.client.restore_login(session).await?;
        self.install_plugins().await?;

        Ok(())
    }

    pub async fn install_plugins(&self) -> Result<()> {
        // An initial sync to set up state and so our bot doesn't respond to old messages.
        // If the `StateStore` finds saved state in the location given the initial sync will
        // be skipped in favor of loading state from the store
        self.client.sync_once(SyncSettings::default()).await.context("Error synchronizing messages with server")?;

        // add our plugins to be notified of incoming messages, we do this after the initial
        // sync to avoid responding to messages from before the bot was running.
        let emitter = EventForwarder::new(self.client.clone());
        self.client.add_event_emitter(Box::new(emitter)).await;

        Ok(())
    }

    pub async fn wait_for_exit(&self) -> Result<()> {
        // since we called `sync_once` before we entered our sync loop we must pass that sync token to `sync`
        let previous_sync = self.client.sync_token().await.context("Could not retrieve token from previous synchronization")?;

        // this call asynchronously blocks, keeping the state from the server streaming into the bot via the EventEmitter trait
        self.client.sync(SyncSettings::default().token(previous_sync)).await;
        Ok(())
    }
}