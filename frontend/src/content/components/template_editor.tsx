import React, { useState, useEffect} from 'react';
import FolderCard from './folder_card'; // Ensure this component is also refactored to TSX
// import TagBuilder from './TagBuilder'; // This should also be a TSX component
import { LuUndo2 } from 'react-icons/lu'; // Assuming you have an SVG version for TypeScript
import FolderTemplatesStorage from '../utils/folder_template_storage';
import { FolderTemplate } from '../utils/folder_template_storage';

interface Folder {
  id: number;
  name: string;
  tags: string;
  children: Folder[];
}
interface TemplateEditorProps {
  folder: Folder;
  name: string | null;
  onUpdateTemplate: (updatedTemplate: Folder) => void;
}

const TemplateEditor: React.FC<TemplateEditorProps> = ({ folder, name,  onUpdateTemplate }) => {

  const [newTemplateName, setNewTemplateName] = useState(name ? name : "");
  const [folders, setFolders] = useState<Folder[]>([folder]);
  const [selectedFolderId, setSelectedFolderId] = useState<number | null>(null);
  const [deletedFolder, setDeletedFolder] = useState<{ folder: Folder; parentId: number | null } | null>(null);

  useEffect(() => {
    onUpdateTemplate(folders[0]); // If folders[0] represents the root folder template
  }, [folders, onUpdateTemplate]);

  // Function to recursively update folder tags
  const updateFolderTags = (folders: Folder[], folderId: number, newTags: string): Folder[] => {
    return folders.map(folder => {
      if (folder.id === folderId) {
        // Found the folder, update its tags
        return { ...folder, tags: newTags };
      } else if (folder.children) {
        // Folder has children, recursively search them
        return { ...folder, children: updateFolderTags(folder.children, folderId, newTags) };
      }
      return folder; // No update needed, return folder as is
    });
  };
  
  // Function to be passed to FolderCard's onEditTags prop
  const onEditTags = (folderId: number, newTags: string) => {
    setFolders(currentFolders => updateFolderTags(currentFolders, folderId, newTags));
  };
  const addChildFolder = (parentId: number, folderToAdd?: Folder) => {
    const newFolder = folderToAdd || {
      id: Date.now(), // Unique ID for the new folder
      name: "New Folder",
      tags: "",
      children: []
    };

    console.log(`Adding new folder to parent ${parentId}`)
  
    const addFolderRecursively = (folders: Folder[], parentId: number, newFolder: Folder): Folder[] => {
      return folders.map(folder => {
        if (folder.id === parentId) {
          return { ...folder, children: [...folder.children, newFolder] };
        }
        if (folder.children.length) {
          return { ...folder, children: addFolderRecursively(folder.children, parentId, newFolder) };
        }
        return folder;
      });
    };
  
    setFolders(folders => addFolderRecursively(folders, parentId, newFolder));
  };
  

  const deleteFolder = (folderId: number) => {
    const findFolderAndParent = (folders: Folder[], folderId: number): [Folder | undefined, number | null] => {
      for (const folder of folders) {
        if (folder.id === folderId) return [folder, null];
        if (folder.children) {
          for (const child of folder.children) {
            if (child.id === folderId) return [child, folder.id];
          }
          const [foundFolder, parentId] = findFolderAndParent(folder.children, folderId);
          if (foundFolder) return [foundFolder, parentId || folder.id];
        }
      }
      return [undefined, null];
    };
  
    const [folderToDelete, parentId] = findFolderAndParent(folders, folderId);
    if (!folderToDelete) return;
  
    setDeletedFolder({ folder: folderToDelete, parentId });
  
    const removeFolderRecursively = (folders: Folder[], folderId: number): Folder[] => {
      return folders.reduce((acc: Folder[], folder) => {
        if (folder.id === folderId) return acc;
        if (folder.children) {
          return [...acc, { ...folder, children: removeFolderRecursively(folder.children, folderId) }];
        }
        return [...acc, folder];
      }, []);
    };
  
    setFolders(folders => removeFolderRecursively(folders, folderId));
  };
  

  const undoDelete = () => {
    if (!deletedFolder) return;
  
    const { folder, parentId } = deletedFolder;
    if (parentId !== null) {
      addChildFolder(parentId, folder);
    } else {
      // The folder was a top-level folder
      setFolders(folders => [...folders, folder]);
    }
    setDeletedFolder(null);
  };
  
const renameFolder = (folderId: number, newName: string) => {
    console.log(`Renaming folder ${folderId} to ${newName}`);
    const renameFolderRecursively = (folders: Folder[], folderId: number, newName: string): Folder[] => {
        return folders.map(folder => {
        if (folder.id === folderId) {
            return { ...folder, name: newName };
        }
        if (folder.children.length) {
            return { ...folder, children: renameFolderRecursively(folder.children, folderId, newName) };
        }
        return folder;
        });
    };

    setFolders(folders => renameFolderRecursively(folders, folderId, newName));
};


const renderFolderCards = (folders: Folder[], depth = 0): JSX.Element[] => {
  return folders.map(folder => {
    // Log the ID of the folder being rendered
    // console.log(`Rendering folder ID: ${folder.id} at depth: ${depth}`);

    return (
      <FolderCard
        key={folder.id}
        folder={folder}
        depth={depth}
        onAddChild={() => addChildFolder(folder.id)}
        onDelete={() => deleteFolder(folder.id)}
        onRename={(folderId, newName) => renameFolder(folderId, newName)}
        onSelect={() => setSelectedFolderId(folder.id)}
        onEditTags={(newTags: string) => onEditTags(folder.id, newTags)} // Correctly pass a function with expected parameters
      >
        {/* Recursively render child folders if they exist */}
        {folder.children && folder.children.length > 0 && renderFolderCards(folder.children, depth + 1)}
      </FolderCard>
    );
  });
};

const saveCurrentTemplate = () => {
  // Check for empty template name
  if (!newTemplateName.trim()) {
    alert("Template name cannot be empty.");
    return;
  }

  const newTemplate: FolderTemplate = {
    folderName: newTemplateName,
    folderCreationDate: new Date().toISOString(),
    folder: folders[0] // Assuming the top-level folder is the template root,
  };

  const storage = FolderTemplatesStorage.loadFromStorage();
  storage.add(newTemplate);
  console.log("Template saved");
};


  // Find the selected folder based on selectedFolderId
  const findFolderById = (folders: Folder[], id: number): Folder | undefined => {
    for (const folder of folders) {
      if (folder.id === id) {
        return folder;
      }
      if (folder.children.length > 0) {
        const found = findFolderById(folder.children, id);
        if (found) return found;
      }
    }
    return undefined;
  };

  const selectedFolder = selectedFolderId ? findFolderById(folders, selectedFolderId) : null;

  return (
    <div className="template-editor">
      <div className="flex flex-col h-full justify-between">
        <div className="mt-auto">
          <div className="flex justify-end items-center space-x-2 py-2">
            <input
              type="text"
              placeholder="Enter template name"
              value={newTemplateName}
              onChange={(e) => setNewTemplateName(e.target.value)}
              className="px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            />
            <button 
              onClick={saveCurrentTemplate} 
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out"
            >
              Save Current Template
            </button>
          </div>
        </div>
      </div>

      <div className="folder-cards">
        {renderFolderCards(folders)}
      </div>
      {deletedFolder && (
        <button 
          className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded"
          onClick={undoDelete}
        >
          <LuUndo2 />
        </button>
      )}
    </div>
  );
};

export default TemplateEditor;