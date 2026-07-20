// SAFE: uses contextBridge to expose only specific APIs to the renderer

import { BrowserWindow, app, contextBridge, ipcRenderer } from 'electron'

function createMainWindow() {
  const win = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
    },
  })
  win.loadURL('https://app.example.com')
  return win
}

app.whenReady().then(createMainWindow)
