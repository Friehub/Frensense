// SAFE: uses contextBridge in the preload to expose only safe APIs, with isolation on

import { BrowserWindow, app, contextBridge, ipcRenderer } from 'electron'

function createWindow() {
  const win = new BrowserWindow({
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      preload: path.join(__dirname, 'preload.js'),
    },
  })
  win.loadFile('index.html')
  return win
}

app.whenReady().then(createWindow)
