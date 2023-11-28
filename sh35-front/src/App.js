import React, { useState } from "react";
import "./App.css";
import MapComponent from "./components/MapComponent";
import MapBoundingBoxForm from "./api/FlaskEndpoints";
import LongLatBoxes from "./components/InputBoxes";
/* Added this stuff, might be scuffed - Anwar */
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

function App() {
  const [boundingBox, setBoundingBox] = useState(null);

  return (
    <div className="App bg-thales-dark min-h-screen flex flex-col">
      {/* Toolbar */}
      <div className="toolbar flex flex-wrap justify-between items-center py-4 px-6">
        <span className="py-2 text-white font-bold text-xl md:text-4xl">
          Geo<span className="text-green-500">Query</span>
        </span>

        <div className="flex flex-wrap items-center space-x-2 md:space-x-4 mt-4 md:mt-0">
          <LongLatBoxes />
          <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded-xl border-2 border-white min-w-full md:min-w-0 md:min-w-200px">
            <span className="font-bold">SW:</span>
            {boundingBox
              ? `${boundingBox.bottomLeft.lat.toFixed(
                  4
                )}, ${boundingBox.bottomLeft.lng.toFixed(4)}`
              : "Lat, Lng"}
          </div>
          <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded-xl border-2 border-white min-w-full md:min-w-0 md:min-w-200px">
            <span className="font-bold">NE:</span>
            {boundingBox
              ? `${boundingBox.topRight.lat.toFixed(
                  4
                )}, ${boundingBox.topRight.lng.toFixed(4)}`
              : "Lat, Lng"}
          </div>
        </div>

        <div className="w-full md:w-auto mt-4 md:mt-0">
          <MapBoundingBoxForm boundingBox={boundingBox} />
          <ToastContainer />
        </div>
      </div>

      {/* Map Area */}
      <div className="flex-grow p-6 border-2 border-white rounded-xl mx-6 my-4">
        <MapComponent
          setBoundingBox={setBoundingBox}
          boundingBox={boundingBox}
        />
      </div>
    </div>
  );
}

export default App;

