import React, { useState } from 'react';
import './App.css';
import MapComponent from './components/MapComponent';
import MapBoundingBoxForm from './api/FlaskEndpoints';

function App() {
    const [boundingBox, setBoundingBox] = useState(null);

    return (
        <div className="App">
            <MapComponent setBoundingBox={setBoundingBox} boundingBox={boundingBox} />
            <MapBoundingBoxForm boundingBox={boundingBox} />
        </div>
    );
}

export default App;
