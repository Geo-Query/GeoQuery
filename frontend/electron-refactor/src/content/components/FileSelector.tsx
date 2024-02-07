import React from 'react';

interface FileHandle {
  name: string;
  path: string;
}

interface FileSelectorProps {
  selectedFiles: FileHandle[];
}

const FileSelector: React.FC<FileSelectorProps> = ({ selectedFiles }) => {
  return (
    <div className="w-full overflow-y-auto max-h-40 my-4">
      {selectedFiles.length > 0 ? (
        selectedFiles.map((file, index) => (
          <div key={index} className="text-white my-2">
            {file.name}
          </div>
        ))
      ) : (
        <div className="text-gray-500">No files selected.</div>
      )}
    </div>
  );
};

export default FileSelector;
