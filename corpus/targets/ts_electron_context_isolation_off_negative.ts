// SAFE: enables contextIsolation to sandbox the renderer process

import { BrowserWindow, app } from 'electron'

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
