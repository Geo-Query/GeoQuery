import React, { useState } from 'react';
import FolderCard from './folder_card'; // Ensure this component is also refactored to TSX
// import TagBuilder from './TagBuilder'; // This should also be a TSX component
import { LuUndo2 } from 'react-icons/lu'; // Assuming you have an SVG version for TypeScript

interface Folder {
  id: number;
  name: string;
  tags: string;
  children: Folder[];
}

const TemplateEditor: React.FC = () => {
  const [folders, setFolders] = useState<Folder[]>([{
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

  const [selectedFolderId, setSelectedFolderId] = useState<number | null>(null);
  const [deletedFolder, setDeletedFolder] = useState<{ folder: Folder; parentId: number | null } | null>(null);

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
    return folders.map(folder => (
        <FolderCard
        key={folder.id}
        folder={folder}
        depth={depth}
        onAddChild={() => addChildFolder(folder.id)}
        onDelete={() => deleteFolder(folder.id)}
        onRename={(folderId, newName) => renameFolder(folderId, newName)}
        onSelect={() => setSelectedFolderId(folder.id)}
        onEditTags={() => {}} // Implement your tag editing logic here
      >
        {folder.children && renderFolderCards(folder.children, depth + 1)}
      </FolderCard>
    ));
  };

  // Find the selected folder based on selectedFolderId
  const findFolderById = (folders: Folder[], id: number): Folder | undefined => {
    for (let folder of folders) {
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
      <div className="folder-cards">
        {renderFolderCards(folders)}
      </div>
      {/* {selectedFolder && (
        <TagBuilder
          folder={selectedFolder}
          onUpdateTags={(tags, expressions) => 
            updateTagsAndExpressions(selectedFolderId ?? 0, tags, expressions)} // Adjust based on TagBuilder's expected props
        />
      )} */}
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