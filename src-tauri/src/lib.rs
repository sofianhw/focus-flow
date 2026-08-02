// Tauri setup for the offline, embedded CSR desktop application.

// Mobile entry point - required for Android/iOS
#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn mobile_main() {
    run();
}

pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
