import './index.css';
// Note: Setting this to true means we're running in Electron context
window.type = true;
import './content/app';
import { selectDirectory, copyFiles } from './content/lib/ipcService'; // assuming ipcService.js is at this path

// Attach the services to window object to make them accessible throughout the renderer process
window.selectDirectory = selectDirectory;
window.copyFiles = copyFiles;
