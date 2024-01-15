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
    <div className="flex-grow p-4 rounded mx-6 my-2 bg-[#353744] minwidth xl:flex-grow-0">
      <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded min-w-full md:min-w-0 md:min-w-200px">
        <span className="font-bold">Query History</span>
      </div>
      <div className="flex flex-col-reverse gap-1 mt-2">
        {queryHistory.map((query, index) => (
        <div className="bg-[#525461] text-white font-bold py-2 px-4 rounded w-full md:w-auto md:min-w-200px hover:bg-[#526071] flex gap-4 text-left justify-between"
             key={index} onClick={() => handleCoordinateClick(query)}>
          <span className="coordinate"><b>North West:</b>   <span className="darker"> {query.northWest.lat.toFixed(8)}, {query.northWest.lng.toFixed(8)}</span></span>
          <span className="coordinate"><b>South East:</b>   <span className="darker"> {query.southEast.lat.toFixed(8)}, {query.southEast.lng.toFixed(8)}</span></span>
        </div>
        ))}
        {/*<button onClick={clearCoordinates}>Clear List</button>*/}
      </div>
    </div>
  );
};

export default QueryHistory;
