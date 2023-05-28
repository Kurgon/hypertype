use dialoguer::{theme::ColorfulTheme, Password, Select};
use log::info;
use regex::Regex;
use std::fs;
use std::{path::PathBuf, str::FromStr};

pub async fn get_token() -> Result<String, ()> {
    Ok("placeholder_token".to_string())
}

fn init_config_files() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing config...");
    if !get_config_dir().exists() {
        info!("Config folder not found, creating...");
        fs::create_dir(get_config_dir())?;
    }
    if !get_config_file().exists() {
        info!("Config file not found, creating...");
        fs::File::create(get_config_file())?;
    }
    Ok(())
}

pub async fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    init_config_files()?;

    // Regex is slow, so build regex in background to increase speed of application.

    let re_thread = tokio::spawn(async {
        return Regex::new(r"/[\w-]{24}\.[\w-]{6}\.[\w-]{27}/").unwrap();
    });

    let re_mfa_thread = tokio::spawn(async { Regex::new(r"/mfa\.[\w-]{84}/").unwrap() });

    let auto_get_options = &[
        "Automatically get Discord token",
        "Supply your own Discord token",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Discord token")
        .items(&auto_get_options[..])
        .default(0)
        .interact()
        .unwrap();

    if selection == 0 {
        get_token().await.unwrap();
    } else {
        let re = re_thread.await.unwrap();
        let re_mfa = re_mfa_thread.await.unwrap();
        let token = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Discord token:")
            .validate_with(|input: &String| -> Result<(), &str> {
                info!("Token validity check...");
                if re.is_match(input) && re_mfa.is_match(input) {
                    info!("Valid token! Proceeding...");
                    Ok(())
                } else {
                    Err("Token invalid.")
                }
            })
            .interact()
            .unwrap();
        println!("{}", token);
    }
    Ok(())
}

pub fn get_config_dir() -> PathBuf {
    let mut root_config_dir = dirs::config_dir()
        .expect("Failed to get user config directory.")
        // This hairball of code converts the PathBuf to a string
        .into_os_string()
        .into_string()
        .unwrap();
    root_config_dir.push_str("/hypertype");
    return PathBuf::from_str(&root_config_dir).unwrap(); // And then turns it back into a PathBuf
}
pub fn get_config_file() -> PathBuf {
    let mut root_config_dir = get_config_dir().into_os_string().into_string().unwrap(); // hooray!
    root_config_dir.push_str("/config.yml");
    return PathBuf::from_str(&root_config_dir).unwrap();
}
