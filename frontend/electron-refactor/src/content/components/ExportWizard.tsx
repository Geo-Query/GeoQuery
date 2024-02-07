import React, { useState } from 'react';
import Modal from './modal';
import FileSelector from './FileSelector';
import DirectoryPicker from './DirectoryPicker';

const { ipcRenderer } = window.require('electron');

interface FileHandle {
  name: string;
  path: string;
}

interface ExportWizardProps {
  isOpen: boolean;
  onClose: () => void;
  selectedFiles: FileHandle[];
}

const ExportWizard: React.FC<ExportWizardProps> = ({ isOpen, onClose, selectedFiles }) => {
  const [copySuccess, setCopySuccess] = useState(false);
  const [directoryPath, setDirectoryPath] = useState<string>('');

  const handleCopyFiles = async () => {
    if (!directoryPath) {
      console.error("No directory selected for copying files.");
      return;
    }

    try {
      const filePaths = selectedFiles.map(file => file.path);
      const result = await ipcRenderer.invoke('copy-files', filePaths, directoryPath);
      if (result.success) {
        setCopySuccess(true);
        console.log('All files have been copied successfully!');
      } else {
        throw new Error(result.error);
      }
    } catch (error) {
      console.error('Error copying files:', error);
      setCopySuccess(false);
    }
  };

  const handleDirectorySelected = (path: string) => {
    setDirectoryPath(path);
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <div className="flex flex-col items-center p-4">
        <FileSelector selectedFiles={selectedFiles} />

        <DirectoryPicker onDirectorySelect={handleDirectorySelected} />

        {copySuccess && (
          <div className="text-sm bg-green-200 text-green-700 p-2 rounded-lg">
            Files copied successfully!
          </div>
        )}

        <button
          className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
          onClick={handleCopyFiles}
          disabled={selectedFiles.length === 0 || !directoryPath}
        >
          Copy Files
        </button>
      </div>
    </Modal>
  );
};

export default ExportWizard;
