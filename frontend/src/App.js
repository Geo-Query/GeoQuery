import React, {useRef, useState} from "react";
import "./k.css";
import MapComponent2 from "./components/MapComponent";

function App() {
  const [boundingBox, setBoundingBox] = useState({
    northWest: {lat: "", long: ""},
    southEast: {lat: "", long: ""}
  });

  return (
    <div className="App bg-thales-dark min-h-screen flex flex-col">

      {/* Map Area */}
      <div className="swn">
        <span className="py-2 text-white font-bold text-xl md:text-4xl">
          Geo<span className="text-green-500">Query</span>
         </span>
      </div>
      <div>
        <MapComponent2 setBoundingBox={setBoundingBox} boundingBox={boundingBox}/>
      </div>


    </div>
  );
}

export default App;

