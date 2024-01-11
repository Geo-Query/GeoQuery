import React, { useState } from 'react';
import "./QueryHistory.css";

const QueryHistory = () => {
  const [coordinates, setCoordinates] = useState([]);

  // adds a random set of coordinates to the list
  const addCoordinates = () => {
    const newCoordinates = {
      // SW
      SWlatitude: Math.random() * 90,
      SWlongitude: Math.random() * 180,
      // NE
      NElatitude: Math.random() * 90,
      NElongitude: Math.random() * 180,
    };

    // Adds the new coordinate to the list
    setCoordinates([newCoordinates, ...coordinates]);
  };

  // clears the list
  const clearCoordinates = () => {
    // Clearing the coordinates list
    setCoordinates([]);
  };

  return (
    <div className= "log">
      <h2>Query Log</h2>
      <ul>
        {coordinates.map((coord, index) => (
          <li key={index}>
            SWLat: {coord.SWlatitude.toFixed(4)},
            SWLng: {coord.SWlongitude.toFixed(4)},
            NELat: {coord.NElatitude.toFixed(4)},
            NELng: {coord.NElongitude.toFixed(4)}
          </li>
        ))}
      </ul>

      <button onClick={addCoordinates}>Add Coords</button>
      <button onClick={clearCoordinates}>Clear List</button>
    </div>
  );
};

export default QueryHistory;
