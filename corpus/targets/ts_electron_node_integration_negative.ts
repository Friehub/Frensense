// SAFE: disables nodeIntegration and enables contextIsolation

import { BrowserWindow, app } from 'electron'

function createMainWindow() {
  const win = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
    },
  })
  win.loadURL('https://app.example.com')
  return win
}

app.whenReady().then(createMainWindow)
