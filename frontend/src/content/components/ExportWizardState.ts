// //ExportWizardState.ts
// import { useState } from 'react';

// export interface ExportState {
//   isLoading: boolean;
//   isSuccess?: boolean;
//   message?: string;
// }

// export function useExportState(initialState: ExportState) {
//   const [exportStatus, setExportStatus] = useState<ExportState>(initialState);

//   return {
//     exportStatus,
//     setExportStatus,
//   };
// }

//! deprecated due to IPC implementation - resuse as needed.