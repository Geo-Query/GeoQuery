import React from 'react';
// const { ipcRenderer } = window.require('electron');

const TestFileCopy = () => {
  const handleTestFileCopy = async () => {
    try {
      // Mock a list of file paths to copy
      const mockFilesToCopy = [
        'C:/Users/fkgde/Downloads/Map Data V3.7z',
        'C:/Users/fkgde/Documents/Books/Ai in games.pdf',
        // Add more mock file paths as needed
      ];

      // Prompt the user to select a directory
      const directoryPath = await ipcRenderer.invoke('select-directory');
      if (!directoryPath) {
        alert('Directory selection was cancelled');
        return;
      }

      // Copy the mock files to the selected directory
      const result = await ipcRenderer.invoke('copy-files', mockFilesToCopy, directoryPath);
      if (result.success) {
        alert('Files copied successfully!');
      } else {
        throw new Error(result.error);
      }
    } catch (error) {
      alert(`Error copying files: ${error}`);
    }
  };

  return (
    <button onClick={handleTestFileCopy} className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
      Test File Copy
    </button>
  );
};

export default TestFileCopy;
