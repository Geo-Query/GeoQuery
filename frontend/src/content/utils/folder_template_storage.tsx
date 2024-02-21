import React from "react";

interface Folder {
    id: number;
    name: string;
    tags: string;
    children: Folder[];
  }

export interface FolderTemplate{
    folderName: string;
    folderCreationDate: string;
    folder: Folder;
}

  
  const FOLDER_TEMPLATES_KEY = "folderTemplates";
  
  class FolderTemplatesStorage {
    templates: FolderTemplate[];

    constructor(templates: FolderTemplate[] = []) {
        this.templates = templates;
    }

    saveToStorage() {
        localStorage.setItem(FOLDER_TEMPLATES_KEY, JSON.stringify(this.templates));
        console.log("Stored Folder Templates!");
    }

    add(template: FolderTemplate) {
        this.templates.push(template);
        this.saveToStorage();
    }

    delete(templateId: number) {
      // First ensure that every template has a folder object and that folder object has an id
      this.templates = this.templates.filter(template => template.folder && typeof template.folder.id !== 'undefined' && template.folder.id !== templateId);
      this.saveToStorage();
  }
  

    static loadFromStorage(): FolderTemplatesStorage {
        const storedTemplates = localStorage.getItem(FOLDER_TEMPLATES_KEY);
        if (storedTemplates) {
            try {
                const templates: FolderTemplate[] = JSON.parse(storedTemplates);
                return new FolderTemplatesStorage(templates);
            } catch (e) {
                console.error("Failed to parse folder templates from storage", e);
                return new FolderTemplatesStorage();
            }
        } else {
            return new FolderTemplatesStorage();
        }
    }
}

export default FolderTemplatesStorage;