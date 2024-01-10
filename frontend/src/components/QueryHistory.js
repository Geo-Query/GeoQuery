import React, { useState } from 'react';

const QueryHistory = () => {
  const [coordinates, setCoordinates] = useState([]);

  const addCoordinate = () => {
    const newCoordinate = {
      latitude: Math.random() * 90,
      longitude: Math.random() * 180,
    };

    // Adding the new coordinate to the list
    setCoordinates([newCoordinate, ...coordinates]);
  };

  return (
    <div>
      <button onClick={addCoordinate}>Add Coordinate</button>
      <ul>
        {coordinates.map((coord, index) => (
          <li key={index}>
            Latitude: {coord.latitude.toFixed(4)}, Longitude: {coord.longitude.toFixed(4)}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default QueryHistory;
