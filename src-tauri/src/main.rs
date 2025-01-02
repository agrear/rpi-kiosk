// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tauri::{Manager, PhysicalPosition, Position};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(long, default_value_t = false)]
    hide_cursor: bool
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let args = Args::parse();

            let window = app.get_window("main").unwrap();
            window.eval(&format!("window.location.replace('{0}')", args.url))?;

            //window.set_cursor_visible(args.hide_cursor).unwrap();
            // Workaround for https://github.com/tauri-apps/tauri/issues/10231
            if args.hide_cursor {
                let position = Position::Physical(PhysicalPosition{ x: 9999, y: 9999 });
                window.set_cursor_position(position).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
