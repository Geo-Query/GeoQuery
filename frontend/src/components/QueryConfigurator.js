import React from "react";
import MapBoundingBoxForm from "../api/FlaskEndpoints";

const QueryConfigurator = ({boundingBox, setBoundingBox, redraw, queryHistory, setQueryHistory}) => {

  const handleManualInput = (n, v) => {
    switch (n) {
      case 1:
        setBoundingBox({
          northWest: {lat: v.target.value, lng: boundingBox.northWest.lng},
          southEast: boundingBox.southEast
        });
        break;
      case 2:
        setBoundingBox({
          northWest: {lat: boundingBox.northWest.lat, lng: v.target.value},
          southEast: boundingBox.southEast
        });
        break;
      case 3:
        setBoundingBox({
          northWest: boundingBox.northWest,
          southEast: {lat: v.target.value, lng: boundingBox.southEast.lng}
        });
        break;
      case 4:
        setBoundingBox({
          northWest: boundingBox.northWest,
          southEast: {lat: boundingBox.southEast.lat, lng: v.target.value}
        });
        break;
    }
  }
  return (

    <div className="flex-grow p-6 border-2 border-white rounded-xl mx-6 my-2">
      <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded-xl border-2 border-white min-w-full md:min-w-0 md:min-w-200px">
        <span className="font-bold">Query Configuration :: Click away from input to see preview.</span>
      </div>
      <div className="flex flex-col my-2 gap-2 text-white font-bold p-1">
        <div className="flex items-center">
          <h1 style={{width: "15%"}}>North West: </h1>
          <div className="flex gap-1 w-full">
            <input
              type="text"
              placeholder="Latitude"
              value={boundingBox.northWest.lat || ""}
              onChange={(v) => handleManualInput(1, v)}
              onBlur={() => {redraw()}}
              className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            />
            <input
              type="text"
              placeholder="Longitude"
              value={boundingBox.northWest.lng || ""}
              onChange={(v) => handleManualInput(2, v)}
              onBlur={redraw}
              className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            />
          </div>
        </div>
        <div className="flex items-center">
          <h1 style={{width: "15%"}} >South East: </h1>

          <div className="flex gap-1 w-full">
            <input
              type="text"
              placeholder="Latitude"
              value={boundingBox.southEast.lat || ""}
              onChange={(v) => handleManualInput(3, v)}
              onBlur={redraw}
              className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            />
            <input
              type="text"
              placeholder="Longitude"
              value={boundingBox.southEast.lng || ""}
              onChange={(v) => handleManualInput(4, v)}
              onBlur={redraw}
              className="w-full text-black px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            />
          </div>
        </div>
        <MapBoundingBoxForm boundingBox={boundingBox} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
      </div>
    </div>
  );
};

export default QueryConfigurator;
