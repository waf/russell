mod bot;
mod config;
mod plugins;

use bot::Bot;
use config::{BotConfig};
use anyhow::{Context, Result};
use matrix_sdk::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<()> {

    let config_file = "./bot_config.toml";
    let config = config::get_configuration(config_file)?;

    let mut bot = init_bot(&config)?;

    match config.auth_token {
        // auth token. use it to log in.
        Some(token) => bot.restore_session(&token).await?,
        // no auth token. prompt for password, trade it for token, then save token to config file for future runs.
        None => {
            let password = password_prompt()?;
            let token = bot.create_new_session(&config.user_name, &password, &config.display_name).await?;
            config::save_token_configuration(config_file, &BotConfig { auth_token: Some(token), ..config })?;
        }
    }

    bot.wait_for_exit().await?;

    Ok(())
}

fn init_bot(config: &BotConfig) -> Result<Bot> {
    let matrix_api_client_config = ClientConfig::new().store_path(&config.state_directory);
    let matrix_api_client = Client::new_with_config(config.home_server.as_ref(), matrix_api_client_config)?;
    let bot = Bot::new(matrix_api_client);
    Ok(bot)
}

fn password_prompt() -> Result<String> {
    loop {
        let password = rpassword::read_password_from_tty(Some("Password: ")).context("Could not prompt for password in terminal")?;
        if password.len() > 0 {
            return Ok(password);
        }
    }
}