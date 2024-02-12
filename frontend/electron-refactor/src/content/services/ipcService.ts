// ipcService.ts
//! Used to communicate with the main process via IPC

export const selectDirectory = async () => {
  return ipcRenderer.invoke('select-directory');
};

export const copyFiles = async (sourceFiles: string[], destination: string) => {
  return ipcRenderer.invoke('copy-files', sourceFiles, destination);
};
