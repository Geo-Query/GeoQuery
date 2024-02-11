import { app, BrowserWindow, ipcMain, dialog } from 'electron';
import path from 'path';
import fs from 'fs';
import util from 'util';
import { readFile } from 'fs/promises';
import { createFolderStructure } from './content/services/folderStructureService';
import { copyFilesToStructure } from './content/services/fileCopierService';

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
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
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

ipcMain.handle('read-template-file', async (event, jsonFilePath) => {
  try {
    const data = await readFile(jsonFilePath, 'utf8');
    return JSON.parse(data);
  } catch (error) {
    console.error('Failed to read template file:', error);
    throw error; // Rethrow to send error back to renderer
  }
});

ipcMain.handle('execute-export', async (event, directory, template) => {
  try {
      // Assuming createFolderStructure and copyFilesToStructure are functions that you will define or import
      await createFolderStructure(template, directory);
      await copyFilesToStructure(template[0], directory).then(() => {
        console.log('Export process completed.');
      }).catch(console.error);
      return { success: true, message: "Export completed successfully!" };
  } catch (error) {
      console.error("Export failed:", error);
      return { success: false, message: `Export failed: ${error.message}` };
  }
});
