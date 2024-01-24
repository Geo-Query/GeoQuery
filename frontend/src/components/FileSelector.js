import React from 'react';

const FileSelector = ({ onFileSelect }) => {
  const handleFileSelect = async () => {
    try {
      const [fileHandle] = await window.showOpenFilePicker();
      onFileSelect(fileHandle);
    } catch (error) {
      console.error('Error selecting file:', error);
    }
  };

  return <button onClick={handleFileSelect}>Select File</button>;
};

export default FileSelector;
