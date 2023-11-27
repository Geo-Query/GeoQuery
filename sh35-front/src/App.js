import React, { useState } from 'react';
import './App.css';
import MapComponent from './components/MapComponent';
import MapBoundingBoxForm from './api/FlaskEndpoints';
import LongLatBoxes from "./components/InputBoxes";

function App() {
    const [boundingBox, setBoundingBox] = useState(null);

    return (
        <div className="App">
            <MapComponent setBoundingBox={setBoundingBox} boundingBox={boundingBox} />
            <MapBoundingBoxForm boundingBox={boundingBox} />
            <LongLatBoxes />
        </div>
    );
}

export default App;
