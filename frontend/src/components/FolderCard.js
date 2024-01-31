import React, {useState} from 'react';
import { IoMdAdd } from "react-icons/io";
import { IoMdRemove } from "react-icons/io";

const FolderCard = ({ folder, depth, onAddChild, onDelete, onRename, onSelect, onEditTags, children }) => {
    // Style modifications based on depth
    const borderIntensity = 100 + depth * 30; // Increase border intensity with depth
    const borderColor = `rgba(255, 255, 255, ${borderIntensity > 255 ? 255 : borderIntensity / 255})`;
    const [isEditing, setIsEditing] = useState(false);
    const [newName, setNewName] = useState(folder.name);

    const isValidName = (name) => {
        const invalidChars = /[<>:"/\\|?*]/;
        return !invalidChars.test(name);
    };

    const handleRename = () => {
        const trimmedName = newName.trim();
        if (trimmedName.length > 0 && isValidName(trimmedName)) {
            onRename(folder.id, trimmedName);
            setIsEditing(false);
        } else {
            // Alert the user about invalid folder name
            alert("Invalid folder name. Folder name cannot be empty and must not contain characters: < > : \" / \\ | ? *");
        }
    };


    const style = {
        backgroundColor: `rgba(82, 84, 97, ${1 - depth * 0.1})`, // Adjust background opacity
        border: `2px solid ${borderColor}`, // Border with variable color intensity
        padding: '1rem',
        margin: `${depth * 0.5}rem 0`, // Increase margin for nested folders
    };

    return (
        <div className="rounded-lg shadow-lg hover:shadow-xl" style={style}>
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
          <div>
            <span 
                className="font-mono text-sm text-white cursor-pointer"
                onClick={() => setIsEditing(true)}
            >
                {folder.name}
            </span>
          </div>
        )}

          <span className="text-xs font-semibold text-white">
            {folder.tags}
          </span>
        </div>
        <div className="border-t border-gray-600 my-2"></div>
        <div className="text-sm text-white">
          {children}
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
          
        </div>
        {/* Additional buttons or actions for the folder can go here */}
      </div>
    );
};

export default FolderCard;
