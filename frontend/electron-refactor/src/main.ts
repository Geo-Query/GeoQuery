import { app, BrowserWindow, ipcMain, dialog } from 'electron';
import path from 'path';
import fs from 'fs';
import util from 'util';

const copyFile = util.promisify(fs.copyFile);

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  app.quit();
}

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow: BrowserWindow | null;

const createWindow = (): void => {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      // It's important to preload a script to have access to Node.js features
      preload: path.join(__dirname, 'preload.js'),
      // Enable Node.js integration
      nodeIntegration: true,
      // Context Isolation is an Electron feature that protects against prototype pollution
      // and bypasses other security measures. It should be kept as true for security purposes.
      // If you need to disable it to use certain Electron APIs, be sure to understand the security implications.
      contextIsolation: false, // Set to false if you need to disable it
    },
  });

  // Load the index.html of the app.
  if (MAIN_WINDOW_VITE_DEV_SERVER_URL) {
    mainWindow.loadURL(MAIN_WINDOW_VITE_DEV_SERVER_URL);
  } else {
    mainWindow.loadFile(path.join(__dirname, `../renderer/${MAIN_WINDOW_VITE_NAME}/index.html`));
  }
  // Open the DevTools.
  mainWindow.webContents.openDevTools();
};

// IPC event for selecting the destination directory
ipcMain.handle('select-directory', async () => {
  const result = await dialog.showOpenDialog(mainWindow!, {
    properties: ['openDirectory']
  });
  if (result.canceled) {
    return null;
  } else {
    return result.filePaths[0];
  }
});

// IPC event for copying files
ipcMain.handle('copy-files', async (event, sourceFiles: string[], destination: string) => {
  try {
    for (const sourceFile of sourceFiles) {
      const fileName = path.basename(sourceFile);
      const destinationPath = path.join(destination, fileName);
      await copyFile(sourceFile, destinationPath);
    }
    return { success: true };
  } catch (error) {
    console.error('Failed to copy files:', error);
    return { success: false, error: error.message };
  }
});

app.on('ready', createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (mainWindow === null) {
    createWindow();
  }
});
