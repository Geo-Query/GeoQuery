import React, { useState } from "react";

const LongLatBoxes = ({}) => {

  return (
    <div className="flex space-x-2">
      <input
        type="text"
        placeholder="Lat"
        value=""
        className=""
      />
      <input
        type="text"
        placeholder="Lng"
        value=""
        className="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
      />
    </div>
  );
};

export default LongLatBoxes;
