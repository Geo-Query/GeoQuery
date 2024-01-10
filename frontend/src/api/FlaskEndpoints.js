import React, { useState } from 'react';
import axios from "axios";
import { RUST_BACKEND_URL } from '../config'; // Adjust the path as necessary

const MapBoundingBoxForm = ({ boundingBox }) => {
  const [errorMessage, setErrorMessage] = useState("");

  const validateAndSanitizeData = (box) => {
    // Check if the box object is provided and not empty
    if (!box || Object.keys(box).length === 0) {
      return { valid: false, message: "No bounding box provided" };
    }

    // Validate and sanitize longitude and latitude
    const topLeftLong = parseFloat(box.bottomLeft.lng);
    const topLeftLat = parseFloat(box.topRight.lat);
    const bottomRightLong = parseFloat(box.topRight.lng);
    const bottomRightLat = parseFloat(box.bottomLeft.lat);
  
    if ([topLeftLong, topLeftLat, bottomRightLong, bottomRightLat].some(isNaN)) {
      console.log(topLeftLong, topLeftLat, bottomRightLong, bottomRightLat);
      return { valid: false, message: "Invalid coordinate values" };
    }
    
    if (topLeftLong < -180 || topLeftLong > 180 || bottomRightLong < -180 || bottomRightLong > 180) {
      return { valid: false, message: "Longitude values must be between -180 and 180" };
    }

    if (topLeftLat < -90 || topLeftLat > 90 || bottomRightLat < -90 || bottomRightLat > 90) {
      return { valid: false, message: "Latitude values must be between -90 and 90" };
    }

 
    return {
      valid: true,
      data: {
        top_left_long: topLeftLong,
        top_left_lat: topLeftLat,
        bottom_right_long: bottomRightLong,
        bottom_right_lat: bottomRightLat
      }
    };
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    setErrorMessage(""); // Reset error message
    console.log(boundingBox);
    const validationResult = validateAndSanitizeData(boundingBox);
    if (!validationResult.valid) {
      setErrorMessage(validationResult.message); // Set error message
      return;
    }

    try {
      const response = await axios.post(
        RUST_BACKEND_URL + "search",
        validationResult.data
      );
      console.log(response.data);
    } catch (error) {
      console.error("There was an error sending the coordinates", error);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex items-center">
      {errorMessage && (
        <div className="text-white font-bold py-2 px-4 mr-2">
          {errorMessage}
        </div>
      )}
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
