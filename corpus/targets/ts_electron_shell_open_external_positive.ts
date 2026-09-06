// [frensense]
// observation: shell.openExternal is called with a user-supplied URL without validation
// impact: attacker invokes arbitrary protocols (file://, ssh://, custom handlers) leading to RCE
// improvement: validate URL against an allowlist of permitted schemes and hosts

import { shell } from 'electron'
import { BrowserWindow, app } from 'electron'

function openLink(win: BrowserWindow, url: string) {
  win.webContents.on('will-navigate', (event, targetUrl) => {
    event.preventDefault()
    shell.openExternal(targetUrl)
  })
}

app.whenReady().then(() => {
  const win = new BrowserWindow({ width: 800, height: 600 })
  win.loadURL('https://app.example.com')
  openLink(win, 'https://evil.com/steal')
})
