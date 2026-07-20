// SAFE: validates the URL against an allowlist of trusted domains

import { shell } from 'electron'
import { BrowserWindow, app } from 'electron'

const ALLOWED_HOSTS = new Set(['docs.example.com', 'help.example.com'])

function isTrustedUrl(raw: string): boolean {
  try {
    const url = new URL(raw)
    return url.protocol === 'https:' && ALLOWED_HOSTS.has(url.hostname)
  } catch {
    return false
  }
}

function openLink(win: BrowserWindow, url: string) {
  win.webContents.on('will-navigate', (event, targetUrl) => {
    if (!isTrustedUrl(targetUrl)) {
      event.preventDefault()
      return
    }
    shell.openExternal(targetUrl)
  })
}

app.whenReady().then(() => {
  const win = new BrowserWindow({ width: 800, height: 600 })
  win.loadURL('https://app.example.com')
  openLink(win, 'https://docs.example.com/guide')
})
