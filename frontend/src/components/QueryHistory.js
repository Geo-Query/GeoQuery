import React, { useState } from 'react';
import "./QueryHistory.css";

const QueryHistory = () => {
  const [coordinates, setCoordinates] = useState([]);

  // adds a random set of coordinates to the list

    //TODO: Link to the bounding box instead of the random data

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
    <div className="flex-grow p-6 border-2 border-white rounded-xl mx-6 my-2">
      <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded-xl border-2 border-white min-w-full md:min-w-0 md:min-w-200px">
        <span className="font-bold">Query History</span>
      </div>
      <div className= "log">
        <ul>
          {coordinates.map((coord, index) => (
            <li key={index}>
              NWLat: {coord.NWlatitude.toFixed(4)},
              NWLng: {coord.NWlongitude.toFixed(4)},
              SELat: {coord.SElatitude.toFixed(4)},
              SELng: {coord.SElongitude.toFixed(4)}
            </li>
          ))}
        </ul>

        <button onClick={addCoordinates}>Add Coords</button>
        <button onClick={clearCoordinates}>Clear List</button>
      </div>
    </div>
  );
};

export default QueryHistory;
