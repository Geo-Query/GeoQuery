import React from 'react';
import "./QueryHistory.css";


const QueryHistory = ({queryHistory, setQueryHistory, setBoundingBox}) => {

  // Function to clear the list of coordinates
  const clearCoordinates = () => {
    setQueryHistory([]);
  };

  // Function to query from the log
  const handleCoordinateClick = (query) => {
    setBoundingBox(query);
  }

  return (
    <div className="flex-grow p-6 border-2 border-white rounded-xl mx-6 my-2">
      <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded-xl border-2 border-white min-w-full md:min-w-0 md:min-w-200px">
        <span className="font-bold">Query History</span>
      </div>
      <div className="log">
        <ul>
          {queryHistory.map((query, index) => (
            <li key={index} onClick={() => handleCoordinateClick(query)}>
              NW: {query.northWest.lat.toFixed(4)}, {query.northWest.lng.toFixed(4)},
              SE: {query.southEast.lat.toFixed(4)}, {query.southEast.lng.toFixed(4)}
            </li>
          ))}
        </ul>

        <button onClick={clearCoordinates}>Clear List</button>
      </div>
    </div>
  );
};

export default QueryHistory;
