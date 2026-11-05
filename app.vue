<template>
  <div class="app-container">
    <header class="header">
      <h1>🎬 CineTorrent <span>Next</span></h1>
    </header>

    <main class="main-content">
      <div class="input-group">
        <input 
          type="text" 
          placeholder="Cole um link Magnet ou arraste um arquivo .torrent..." 
          v-model="magnetLink"
          @keyup.enter="addTorrent(magnetLink)"
        />
        <button @click="addTorrent(magnetLink)">Adicionar</button>
      </div>

      <div class="downloads-panel">
        <h2>Downloads Ativos</h2>
        
        <div v-if="progress" class="progress-container">
          <div class="progress-header">
            <span>Progresso: {{ progress.progress_percent.toFixed(2) }}%</span>
            <span>{{ formatBytes(progress.downloaded_bytes) }} / {{ formatBytes(progress.total_bytes) }}</span>
          </div>
          
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progress.progress_percent + '%' }"></div>
          </div>
          
          <div class="stats-info">
            <span>⬇️ {{ formatBytes(progress.download_speed) }}/s</span>
            <span>⬆️ {{ formatBytes(progress.upload_speed) }}/s</span>
            <span>👥 Peers: {{ progress.peers }}</span>
          </div>

          <!-- Botão para assistir no VLC -->
          <button 
            v-if="progress.progress_percent > 2" 
            @click="openVlc" 
            class="stream-button"
          >
            ▶️ Assistir no VLC
          </button>
          <p v-else class="hint-text">Aguarde pelo menos 2% do download para habilitar o VLC.</p>
        </div>

        <div v-else class="empty-state">
          Nenhum download ativo no momento.
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const magnetLink = ref('')
const progress = ref(null)
let unlisten = null

const formatBytes = (bytes) => {
  if (!bytes || bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

onMounted(async () => {
  unlisten = await listen('torrent-progress', (event) => {
    progress.value = event.payload
  })
})

onUnmounted(() => {
  if (unlisten) unlisten()
})

const addTorrent = async (source) => {
  if (!source) return
  try {
    const resposta = await invoke('add_torrent', { source: source })
    alert(resposta)
    magnetLink.value = ''
  } catch (erro) {
    console.error(erro)
    alert('Erro no backend: ' + erro)
  }
}

const openVlc = async () => {
  try {
    const resposta = await invoke('open_vlc', { torrentIndex: 0 })
    alert(resposta)
  } catch (erro) {
    alert('Erro ao abrir VLC: ' + erro)
  }
}
</script>

<style>
* { box-sizing: border-box; margin: 0; padding: 0; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
body { background-color: #121212; color: #ffffff; }
.app-container { display: flex; flex-direction: column; min-height: 100vh; }
.header { padding: 20px 30px; background-color: #1e1e1e; border-bottom: 1px solid #333; }
.header h1 { font-size: 24px; font-weight: 600; }
.header h1 span { color: #4CAF50; }
.main-content { padding: 30px; display: flex; flex-direction: column; gap: 30px; max-width: 1000px; margin: 0 auto; width: 100%; }

.input-group { display: flex; gap: 10px; }
.input-group input { flex: 1; padding: 12px 15px; border-radius: 6px; border: 1px solid #333; background-color: #1e1e1e; color: #fff; font-size: 16px; outline: none; }
.input-group input:focus { border-color: #4CAF50; }
.input-group button { padding: 12px 25px; border-radius: 6px; border: none; background-color: #4CAF50; color: white; font-size: 16px; font-weight: bold; cursor: pointer; }
.input-group button:hover { background-color: #45a049; }

.downloads-panel { background-color: #1e1e1e; border-radius: 8px; padding: 20px; border: 1px solid #333; }
.downloads-panel h2 { font-size: 18px; margin-bottom: 15px; color: #ccc; }
.empty-state { color: #666; text-align: center; padding: 40px 0; font-style: italic; }

.progress-container { display: flex; flex-direction: column; gap: 10px; }
.progress-header { display: flex; justify-content: space-between; font-size: 14px; color: #ccc; }
.progress-bar { width: 100%; height: 20px; background-color: #333; border-radius: 4px; overflow: hidden; }
.progress-fill { height: 100%; background-color: #4CAF50; transition: width 0.3s; }
.stats-info { display: flex; justify-content: space-between; font-size: 13px; color: #888; }

.stream-button { margin-top: 10px; padding: 10px; background-color: #2196F3; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 16px; font-weight: bold; }
.stream-button:hover { background-color: #1976D2; }
.hint-text { font-size: 12px; color: #888; text-align: center; margin-top: 5px; }
</style>