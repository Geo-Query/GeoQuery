import React from 'react';

const FileSelector = ({ onFileSelect }) => {
    const handleFileSelect = async () => {
        try {
          const fileHandles = await window.showOpenFilePicker({ multiple: true });
          fileHandles.forEach(fileHandle => {
            onFileSelect(fileHandle);
          });
        } catch (error) {
          console.error('Error selecting files:', error);
        }
      };

  return <button onClick={handleFileSelect} className="text-white">Select File</button>;
};

export default FileSelector;
