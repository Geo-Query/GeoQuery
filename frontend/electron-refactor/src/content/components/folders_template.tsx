import React, { useState } from 'react';
import FolderTemplatesStorage from '../lib/folder_template_storage'; // Adjust the import path as necessary
import TemplateEditor from './template_editor'; // Adjust the import path as necessary
import { FolderTemplate } from '../lib/folder_template_storage';
import { QueryResult } from '../lib/query';
interface Folder {
  id: number;
  name: string;
  tags: string;
  children: Folder[];
}
interface ExportFolder {
  id: number;
  name: string;
  relativePath: string;
  filesContained: string[];
  children: ExportFolder[];
}

interface FoldersTemplateProps {
  results: Array<QueryResult>;
}

const FoldersTemplate: React.FC<FoldersTemplateProps> = ({ results }) => {
  const [selectedTemplate, setSelectedTemplate] = useState<Folder | null>(null);
  const folderTemplates = FolderTemplatesStorage.loadFromStorage();
  const [, setDirectoryPath] = useState("");

  const [editedTemplate, setEditedTemplate] = useState<Folder | null>(null);

  const handleTemplateEdit = (updatedTemplate: Folder) => {
    setSelectedTemplate(updatedTemplate);
  };


  const handleExport = () => {
    if (selectedTemplate) {
      exportData(selectedTemplate, results); // Use results here
    } else {
      console.error("No template selected for export");
    }
  };


  const exportData = async (selectedTemplate: Folder, queryResults: Array<QueryResult>) => {
    try {
    
        const directory = await window.electronAPI.selectDirectory();
        if (!directory) {
            alert("Please select a directory first.");
            return;
        }
        
        console.log(`Directory selected: ${directory}`);
        
        const parseExpression = (expression: string): ((tags: string[]) => boolean) => {
          console.log(`Parsing expression: ${expression}`);
          // Split the expression into conditions
          const conditions = expression.split('AND').map(part => part.trim().toLowerCase());
      
          console.log(`Conditions split from expression:`, conditions);
      
          // Convert each condition into a function that evaluates it against file tags
          const evaluators = conditions.map(condition => {
              console.log(`Processing condition: ${condition}`);
      
              // Match condition parts (assuming simple format for demonstration)
              const match = condition.match(/(\w+)\s*([=><!]+)\s*'([^']+)'/);
              if (!match) {
                  console.error(`Invalid condition format: ${condition}`);
                  return () => false; // Return a function that always returns false for invalid conditions
              }
      
              const [, attribute, operator, value] = match.map(m => m.toLowerCase()); // Ensure attribute and value are also lowercase
              console.log(`Parsed condition - Attribute: ${attribute}, Operator: ${operator}, Value: ${value}`);
      
              return (tags: string[]) => {
                  console.log(`Evaluating condition against tags. Attribute: ${attribute}, Operator: ${operator}, Value: ${value}, Tags:`, tags);
      
                  
                  // Find the tag in file's tags that corresponds to the attribute
                  const tagValue = tags.find(tag => tag.toLowerCase().startsWith(`${attribute}:`))?.split(':')[1].trim().toLowerCase();
                  console.log(`Evaluating tag: ${attribute}:${tagValue} against condition: ${condition}`);
                  console.log(`Found tag value for attribute '${attribute}': ${tagValue}`);
                  console.log(`Comparing tagValue: '${tagValue}' with condition value: '${value}' for operator: '${operator}'`);

                  if (!tagValue) {
                      console.log(`No tag value found for attribute '${attribute}', condition evaluates to false.`);
                      return false;
                  }
      
                  let result;
                  switch (operator) {
                      case '=':
                          result = tagValue === value;
                          break;
                      case '!=':
                          result = tagValue !== value;
                          break;
                      case '>':
                          result = parseFloat(tagValue) > parseFloat(value);
                          break;
                      case '<':
                          result = parseFloat(tagValue) < parseFloat(value);
                          break;
                      case '>=':
                          result = parseFloat(tagValue) >= parseFloat(value);
                          break;
                      case '<=':
                          result = parseFloat(tagValue) <= parseFloat(value);
                          break;
                      default:
                          console.error(`Unsupported operator: ${operator}`);
                          result = false; // Consider unsupported operators as false
                  }
                  console.log(`Condition evaluation result for attribute '${attribute}': ${result}`);
                  return result;
              };
          });
      
          // Combine evaluators into a single function that applies all conditions
          return (tags: string[]) => evaluators.every(evaluator => evaluator(tags));
      };
    
      
      // Example usage with a query result
      const fileMatchesExpression = (file: QueryResult, expression: string, tagsLowercase: string[]): boolean => {
        // If the expression is empty or invalid, evaluators may return false
        const evaluator = parseExpression(expression);
        return evaluator(tagsLowercase);
    };
    
      
      // Build export template structure
      const buildExportTemplate = (folder: Folder, results: Array<QueryResult>, basePath: string = ""): ExportFolder => {
          // Filter files that match folder's tags
          const filesContained = results.filter(result => {
            // Convert file tags to lowercase before passing them to the evaluator
            const tagsLowercase = result.tags.map(tag => tag.toLowerCase());
            return fileMatchesExpression(result, folder.tags, tagsLowercase); // Adjust fileMatchesExpression to accept tags as an argument
        }).map(result => result.file.path);
              
          // Construct the relative path and recursively build the structure for children
          const relativePath = basePath ? `${basePath}/${folder.name}` : folder.name;
          const children = folder.children.map(child => buildExportTemplate(child, results, relativePath));
      
          return { id: folder.id, name: folder.name, relativePath, filesContained, children };
      };

        // Construct the export template
        const exportTemplate = buildExportTemplate(selectedTemplate, queryResults);
        console.log("Export template:", exportTemplate);
        const result = await window.electronAPI.executeExport(directory, [exportTemplate]); // Assuming executeExport expects an array
        if (result.success) {
            alert(result.message);
        } else {
            throw new Error(result.message);
        }
    } catch (error) {
        console.error("Export failed:", error);
        alert(`Export failed: ${error.toString()}`);
    }
};

 
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
        <TemplateEditor
          folder={selectedTemplate}
          onUpdateTemplate={handleTemplateEdit}
        />
          <div className="flex justify-end space-x-2 mt-4">
            <button 
              onClick={() => setSelectedTemplate(null)}
              className="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition duration-150 ease-in-out shadow"
            >
              Back
            </button>

            <button 
              // Assuming there's a function to handle export
              onClick={() => handleExport()}
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
