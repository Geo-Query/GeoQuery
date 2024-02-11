//folderStructureService.ts
import fs from 'fs/promises';
import path from 'path';

async function createFolderRecursively(folder, rootPath) {
  const folderPath = path.join(rootPath, folder.relativePath);
  await fs.mkdir(folderPath, { recursive: true });

  for (const child of folder.children) {
    await createFolderRecursively(child, rootPath);
  }
}

export async function createFolderStructure(template, rootPath: string) {
  for (const folder of template) {
    await createFolderRecursively(folder, rootPath);
  }
}
