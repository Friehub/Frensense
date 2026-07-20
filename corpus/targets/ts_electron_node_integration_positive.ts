// [frensense]
// observation: BrowserWindow created with nodeIntegration: true in production
// impact: XSS vulnerability in renderer leads to full RCE via Node.js APIs (require, process)
// improvement: set nodeIntegration: false and use contextBridge for IPC

import { BrowserWindow, app } from 'electron'

function createMainWindow() {
  const win = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
    },
  })
  win.loadURL('https://app.example.com')
  return win
}

app.whenReady().then(createMainWindow)
