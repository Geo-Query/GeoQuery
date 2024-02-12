// mockExportProcess.ts
export const mockExportProcess = async (setExportStatus: Function) => {
    console.log("mockExportProcess started");
    setExportStatus({ isLoading: true, message: 'Mock export started...' });
    await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate export delay
    const isSuccess = true; // Adjust for testing
    console.log("mockExportProcess setting success:", isSuccess);
    if (isSuccess) {
      setExportStatus({ isLoading: false, isSuccess: true, message: 'Mock export completed successfully!' });
    } else {
      setExportStatus({ isLoading: false, isSuccess: false, message: 'Mock export failed.' });
    }
  };