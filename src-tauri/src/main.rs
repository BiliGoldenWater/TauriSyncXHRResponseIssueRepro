#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .register_uri_scheme_protocol("testUriScheme", |_app_handle, _req| {
      tauri::http::ResponseBuilder::new()
        .header("Access-Control-Allow-Origin", "*")
        .body("test_long_body".to_string().as_bytes().to_vec())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
