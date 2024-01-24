// App.js
import React, { useState } from "react";
import "./k.css";
import MapComponent2 from "./components/MapComponent";
import ExportWizard from "./components/ExportWizard";

function App() {
  const [boundingBox, setBoundingBox] = useState({
    northWest: { lat: "", long: "" },
    southEast: { lat: "", long: "" }
  });

  const [isExportWizardOpen, setExportWizardOpen] = useState(false);
  const [selectedFiles, setSelectedFiles] = useState([]); // This will hold the file paths

  // Dummy function to simulate file selection based on bounding box
  const handleSearch = () => {
    // Ideally, you would have an API call to the backend here
    // For now, let's simulate with dummy data
    const files = ['/path/to/file1', '/path/to/file2'];
    setSelectedFiles(files);
    setExportWizardOpen(true);
  };

  return (
    <div className="App bg-thales-dark min-h-screen flex flex-col">
      {/* Map Area */}
      <div className="swn">
        <span className="py-2 text-white font-bold text-xl md:text-4xl">
          Geo<span className="text-green-500">Query</span>
        </span>
      </div>
      <div>
        <MapComponent2 setBoundingBox={setBoundingBox} boundingBox={boundingBox} />
      </div>
      {/* Temporary button to trigger the export wizard for demonstration */}
      <button
        className="bg-green-500 text-white p-3 rounded"
        onClick={handleSearch}
      >
        Simulate Search and Open Export Wizard
      </button>
      {/* Export Wizard */}
      <ExportWizard
        isOpen={isExportWizardOpen}
        onClose={() => setExportWizardOpen(false)}
        selectedFiles={selectedFiles}
      />
    </div>
  );
}

export default App;
