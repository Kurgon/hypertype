use hypertype::config::init_config;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    init_config().await?;
    Ok(())
}
