// ExportWizard.js
import React, { useState } from 'react';
import Modal from './Modal';
import FileSelector from './FileSelector';
import DirectoryPicker from './DirectoryPicker';
import { copyFile } from '../utils/fileOperations';

const ExportWizard = ({ isOpen, onClose, selectedFiles }) => {
  // State to hold user's file and directory choices
  const [fileHandles, setFileHandles] = useState([]);
  const [directoryHandle, setDirectoryHandle] = useState(null);

  // Function to update the fileHandles state
  const addFileHandle = (fileHandle) => {
    setFileHandles((prevHandles) => [...prevHandles, fileHandle]);
  };

  // Function to update the directoryHandle state
  const addDirectoryHandle = (dirHandle) => {
    setDirectoryHandle(dirHandle);
  };

  // Function to initiate the file copy process
  const handleCopyFiles = async () => {
    if (!directoryHandle) {
      console.error("No directory selected for copying files.");
      return;
    }
    
    for (const fileHandle of fileHandles) {
      await copyFile(fileHandle, directoryHandle);
    }
  
    console.log('All files have been copied successfully!');
    // Here, you can close the modal or provide any success message
    // onClose(); // Optional: Close the modal after copying
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <div className="flex flex-col items-center">
        <FileSelector onFileSelect={addFileHandle} />
        <DirectoryPicker onDirectorySelect={addDirectoryHandle} />
        {selectedFiles.map((file, index) => (
          <div key={index} className="text-white">
            {file}
          </div>
        ))}
        <button
          className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 m-1 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
          onClick={handleCopyFiles}
        >
          Copy Files
        </button>
      </div>
    </Modal>
  );
};

export default ExportWizard;
