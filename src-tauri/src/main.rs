#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::path::PathBuf;
use std::time::Duration;
use std::process::Command;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use librqbit::{Session, AddTorrent, ManagedTorrent, SessionOptions, DhtSessionConfig, AddTorrentOptions};
use tauri::{AppHandle, Emitter};
use serde::Serialize;

static SESSION: Lazy<Mutex<Option<Arc<Session>>>> = Lazy::new(|| Mutex::new(None));
static TORRENTS: Lazy<Mutex<Vec<Arc<ManagedTorrent>>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Serialize, Clone)]
struct ProgressData {
    progress_percent: f64,
    downloaded_bytes: u64,
    total_bytes: u64,
    download_speed: u64,
    upload_speed: u64,
    peers: u32,
    state: String,
}

#[tauri::command]
async fn add_torrent(app: AppHandle, source: String) -> Result<String, String> {
    println!("[RUST] Recebi: {}", source);

    let session = {
        let mut session_lock = SESSION.lock().await;

        if session_lock.is_none() {
            println!("[RUST] Iniciando sessão BitTorrent (sem persistência de DHT)...");
            let download_path = PathBuf::from(r"C:\CineTorrentDownloads");

            std::fs::create_dir_all(&download_path)
                .map_err(|e| format!("Erro ao criar pasta: {}", e))?;

            let mut opts = SessionOptions::default();
            opts.dht = Some(DhtSessionConfig {
                persistence: None,
                ..Default::default()
            });

            let session = Session::new_with_opts(download_path, opts)
                .await
                .map_err(|e| format!("Erro ao criar sessão: {}", e))?;

            *session_lock = Some(session);
        }
        session_lock.as_ref().unwrap().clone()
    };

    let add_request = if source.starts_with("magnet:") {
        AddTorrent::from_url(source)
    } else {
        let file_bytes = std::fs::read(&source)
            .map_err(|e| format!("Erro ao ler arquivo torrent: {}", e))?;
        AddTorrent::from_bytes(file_bytes)
    };

    println!("[RUST] Adicionando torrent...");
    
    let add_options = AddTorrentOptions {
        overwrite: true,
        ..Default::default()
    };

    let add_response = session
        .add_torrent(add_request, Some(add_options))
        .await
        .map_err(|e| format!("Erro ao adicionar torrent: {}", e))?;

    let handle = add_response
        .into_handle()
        .ok_or_else(|| "Torrent listado, mas não iniciado para download.".to_string())?;

    TORRENTS.lock().await.push(handle.clone());
    println!("[RUST] Torrent adicionado e gerenciado!");

    let app_handle = app.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let stats = handle.stats();
            let percent = if stats.total_bytes > 0 {
                (stats.progress_bytes as f64 / stats.total_bytes as f64) * 100.0
            } else { 0.0 };

            let live_json = serde_json::to_value(&stats.live).unwrap_or(serde_json::Value::Null);
            let down_speed = live_json.get("download_speed").and_then(|v| v.as_u64()).unwrap_or(0);
            let up_speed = live_json.get("upload_speed").and_then(|v| v.as_u64()).unwrap_or(0);
            let peer_count = live_json.get("peers_connected").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

            let data = ProgressData {
                progress_percent: percent,
                downloaded_bytes: stats.progress_bytes,
                total_bytes: stats.total_bytes,
                download_speed: down_speed,
                upload_speed: up_speed,
                peers: peer_count,
                state: format!("{:?}", stats.state),
            };

            let _ = app_handle.emit("torrent-progress", &data);
        }
    });

    Ok("Torrent adicionado com sucesso!".to_string())
}

#[tauri::command]
async fn open_vlc(torrent_index: usize) -> Result<String, String> {
    let torrents = TORRENTS.lock().await;
    let handle = torrents.get(torrent_index).ok_or("Torrent não encontrado.")?;

    let torrent_name = handle.name().unwrap_or_else(|| "arquivo_desconhecido".to_string());
    let file_path = format!("C:\\CineTorrentDownloads\\{}", torrent_name);

    let vlc_path = r"C:\Program Files\VideoLAN\VLC\vlc.exe";

    if !std::path::Path::new(vlc_path).exists() {
        return Err("VLC não encontrado. Instale o VLC em: https://www.videolan.org/".to_string());
    }

    Command::new(vlc_path)
        .arg(&file_path)
        .spawn()
        .map_err(|e| format!("Erro ao abrir VLC: {}", e))?;

    Ok(format!("VLC aberto com: {}", file_path))
}

#[tauri::command]
async fn get_stream_url(torrent_index: usize, file_index: usize) -> Result<String, String> {
    let torrents = TORRENTS.lock().await;
    if torrents.get(torrent_index).is_some() {
        Ok(format!("http://localhost:3030/torrents/{}/stream/{}", torrent_index, file_index))
    } else {
        Err("Torrent não encontrado.".to_string())
    }
}

fn main() {
    println!("[RUST] Backend CineTorrent Next iniciado.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_torrent,
            open_vlc,
            get_stream_url
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar aplicação Tauri");
}