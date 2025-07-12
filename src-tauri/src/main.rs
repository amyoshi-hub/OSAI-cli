use tauri::{Builder};
pub mod server;
pub mod server_signal;
pub mod client;
pub mod file_copy;
pub mod websocket;
pub mod file_server;
pub mod file_read;
pub mod http_server;

use server::start_server;
use client::send_text;
use file_copy::{process_and_add_world};
use file_copy::{get_world_list, open_world};
use websocket::start_websocket_server;
use file_server::get_file_list;
use file_read::read_file_content;
use http_server::{http_server, open_url_window, fetch_file_list, request_file};
use http_server::FileList;
mod AI;
mod format_handler;


#[tokio::main]
async fn main() {

    println!("App start");

    Builder::default()
        // generate_handler!に複数のコマンドを渡せます
        .invoke_handler(tauri::generate_handler![
            start_server,
            send_text,
            process_and_add_world,
            get_world_list,
//            read_text_file,
            open_world,
            start_websocket_server,
            get_file_list,
            read_file_content,
            http_server,
            open_url_window,
            fetch_file_list,
            request_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
