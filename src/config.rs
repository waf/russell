use serde::{Serialize, Deserialize};
use toml;
use url::Url;
use std::{io, fs};
use anyhow::{Context, Result, anyhow};
use matrix_sdk::{
    identifiers::{UserId, DeviceId},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub home_server: Url,
    pub user_name: String,
    pub display_name: String,
    pub state_directory: String,
    pub auth_token: Option<TokenAuthentication>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAuthentication {
    pub user_id: UserId,
    pub device_id: Box<DeviceId>,
    pub access_token: String,
}

pub fn get_configuration(filename: &str) -> Result<BotConfig> {
    let contents = fs::read_to_string(filename);

    match contents {
        Err(err) => {
            generate_new_config_file(err, filename).context("Failed to generate new configuration file")?;
            Err(anyhow!("Configuration file not found. New configuration file generated at {}", &filename))
        }
        Ok(toml) => {
            let config : BotConfig = toml::from_str(&toml)
                .context("Unrecognized configuration file format")?;

            Ok(config)
        }
    }
}

pub fn save_token_configuration(filename: &str, token: &BotConfig) -> Result<()> {
    
    println!("Saving auth token to configuration file, future logins will not require a password.");
    let contents = toml::to_string_pretty(token).context("Unable to serialize token configuration")?;
    fs::write(filename, contents).context(format!("Unable to write configuration to file: {}", filename))?;

    Ok(())
}


fn generate_new_config_file(error: std::io::Error, filename: &str) -> Result<()> {
    println!("Unable to read configuration file");
    println!("Error: {}", error);
    println!("Generate a new, empty configuration file? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Could not retrieve user input")?;

    if !input.contains("y") {
        return Err(anyhow!("User cancelled"));
    }
    let default_config = BotConfig {
        home_server: Url::parse("https://matrix.org/").unwrap(),
        state_directory: "./target/bot_state/".to_owned(),
        user_name: "russell_bot".to_owned(),
        display_name: "russell".to_owned(),
        auth_token: Option::None
    };

    let serialized = toml::to_string(&default_config).context("Could not serialize default configuration to file")?;
    fs::write(&filename, &serialized).context("Error writing default configuration to file")?;

    println!("Configuration file created: {}", filename);
    println!("Modify this configuration file as required and then run this program again.");

    Ok(())
}