DIAGNOSTICO CINE TORRENT NEXT
Data: 11/05/2026 10:42:16
Pasta: C:\Users\DANIEL T.I\Desktop\CineTorrent-Next
Total de arquivos: 37

===== LISTA DE ARQUIVOS =====
0 bytes | .env.example
0 bytes | .gitignore
3205 bytes | app.vue
12939 bytes | DIAGNOSTICO_CINETORRENT.txt
10449 bytes | icon.png
3432 bytes | nao_usar.txt
241 bytes | nuxt.config.ts
373720 bytes | package-lock.json
319 bytes | package.json
0 bytes | README.md
47 bytes | tsconfig.json
2727 bytes | app\app.vue
0 bytes | app\pages\downloads.vue
0 bytes | app\pages\index.vue
0 bytes | app\pages\library.vue
0 bytes | app\pages\settings.vue
0 bytes | app\stores\player.ts
0 bytes | app\stores\torrents.ts
0 bytes | app\types\player.ts
0 bytes | app\types\torrent.ts
0 bytes | docs\architecture\ARCHITECTURE.md
0 bytes | docs\development\ROADMAP.md
45 bytes | src-tauri\build.rs
145188 bytes | src-tauri\Cargo.lock
468 bytes | src-tauri\Cargo.toml
672 bytes | src-tauri\tauri.conf.json
66720 bytes | src-tauri\gen\schemas\acl-manifests.json
2 bytes | src-tauri\gen\schemas\capabilities.json
117263 bytes | src-tauri\gen\schemas\desktop-schema.json
117263 bytes | src-tauri\gen\schemas\windows-schema.json
126 bytes | src-tauri\icons\icon.ico
0 bytes | src-tauri\src\lib.rs
2585 bytes | src-tauri\src\main.rs
0 bytes | src-tauri\src\commands\mod.rs
0 bytes | src-tauri\src\player\mod.rs
0 bytes | src-tauri\src\storage\mod.rs
0 bytes | src-tauri\src\torrent\mod.rs

===== ARQUIVOS VAZIOS =====
.env.example
.gitignore
README.md
app\pages\downloads.vue
app\pages\index.vue
app\pages\library.vue
app\pages\settings.vue
app\stores\player.ts
app\stores\torrents.ts
app\types\player.ts
app\types\torrent.ts
docs\architecture\ARCHITECTURE.md
docs\development\ROADMAP.md
src-tauri\src\lib.rs
src-tauri\src\commands\mod.rs
src-tauri\src\player\mod.rs
src-tauri\src\storage\mod.rs
src-tauri\src\torrent\mod.rs

===== CONTEUDO DOS ARQUIVOS PRINCIPAIS =====

===== package.json =====
{
  "name": "cinetorrent-next",
  "private": true,
  "scripts": {
    "dev": "nuxt dev",
    "build": "nuxt build",
    "tauri": "tauri"
  },
  "dependencies": {
    "nuxt": "^3.10.0",
    "vue": "^3.4.0",
    "@tauri-apps/api": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0"
  }
}

===== nuxt.config.ts =====

export default defineNuxtConfig({
  compatibilityDate: '2026-11-05',

  vite: {
    server: {
      watch: {
        ignored: [
          '**/src-tauri/**',
          '**/target/**',
          '**/.git/**'
        ]
      }
    }
  }
})


===== app.vue =====
<template>
  <div class="app-container">
    <header class="header">
      <h1>ðŸŽ¬ CineTorrent <span>Next</span></h1>
    </header>

    <main class="main-content">
      <div 
        class="drop-zone" 
        :class="{ 'drop-zone-active': isDragging }"
        @dragover.prevent="isDragging = true"
        @dragleave.prevent="isDragging = false"
        @drop.prevent="handleDrop"
      >
        <input 
          type="text" 
          placeholder="Cole um link Magnet ou arraste um arquivo .torrent aqui..." 
          v-model="magnetLink"
          @keyup.enter="addTorrent(magnetLink)"
        />
        <button @click="addTorrent(magnetLink)">Adicionar</button>
      </div>

      <div class="downloads-panel">
        <h2>Downloads Ativos</h2>
        <div class="empty-state">
          Nenhum download ativo no momento.
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const magnetLink = ref('')
const isDragging = ref(false)

