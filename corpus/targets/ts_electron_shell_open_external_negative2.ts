// SAFE: restricts shell.openExternal to http/https only and sanitizes

import { shell } from 'electron'
import { BrowserWindow, app } from 'electron'

function safeOpenExternal(raw: string): void {
  try {
    const url = new URL(raw)
    if (url.protocol !== 'http:' && url.protocol !== 'https:') return
    if (url.hostname === 'localhost' || url.hostname === '127.0.0.1') return
    shell.openExternal(url.toString())
  } catch {
    // ignore invalid URLs
  }
}

app.whenReady().then(() => {
  const win = new BrowserWindow({ width: 800, height: 600 })
  win.loadURL('https://app.example.com')
  win.webContents.on('will-navigate', (event, url) => {
    event.preventDefault()
    safeOpenExternal(url)
  })
})
