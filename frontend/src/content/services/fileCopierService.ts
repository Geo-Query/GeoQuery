import fs from 'fs/promises';
import path from 'path';

// Helper function to copy files
async function copyFiles(files, targetDirectory) {
  for (const file of files) {
    const targetPath = path.join(targetDirectory, path.basename(file));
    console.info(`Copying file ${file} to ${targetPath}`);
    await fs.copyFile(file, targetPath).catch(error => {
      console.error(`Error copying file ${file}:`, error);
    });
  }
}

// Modified function with recursive handling
export async function copyFilesToStructure(folder, rootDirectory) {
  const folderPath = path.join(rootDirectory, folder.relativePath);
  console.info(`Processing folder: ${folderPath}`);

  // Ensure the folder exists
  await fs.mkdir(folderPath, { recursive: true }).catch(console.error);

  // Copy files contained in the current folder
  if (folder.filesContained && Array.isArray(folder.filesContained)) {
    await copyFiles(folder.filesContained, folderPath);
  }

  // Recursively process each child folder
  if (folder.children && Array.isArray(folder.children)) {
    for (const childFolder of folder.children) {
      await copyFilesToStructure(childFolder, rootDirectory); // Pass the same rootDirectory for recursive structure
    }
  }
}
