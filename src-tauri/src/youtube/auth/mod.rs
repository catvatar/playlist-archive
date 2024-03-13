use tauri::AppHandle;

fn youtube_api_key(app : AppHandle) -> Result<String, std::io::Error> {
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();

    let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
    std::fs::read_to_string(api_key_dir)
}

pub fn authenticate(app : AppHandle) -> Result<String, std::io::Error> {
    let api_key : String = youtube_api_key(app)?;
    Ok(api_key)
}