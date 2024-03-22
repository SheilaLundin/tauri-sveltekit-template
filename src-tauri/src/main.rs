// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn command_name(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
  Ok(())
}

fn main() {
    let format =
        time::format_description::parse("[[[year]-[month]-[day] [hour]:[minute]:[second]]")
            .unwrap();

    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(tracing::Level::DEBUG)
                .with_file(true)
                .with_line_number(true)
                .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
                    time::macros::offset!(+8),
                    format,
                ))
                .init();
        } else {
            let dir = "logs";
            let file = "{{project-name}}.log";
            let p = std::path::Path::new(dir).join(file);
            if p.exists() {
                let _ = std::fs::remove_file(p);
            }
            let file_appender = tracing_appender::rolling::never(dir, file);
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(tracing::Level::INFO)
                .with_ansi(false)
                .with_file(true)
                .with_line_number(true)
                .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
                    time::macros::offset!(+8),
                    format,
                ))
                .with_writer(non_blocking)
                .init();
        }
    }

    let _ = color_eyre::install();

    let instance = single_instance::SingleInstance::new("{{project-name}}").unwrap();
    if !instance.is_single() {
        tracing::warn!("程序已运行");
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
