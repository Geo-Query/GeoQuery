import React, { useState } from 'react';
import { ReactNode } from 'react';
import { IoMdAdd, IoMdRemove, IoMdCreate } from "react-icons/io";

import TagBuilder from './expression_builder';
interface Folder {
  id: number;
  name: string;
  tags: string;
  children: Folder[];
}

interface FolderCardProps {
    folder: Folder;
    depth: number;
    onAddChild: (parentId: number) => void;
    onDelete: (folderId: number) => void;
    onRename: (folderId: number, newName: string) => void;
    onSelect: () => void;
    onEditTags: (newTags: string) => void; // Adjust this based on how you plan to use it
    children?: ReactNode;
  }

const FolderCard: React.FC<FolderCardProps> = ({ folder, depth, onAddChild, onDelete, onRename, onSelect, onEditTags, children }) => {
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [newName, setNewName] = useState<string>(folder.name);
  const [showTagBuilder, setShowTagBuilder] = useState<boolean>(false); // State to toggle TagBuilder visibility


  const isValidName = (name: string): boolean => {
    const invalidChars = /[<>:"/\\|?*]/;
    return !invalidChars.test(name);
  };

  const handleRename = () => {
    const trimmedName = newName.trim();
    if (trimmedName.length > 0 && isValidName(trimmedName)) {
      onRename(folder.id, trimmedName);
      setIsEditing(false);
    } else {
      alert("Invalid folder name. Folder name cannot be empty and must not contain characters: < > : \" / \\ | ? *");
    }
  };

  const style = {
    backgroundColor: `rgba(82, 84, 97, ${1 - depth * 0.1})`,
    border: `2px solid rgba(255, 255, 255, ${0.1 + depth * 0.1})`,
    padding: '1rem',
    margin: `${depth * 0.5}rem 0`,
  };

  const toggleTagBuilder = () => setShowTagBuilder(!showTagBuilder); // Function to toggle TagBuilder display

  return (
    <div className="folder-card rounded-lg shadow-lg hover:shadow-xl" style={style}>
      <div className="flex justify-between items-center">
        {isEditing ? (
          <div>
            <input 
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="text-black"
            />
            <button onClick={handleRename}>Save</button>
            <button onClick={() => setIsEditing(false)}>Cancel</button>
          </div>
        ) : (
          <div onClick={onSelect}>
            <span 
                className="font-mono text-sm text-white cursor-pointer"
                onClick={() => setIsEditing(true)}
            >
                {folder.name}
            </span>
          </div>
        )}
      <div className="cursor-pointer">
        <span className="text-xs font-semibold text-white">Tags: {folder.tags}</span>
      </div>
      
      </div>
      <div className="flex justify-end space-x-2 mt-2">
        <button 
          className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
          onClick={() => onDelete(folder.id)}
        >
          <IoMdRemove />
        </button>
        <button 
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          onClick={() => onAddChild(folder.id)}
        >
          <IoMdAdd />
        </button>
        <button 
          className="bg-orange-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          onClick={toggleTagBuilder}
          >
          <IoMdCreate />
        </button>
      </div>
        {/* Conditionally render the TagBuilder component */}
        {showTagBuilder && (
          <TagBuilder
          onTagsUpdated={(newTags: string) => {
            onEditTags(newTags); // Update tags for the folder
            setShowTagBuilder(false); // Hide TagBuilder
          }}
        />
      )}
      {children && <div className="mt-4">{children}</div>}


    </div>
  );
};

export default FolderCard;
