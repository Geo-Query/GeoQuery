// TestExportProcess.tsx
import React, { useEffect } from 'react';
import { useExportState } from '../frontend/electron-refactor/src/content/components/ExportWizardState'; // Adjust path as needed
import { mockExportProcess } from '../frontend/electron-refactor/src/content/testHelpers/mockExportProcess'; // You will create this helper

const TestExportProcess = () => {
    const { exportStatus, setExportStatus } = useExportState({
      isLoading: false,
      isSuccess: undefined,
      message: '',
    });
  
    useEffect(() => {
      console.log("Export status updated:", exportStatus);
    }, [exportStatus]);

  const handleTestExport = async () => {
    // This function will simulate the export process
    console.log("handleTestExport called");
    await mockExportProcess(setExportStatus);
  };

  return (
    <div>
      <button onClick={handleTestExport} className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">Test Export Process</button>
    </div>
  );
};

export default TestExportProcess;
