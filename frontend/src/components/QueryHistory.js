import React, { useState } from 'react';

const QueryHistory = () => {
  const [coordinates, setCoordinates] = useState([]);

  // adds a random set of coordinates to the list
  const addCoordinate = () => {
    const newCoordinate = {
      latitude: Math.random() * 90,
      longitude: Math.random() * 180,
    };

    // Adds the new coordinate to the list
    setCoordinates([newCoordinate, ...coordinates]);
  };

  // clears the list
  const clearCoordinates = () => {
    // Clearing the coordinates list
    setCoordinates([]);
  };


  return (
    <div>
      <h2>Query Log</h2>
      <ul>
        {coordinates.map((coord, index) => (
          <li key={index}>
            Latitude: {coord.latitude.toFixed(4)}, Longitude: {coord.longitude.toFixed(4)}
          </li>
        ))}
      </ul>
        <button onClick={addCoordinate}>Add Coordinate</button>
        <button onClick={clearCoordinates}>Clear List</button>
    </div>
  );
};

export default QueryHistory;
