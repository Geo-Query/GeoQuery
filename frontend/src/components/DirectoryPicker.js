import React from 'react';

const DirectoryPicker = ({ onDirectorySelect }) => {
  const handleDirectorySelect = async () => {
    try {
      const directoryHandle = await window.showDirectoryPicker();
      onDirectorySelect(directoryHandle);
    } catch (error) {
      console.error('Error selecting directory:', error);
    }
  };

  return <button onClick={handleDirectorySelect}>Select Directory</button>;
};

export default DirectoryPicker;
