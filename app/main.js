"use strict";

const { app, BrowserWindow } = require('electron')

function createWindow() {
    // Create the browser window.
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: false
        }
    })

    // and load the index.html of the app.
    win.loadFile('index.html')
    win.webContents.openDevTools({
        "mode": "detach"
    })
}

app.whenReady().then(createWindow)
