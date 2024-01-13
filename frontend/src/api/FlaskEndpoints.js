import React, { useState, useEffect } from 'react';
import axios from "axios";
import { RUST_BACKEND_URL } from '../config'; // Adjust the path as necessary

const MapBoundingBoxForm = ({ boundingBox, queryHistory, setQueryHistory }) => {
  const [errorMessage, setErrorMessage] = useState("");
  const [pollingIntervalId, setPollingIntervalId] = useState(null);

  const fetchResults = async (currentToken, intervalId) => {
    try {
      const response = await axios.get(`${RUST_BACKEND_URL}/results`, { params: { uuid: currentToken } });
      console.log(response.data);

      if (response.data.status === "Waiting") {
        console.log("Query waiting to begin")
      }
      if (response.data.status === "Processing") {
        console.log("Query in process")
        console.log("Results:", response.data.results);

      }
      if (response.data.status === "Complete") {
        clearInterval(intervalId); // Stop polling
        console.log("Query complete")
        console.log("Results:", response.data.results);
      }
    } catch (error) {
      console.error("There was an error fetching the results", error);
      clearInterval(intervalId); // Stop polling on error
      setErrorMessage("Failed to fetch results: " + error.message);
    }
  };

  const startPolling = (currentToken) => {
    const intervalId = setInterval(() => fetchResults(currentToken, intervalId), 1000);
    setPollingIntervalId(intervalId);
  };

  const sendCoordinates = async (data) => {
    try {
      const response = await axios.get(
        `${RUST_BACKEND_URL}/search`, { params: data }
      );
      console.log(response.data);

      if (response.data && response.data.token) {
        console.log("Token has been set");
        startPolling(response.data.token); // Pass the token directly
        console.log("Polling started");
      }

    } catch (error) {
      console.error("There was an error sending the coordinates", error);
      setErrorMessage("Failed to send coordinates: " + error.message);
    }
  };

  const validateAndSanitizeData = (box) => {
    // Check if the box object is provided and not empty
    if (!box || Object.keys(box).length === 0) {
      return { valid: false, message: "No bounding box provided" };
    }
    console.log(box);
    let topLeftLong = parseFloat(box.northWest.lng);
    let topLeftLat = parseFloat(box.northWest.lat);
    let bottomRightLong = parseFloat(box.southEast.lng);
    let bottomRightLat = parseFloat(box.southEast.lat);
  
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
    const validationResult = validateAndSanitizeData(boundingBox);

    if (!validationResult.valid) {
      setErrorMessage(validationResult.message); // Set error message
      return;
    }

    setQueryHistory(queryHistory.concat([boundingBox]))
    await sendCoordinates(validationResult.data);
  };


  useEffect(() => {
    // This function will be called when the component unmounts
    return () => {
      if (pollingIntervalId) {
        clearInterval(pollingIntervalId);
      }
    };
  }, [pollingIntervalId]); // The effect depends on pollingIntervalId


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
