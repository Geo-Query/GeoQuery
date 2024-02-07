import React from 'react';
const { ipcRenderer } = window.require('electron');

interface DirectoryPickerProps {
  onDirectorySelect: (directoryPath: string) => void;
}

const DirectoryPicker: React.FC<DirectoryPickerProps> = ({ onDirectorySelect }) => {
  const handleDirectorySelect = async () => {
    const directoryPath: string | null = await ipcRenderer.invoke('select-directory');
    if (directoryPath) {
      onDirectorySelect(directoryPath);
    }
  };

  return (
    <button onClick={handleDirectorySelect} className="text-white bg-blue-500 hover:bg-blue-700 font-bold py-2 px-4 rounded">
      Select Directory
    </button>
  );
};

export default DirectoryPicker;
