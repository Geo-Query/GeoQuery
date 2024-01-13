import React, { useState } from "react";

const LongLatBoxes = () => {
  const [input1, setInput1] = useState("");
  const [input2, setInput2] = useState("");

  const handleInputChange1 = (e) => {
    setInput1(e.target.value);
  };

  const handleInputChange2 = (e) => {
    setInput2(e.target.value);
  };

  return (
    <div className="flex space-x-2">
      <input
        type="text"
        placeholder="Lat"
        value={input1}
        onChange={handleInputChange1}
        className="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
      />
      <input
        type="text"
        placeholder="Lng"
        value={input2}
        onChange={handleInputChange2}
        className="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
      />
    </div>
  );
};

export default LongLatBoxes;