const addTorrent = async (source) => {
  if (!source) return
  try {
    console.log("Enviando para o Rust:", source)
    const resposta = await invoke('add_torrent', { source: source })
    alert(resposta)
    magnetLink.value = ''
  } catch (erro) {
    console.error(erro)
    alert('Erro no backend: ' + erro)
  }
}

const handleDrop = (event) => {
  isDragging.value = false
  const files = event.dataTransfer.files
  
  if (files.length > 0) {
    const file = files[0]
    const path = file.path || file.name 
    
    if (path.endsWith('.torrent')) {
      addTorrent(path)
    } else {
      alert('Por favor, arraste apenas arquivos .torrent')
    }
  }
}
</script>

<style>
* { box-sizing: border-box; margin: 0; padding: 0; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
body { background-color: #121212; color: #ffffff; }
.app-container { display: flex; flex-direction: column; height: 100vh; }
.header { padding: 20px 30px; background-color: #1e1e1e; border-bottom: 1px solid #333; }
.header h1 { font-size: 24px; font-weight: 600; }
.header h1 span { color: #4CAF50; }
.main-content { padding: 30px; display: flex; flex-direction: column; gap: 30px; max-width: 900px; margin: 0 auto; width: 100%; }

.drop-zone { display: flex; gap: 10px; padding: 20px; border: 2px dashed #444; border-radius: 8px; transition: all 0.3s; }
.drop-zone-active { border-color: #4CAF50; background-color: rgba(76, 175, 80, 0.1); }
.drop-zone input { flex: 1; padding: 12px 15px; border-radius: 6px; border: 1px solid #333; background-color: #1e1e1e; color: #fff; font-size: 16px; outline: none; }
.drop-zone input:focus { border-color: #4CAF50; }
.drop-zone button { padding: 12px 25px; border-radius: 6px; border: none; background-color: #4CAF50; color: white; font-size: 16px; font-weight: bold; cursor: pointer; }
.drop-zone button:hover { background-color: #45a049; }

.downloads-panel { background-color: #1e1e1e; border-radius: 8px; padding: 20px; border: 1px solid #333; }
.downloads-panel h2 { font-size: 18px; margin-bottom: 15px; color: #ccc; }
.empty-state { color: #666; text-align: center; padding: 40px 0; font-style: italic; }
</style>

===== app\app.vue =====
@"
<template>
  <div class="app-container">
    <header class="header">
      <h1>ðŸŽ¬ CineTorrent <span>Next</span></h1>
    </header>

    <main class="main-content">
      <div class="input-group">
        <input 
          type="text" 
          placeholder="Cole um link Magnet ou o caminho de um arquivo .torrent aqui..." 
          v-model="magnetLink"
          @keyup.enter="addTorrent"
        />
        <button @click="addTorrent">Adicionar</button>
      </div>

      <div class="downloads-panel">
        <h2>Downloads Ativos</h2>
        <div class="empty-state">
          Nenhum download ativo no momento.
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const magnetLink = ref('')

const addTorrent = async () => {
  if (!magnetLink.value) return

  try {
    // Envia o comando para o Rust
    const resposta = await invoke('add_torrent', { 
      magnetLink: magnetLink.value 
    })
    
    alert(resposta)
    magnetLink.value = ''
  } catch (erro) {
    console.error(erro)
    alert('Erro ao adicionar torrent: ' + erro)
  }
}
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

body {
  background-color: #121212;
  color: #ffffff;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  padding: 20px 30px;
  background-color: #1e1e1e;
  border-bottom: 1px solid #333;
}

.header h1 {
  font-size: 24px;
  font-weight: 600;
}

.header h1 span {
  color: #4CAF50;
}

.main-content {
  padding: 30px;
  display: flex;
  flex-direction: column;
  gap: 30px;
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.input-group {
  display: flex;
  gap: 10px;
}

.input-group input {
  flex: 1;
  padding: 12px 15px;
  border-radius: 6px;
  border: 1px solid #333;
  background-color: #1e1e1e;
  color: #fff;
  font-size: 16px;
  outline: none;
  transition: border-color 0.2s;
}

.input-group input:focus {
  border-color: #4CAF50;
}

.input-group button {
  padding: 12px 25px;
  border-radius: 6px;
  border: none;
  background-color: #4CAF50;
  color: white;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: background-color 0.2s;
}

.input-group button:hover {
  background-color: #45a049;
}

.downloads-panel {
  background-color: #1e1e1e;
  border-radius: 8px;
  padding: 20px;
  border: 1px solid #333;
}

.downloads-panel h2 {
  font-size: 18px;
  margin-bottom: 15px;
  color: #ccc;
}

.empty-state {
  color: #666;
  text-align: center;
  padding: 40px 0;
  font-style: italic;
}
</style>
"@ | Out-File -FilePath "app.vue" -Encoding UTF8

===== app\pages\index.vue =====
[ARQUIVO VAZIO]

===== app\pages\downloads.vue =====
[ARQUIVO VAZIO]

===== app\pages\library.vue =====
[ARQUIVO VAZIO]

===== app\pages\settings.vue =====
[ARQUIVO VAZIO]

===== app\stores\torrents.ts =====
[ARQUIVO VAZIO]

===== app\stores\player.ts =====
[ARQUIVO VAZIO]

===== app\types\torrent.ts =====
[ARQUIVO VAZIO]

===== src-tauri\Cargo.toml =====
[package]
name = "cinetorrent-next"
version = "0.1.0"
description = "CineTorrent Next"
authors = ["Daniel"]
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
librqbit = "9.1.0"
tokio = { version = "1.0", features = ["full"] }
once_cell = "1.19"

[features]
custom-protocol = ["tauri/custom-protocol"]


===== src-tauri\tauri.conf.json =====
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "CineTorrent Next",
  "version": "0.1.0",
  "identifier": "com.cinetorrent.next",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:3000",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../.output/public"
  },
  "app": {
    "windows": [
      {
        "title": "CineTorrent Next",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": []
  }
}

===== src-tauri\src\main.rs =====

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use librqbit::{Session, AddTorrent, ManagedTorrent};

// Sessão global do BitTorrent
static SESSION: Lazy<Mutex<Option<Arc<Session>>>> =
    Lazy::new(|| Mutex::new(None));

// Referências aos torrents gerenciados
static TORRENTS: Lazy<Mutex<Vec<Arc<ManagedTorrent>>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

#[tauri::command]
async fn add_torrent(source: String) -> Result<String, String> {

    println!("[RUST] Recebi: {}", source);

    // Inicializa ou recupera a sessão
    let session = {
        let mut session_lock = SESSION.lock().await;

        if session_lock.is_none() {
            println!("[RUST] Iniciando sessão BitTorrent...");

            let download_path =
                PathBuf::from(r"C:\CineTorrentDownloads");

            std::fs::create_dir_all(&download_path)
                .map_err(|e| format!("Erro ao criar pasta: {}", e))?;

            let session = Session::new(download_path)
                .await
                .map_err(|e| format!("Erro ao criar sessão: {}", e))?;

            *session_lock = Some(session);
        }

        session_lock.as_ref().unwrap().clone()
    };

    // Prepara a origem do torrent
    let add_request = if source.starts_with("magnet:") {
        AddTorrent::from_url(source)
    } else {
        let file_bytes = std::fs::read(&source)
            .map_err(|e| format!("Erro ao ler arquivo torrent: {}", e))?;

        AddTorrent::from_bytes(file_bytes)
    };

    println!("[RUST] Adicionando torrent...");

    // Adiciona o torrent à sessão
    let add_response = session
        .add_torrent(add_request, None)
        .await
        .map_err(|e| format!("Erro ao adicionar torrent: {}", e))?;

    // Extrai Arc<ManagedTorrent> da resposta
    let handle = add_response
        .into_handle()
        .ok_or_else(|| {
            "Torrent listado, mas não iniciado para download.".to_string()
        })?;

    // Mantém uma referência ao torrent
    TORRENTS.lock().await.push(handle);

    println!("[RUST] Torrent adicionado e gerenciado!");

    Ok("Torrent adicionado com sucesso!".to_string())
}

fn main() {

    println!("[RUST] Backend CineTorrent Next iniciado.");

    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                add_torrent
            ]
        )
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar aplicação Tauri");
}


===== src-tauri\src\lib.rs =====
[ARQUIVO VAZIO]

===== src-tauri\src\commands\mod.rs =====
[ARQUIVO VAZIO]

===== src-tauri\src\torrent\mod.rs =====
[ARQUIVO VAZIO]
