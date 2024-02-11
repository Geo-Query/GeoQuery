// // Import necessary modules and components
// import React, { useState} from 'react';
// import { readTemplateFile } from '../lib/templateReader';
// import { createFolderStructure} from '../lib/folderStructureService';
// import { copyFilesToStructure } from '../lib/fileCopierService';
// import UserFeedback from './ExportUserFeedback';
// import { useExportState } from './ExportWizardState';
// import { selectDirectory } from '../lib/ipcService';

// // Update ExportWizardProps to include jsonFilePath
// interface ExportWizardProps {
//   isOpen: boolean;
//   onClose: () => void;
//   jsonFilePath: string; // Path to the JSON structure file
// }

// const ExportWizard: React.FC<ExportWizardProps> = ({ onClose, jsonFilePath }) => {
//   const { exportStatus, setExportStatus } = useExportState({
//     isLoading: false,
//     isSuccess: undefined,
//     message: '',
//   });
//   const [directoryPath, setDirectoryPath] = useState<string | null>(null);

//   const handleSelectRootDirectory = async () => {
//     const directory = await selectDirectory();
//     if (directory) {
//       setDirectoryPath(directory);
//       setExportStatus({ isLoading: false, message: `Directory selected: ${directory}` });
//     }
//   };

//   const handleExport = async () => {
//     if (!directoryPath) {
//       setExportStatus({ isLoading: false, message: "Please select a directory first." });
//       return;
//     }
//     setExportStatus({ isLoading: true, message: "Exporting..." });
//     try {
//       const template = await readTemplateFile(jsonFilePath);
//       await createFolderStructure(template, directoryPath);
//       await copyFilesToStructure(template, directoryPath);
//       setExportStatus({ isLoading: false, isSuccess: true, message: "Export completed successfully!" });
//     } catch (error) {
//       setExportStatus({ isLoading: false, isSuccess: false, message: `Export failed: ${error.message}` });
//     }
//   };

//   return (
//     <div className="flex flex-col space-y-4">
//       <UserFeedback status={exportStatus} />
//       <button
//         className="btn-primary"
//         onClick={handleSelectRootDirectory}
//         disabled={exportStatus.isLoading}
//       >
//         Select Export Root
//       </button>
//       <button
//         className="btn-secondary"
//         onClick={handleExport}
//         disabled={!directoryPath || exportStatus.isLoading}
//       >
//         EXPORT
//       </button>
//       <button
//         className="btn-cancel"
//         onClick={onClose}
//       >
//         Close
//       </button>
//     </div>
//   );
// };

// export default ExportWizard;

//! deprecated due to IPC implementation - resuse as needed.