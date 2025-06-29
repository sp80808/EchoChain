const { app, BrowserWindow } = require('electron');
const path = require('path');

function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 800,
    minHeight: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      nodeIntegration: true,
      contextIsolation: false, // Be cautious with this in production
    },
  });

  // Load the React app
  // In development, load from Vite's dev server
  // In production, load the built index.html
  const startUrl = process.env.NODE_ENV === 'development'
    ? 'http://localhost:5173' // Default Vite dev server port
    : `file://${path.join(__dirname, '..', 'dist', 'index.html')}`;

  mainWindow.loadURL(startUrl);

  // Open the DevTools.
  // mainWindow.webContents.openDevTools();
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
