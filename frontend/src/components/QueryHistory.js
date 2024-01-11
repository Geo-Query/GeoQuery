import React, { useState } from 'react';
import "./QueryHistory.css";

const QueryHistory = () => {
  const [coordinates, setCoordinates] = useState([]);

  // adds a random set of coordinates to the list
  const addCoordinates = () => {
    const newCoordinates = {
      // SW
      NWlatitude: Math.random() * 90,
      NWlongitude: Math.random() * 180,
      // NE
      SElatitude: Math.random() * 90,
      SElongitude: Math.random() * 180,
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
      <ul>
        {coordinates.map((coord, index) => (
          <li key={index}>
            NWLat: {coord.SWlatitude.toFixed(4)},
            NWLng: {coord.SWlongitude.toFixed(4)},
            SELat: {coord.NElatitude.toFixed(4)},
            SELng: {coord.NElongitude.toFixed(4)}
          </li>
        ))}
      </ul>

      <button onClick={addCoordinates}>Add Coords</button>
      <button onClick={clearCoordinates}>Clear List</button>
    </div>
  );
};

export default QueryHistory;
