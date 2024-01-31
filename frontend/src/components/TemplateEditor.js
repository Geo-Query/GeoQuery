import React, { useState } from 'react';
import FolderCard from './FolderCard';
import TagBuilder from './TagBuilder';
import { LuUndo2 } from "react-icons/lu";

const TemplateEditor = () => {
    
  const [folders, setFolders] = useState([{
    id: 1,
    name: "Folder 1",
    tags: "FileType = 'tiff' AND Resolution >= '10x'",
    children: [
    {
        id: 2,
        name: "Folder 2",
        tags: "FileType = 'GPKG' AND Resolution >= '20x'",
        children: []
    },
    {
        id: 3,
        name: "Folder 3",
        tags: "Resolution = '50x'",
        children: [
            {
                id: 4,
                name: "Folder 4",
                tags: "FileType = 'GEO'",
                children: []
            },
            {
                id: 5,
                name: "Folder 5",
                tags: "FileType = 'GPKG'",
                children: []
            } 
        ]
    },
    {
        id: 6,
        name: "Folder 6",
        tags: "FileType = 'GPKG' AND Resolution < '20x'",
        children: []
      }
      ],
    }]);
    const [selectedFolderId, setSelectedFolderId] = useState(null);
    const [deletedFolder, setDeletedFolder] = useState(null);

 
    const addChildFolder = (parentId, folderToAdd = null) => {
        const newFolder = folderToAdd || {
            id: Date.now(), // generate a unique id for the new folder
            name: "New Folder",
            tags: "",
            children: []
        };
    

    const addFolder = (folders, parentId, newFolder) => {
        return folders.map(folder => {
        if (folder.id === parentId) {
            return { ...folder, children: [...folder.children, newFolder] };
        } else if (folder.children) {
            return { ...folder, children: addFolder(folder.children, parentId, newFolder) };
        } else {
            return folder;
        }
        });
    };

    const newFolders = addFolder(folders, parentId, newFolder);
    setFolders(newFolders);
    };



    const deleteFolder = (folderId) => {

        const folderToDelete = findFolderById(folders, folderId);
        setDeletedFolder({ folder: folderToDelete, parentId: findParentId(folders, folderId) });

        const removeFolder = (folders, folderId) => {
        return folders.reduce((result, folder) => {
            if (folder.id === folderId) {
            return result;
            } else if (folder.children) {
            return [...result, { ...folder, children: removeFolder(folder.children, folderId) }];
            } else {
            return [...result, folder];
            }
        }, []);
        };

        const newFolders = removeFolder(folders, folderId);
        setFolders(newFolders);
    };

    const findParentId = (folders, folderId, parentId = null) => {
        for (let folder of folders) {
            if (folder.id === folderId) {
                return parentId;
            }
            if (folder.children) {
                const foundParentId = findParentId(folder.children, folderId, folder.id);
                if (foundParentId !== null) return foundParentId;
            }
            }
        return null;
    };

    const undoDelete = () => {
        if (deletedFolder) {
            const { folder, parentId } = deletedFolder;
            addChildFolder(parentId, folder);
            setDeletedFolder(null);
        }
    };

    const renameFolder = (folderId, newName) => {
        const rename = (folders) => {
          return folders.map(folder => {
            if (folder.id === folderId) {
              return { ...folder, name: newName };
            } else if (folder.children) {
              return { ...folder, children: rename(folder.children) };
            } else {
              return folder;
            }
          });
        };
    
        const newFolders = rename(folders);
        setFolders(newFolders);
      };

  // Function to update tags and boolean expressions for a folder
  const updateTagsAndExpressions = (folderId, tags, expressions) => {
    // Logic to update tags and boolean expressions for a folder
  };

// Function to render folder cards
const renderFolderCards = (folderData, depth = 0) => {
    return folderData.map(folder => (
      <FolderCard
        key={folder.id}
        folder={folder}
        depth={depth}
        onAddChild={() => addChildFolder(folder.id)}
        onDelete={() => deleteFolder(folder.id)}
        onRename={renameFolder}
        onSelect={() => setSelectedFolderId(folder.id)}
        onEditTags={() => {/* logic to handle tag editing */}}
      >
        {/* Recursively render child folders with increased depth */}
        {folder.children && renderFolderCards(folder.children, depth + 1)}
      </FolderCard>
    ));
  };
  

  // Get the currently selected folder object
  const findFolderById = (folders, id) => {
    for (let folder of folders) {
      if (folder.id === id) {
        return folder;
      }
      if (folder.children) {
        const found = findFolderById(folder.children, id);
        if (found) return found;
      }
    }
    return null;
  };

  const selectedFolder = findFolderById(folders, selectedFolderId);

  return (
<div className="template-editor">
      <div className="folder-cards">
        {renderFolderCards(folders)}
      </div>
      {selectedFolder && (
        <TagBuilder
          folder={selectedFolder}
          onUpdateTags={(tags, expressions) => 
            updateTagsAndExpressions(selectedFolderId, tags, expressions)}
        />
      )}
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
