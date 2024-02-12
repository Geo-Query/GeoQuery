import React, { useState } from 'react';
import FolderTemplatesStorage from '../lib/folder_template_storage'; // Adjust the import path as necessary
import TemplateEditor from './template_editor'; // Adjust the import path as necessary
import { FolderTemplate } from '../lib/folder_template_storage';

interface Folder {
  id: number;
  name: string;
  tags: string;
  children: Folder[];
}

const FoldersTemplate: React.FC = () => {
  const [selectedTemplate, setSelectedTemplate] = useState<Folder | null>(null);
  const folderTemplates = FolderTemplatesStorage.loadFromStorage();

  const onSelectTemplate = (template: Folder) => {
    setSelectedTemplate(template);
  };

  const addNewTemplate = () => {
    const newFolderTempalate: Folder = {
      id: Date.now(),
      name: 'New Template',
      tags: '',
      children: [],
    };

    //create a new folder

    onSelectTemplate(newFolderTempalate);

  };

  const handleDelete = (templateId: number) => {
    folderTemplates.delete(templateId);
  };

  return (
    <div>
    {selectedTemplate ? (
      <>
        <div className="flex flex-col h-full justify-between">
          <TemplateEditor folder={selectedTemplate} />

          <div className="flex justify-end space-x-2 mt-4">
            <button 
              onClick={() => setSelectedTemplate(null)}
              className="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out shadow"
            >
              Back
            </button>

            <button 
              // Assuming there's a function to handle export
              // onClick={() => handleExport(selectedTemplate)}
              className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out shadow"
            >
              Export
            </button>
          </div>
        </div>
      </>
    ) : (
      <>
        <button 
          onClick={addNewTemplate} 
          className="mb-4 mx-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out shadow"
        >
          Add New Template
        </button>
        <div className="flex flex-col space-y-4">
          {folderTemplates.templates.map((template, index) => (
            <div key={index} className="flex justify-between items-center mb-2 bg-[#525461] rounded-lg shadow-lg p-4 transition-transform duration-300 ease-in-out hover:bg-[#526071]">
              <button 
                onClick={() => onSelectTemplate(template.folder)} 
                className="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow transition duration-150 ease-in-out"
              >
                {template.folderName}
              </button>
              <button 
                onClick={() => handleDelete(template.folder.id)} 
                className="ml-2 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out shadow"
              >
                Delete
              </button>
            </div>
          ))}
        </div>
      </>
    )}
  </div>
  
  );
};

export default FoldersTemplate;
