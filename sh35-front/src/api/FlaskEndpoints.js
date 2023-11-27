import React, { useState } from "react";
import axios from "axios";

// Bottom Left: 55.830215663223996, -4.3157386779785165
// Top Right: 55.88898321305664, -4.199352264404298

const MapBoundingBoxForm = ({ boundingBox }) => {
  const handleSubmit = async (event) => {
    event.preventDefault();

    console.log(boundingBox);
    if (!boundingBox) {
      console.error("No bounding box to send");
      return;
    }

    try {
      const response = await axios.post(
        "http://localhost:8080/api/post-coordinates",
        boundingBox
      );
      console.log(response.data);
    } catch (error) {
      console.error("There was an error sending the coordinates", error);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <button
        className="export-btn bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-xl border-2 border-white"
        type="submit"
      >
        Send Coordinates
      </button>
    </form>
  );
};

export default MapBoundingBoxForm;
