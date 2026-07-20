// [frensense]
// observation: contextIsolation: false allows the renderer to access Electron and Node APIs directly
// impact: XSS in the renderer exposes full IPC, process, and file system access
// improvement: always set contextIsolation: true

import { BrowserWindow, app } from 'electron'

function createWindow() {
  const win = new BrowserWindow({
    webPreferences: {
      contextIsolation: false,
      preload: path.join(__dirname, 'preload.js'),
    },
  })
  win.loadFile('index.html')
  return win
}

app.whenReady().then(createWindow)
