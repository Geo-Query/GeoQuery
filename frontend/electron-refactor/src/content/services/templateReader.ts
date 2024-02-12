// Import the `app` module from `electron` to get the application's root directory.
// This code assumes that `readTemplateFile` is running in the main process. 
// If it's in the renderer process, you'd need to use Electron's `remote` module instead (which is not recommended in Electron's latest versions).

// import { app } from 'electron';
// import fs from 'fs/promises';
// import path from 'path';

// export async function readTemplateFile(relativeFilePath: string) {
//   // Use app.getAppPath() to get the absolute path of the application's root directory
//   const appPath = app.getAppPath();
//   // Resolve the absolute path of the file
//   const filePath = path.resolve(appPath, relativeFilePath);
//   const data = await fs.readFile(filePath, 'utf8');
//   return JSON.parse(data);
// }

//! Currently unused - delete if not needed
