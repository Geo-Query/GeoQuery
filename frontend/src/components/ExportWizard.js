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
    const addFileHandle = (newFileHandles) => {
        // Ensure newFileHandles is always treated as an array
        const fileHandlesArray = [].concat(newFileHandles);
        setFileHandles((prevHandles) => [...prevHandles, ...fileHandlesArray]);
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

        try {
            for (const fileHandle of fileHandles) {
                await copyFile(fileHandle, directoryHandle);
            }
            setCopySuccess(true); // Set success notification state
            setFileHandles([]); // Clear the selected file handles
            console.log('All files have been copied successfully!');
            // Optional: Automatically close the modal after a delay
            //   setTimeout(() => {
            //     onClose();
            //     setCopySuccess(false); // Reset the success notification state
            //   }, 3000); // 3 seconds delay
        } catch (error) {
            console.error('Error copying files:', error);
            // Handle error state here if needed
        }
    };



    // Here, you can close the modal or provide any success message
    // onClose(); // Optional: Close the modal after copying
    const [copySuccess, setCopySuccess] = useState(false);

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <div className="flex flex-col items-center p-4">
                {/* File Selector allows multiple file selection */}
                <FileSelector onFileSelect={(fileHandles) => addFileHandle(fileHandles)} />

                {/* Directory Picker allows the user to select a destination directory */}
                <DirectoryPicker onDirectorySelect={addDirectoryHandle} />

                {/* List the names of the selected files */}
                <div className="w-full overflow-y-auto max-h-40 my-4">
                    {fileHandles.length > 0 ? (
                        fileHandles.map((fileHandle, index) => (
                            <div key={index} className="text-white my-2">
                                {fileHandle.name} {/* Displaying the file name */}
                            </div>
                        ))
                    ) : (
                        <div className="text-gray-500">No files selected.</div>
                    )}
                </div>

                {/* Button to trigger the file copy process */}
                {/* Success message */}
                {copySuccess && (
                    <div className="text-sm bg-green-200 text-green-700 p-2 rounded-lg">
                        Files copied successfully!
                    </div>
                )}
                <button
                    className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                    onClick={handleCopyFiles}
                    disabled={fileHandles.length === 0} // Disable the button if no files are selected
                >
                    Copy Files
                </button>
            </div>
        </Modal>
    );
};

export default ExportWizard;
