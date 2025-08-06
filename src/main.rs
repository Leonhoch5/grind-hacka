use std::fs;
use std::path::PathBuf;
use configparser::ini::Ini;


#[tokio::main]
async fn main() {
    match get_wakatime_config() {
        Some((api_key, api_url, path, _contents)) => {
            println!("Found config at: {}", path);
            println!("Wakatime API Key: {}", api_key);
            println!("Wakatime API URL: {}", api_url);

            // Build the correct endpoint for today's time
            let today_url = format!("{}/users/current/statusbar/today", api_url.trim_end_matches('/'));

            match fetch_wakatime_data(&today_url, &api_key).await {
                Ok(body) => println!("API response:\n{}", body),
                Err(e) => eprintln!("API request failed: {}", e),
            }
        }
        None => println!("Wakatime config or keys not found."),
    }
}


fn get_wakatime_config() -> Option<(String, String, String, String)> {
    let home_dir = dirs::home_dir()?;
    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".wakatime.cfg");

    println!("Searching for config at: {}", config_path.display()); // Show the path

    let mut contents = fs::read_to_string(&config_path).ok()?;
    // Remove BOM if present, no idea why it exists but it does
    // This is a workaround for some UTF-8 files that start with a BOM
    if contents.starts_with('\u{feff}') {
        contents = contents.trim_start_matches('\u{feff}').to_string();
    }

    let mut config = Ini::new();
    config.read(contents.clone()).ok()?;

    let api_key = config.get("settings", "api_key")?;
    let api_url = config.get("settings", "api_url")?;
    Some((api_key, api_url, config_path.display().to_string(), contents))
}


async fn fetch_wakatime_data(api_url: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    res.text().await
}