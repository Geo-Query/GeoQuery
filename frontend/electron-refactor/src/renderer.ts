import './index.css';
// Note: Setting this to true means we're running in Electron context
window.type = true;
import './content/app';

// Import IPC Renderer to communicate with the main process
const { ipcRenderer } = require('electron');

// Function to request the main process to open the directory picker
window.selectDirectory = async () => {
  return await ipcRenderer.invoke('select-directory');
};

// Function to request the main process to copy files
window.copyFiles = async (sourceFiles: string[], destination: string) => {
  return await ipcRenderer.invoke('copy-files', sourceFiles, destination);
};
