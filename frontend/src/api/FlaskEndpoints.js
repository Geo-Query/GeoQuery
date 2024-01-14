import React, { useState, useEffect } from 'react';
import axios from "axios";
import { RUST_BACKEND_URL } from '../config'; // Adjust the path as necessary
import Modal from '../components/Modal.js';

const MapBoundingBoxForm = ({ boundingBox, queryHistory, setQueryHistory }) => {
  const [errorMessage, setErrorMessage] = useState("");
  const [pollingIntervalId, setPollingIntervalId] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modalContent, setModalContent] = useState("");

  const fetchResults = async (currentToken, intervalId) => {
    try {
      const response = await axios.get(`${RUST_BACKEND_URL}/results`, { params: { uuid: currentToken } });

      console.log(response);

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


      const modalContent = generateModalContent(response);
      setModalContent(modalContent);

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
        startPolling(response.data.token); // Pass the token directly
        setIsModalOpen(true);
        setModalContent("Sending coordinates and starting polling...");
      }

    } catch (error) {
      console.error("There was an error sending the coordinates", error);
      setErrorMessage("Failed to send coordinates: " + error.message);
    }
  };


  //Generates the modal content and populates it with the current status of search and results
  const generateModalContent = (responseData) => {
    if (!responseData) return <p>No data available.</p>;
    console.log(responseData);
    return (
      <div>
        <div className="flex items-center space-x-2">
          <p>Status:</p>
          <p>{responseData.data.status}</p>

          {/* Shows a loading circle */}
          {(responseData.data.status === "Processing" || responseData.data.status === "Waiting")  && (
            
          <div class="animate-spin inline-block w-6 h-6 border-[3px] border-current border-t-transparent text-green-500 rounded-full dark:text-green-500" role="status" aria-label="loading">
            <span className="sr-only">Loading...</span>
          </div>
          )}
          {/* Shows a green tick if complete */}
          {responseData.data.status === "Complete" && (
            <div>
              <svg className="w-8 h-8 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              <span className="sr-only">Complete</span>
            </div>
          )}
        </div>
        <br></br>
        
        {/* Populates a list with the results as they come in - update in next sprint */}
        {responseData.data.results && responseData.data.results.length > 0 ? (
          <div>
            <h3>Results:</h3>
            <ul className="list-disc list-inside">
              {responseData.data.results.map((result, index) => (
                <li key={index} className="mb-1">NW: {result.region.top_left[0]},{result.region.top_left[1]}; SE: {result.region.bottom_right[0]}, {result.region.bottom_right[1]} :: {result.file.path}</li>
              ))}
            </ul>
          </div>
          
        ) : (
          <p>No results found.</p>
        )}
        
      </div>
    );
  };
  

  const closeModal = () => {
    setIsModalOpen(false);
  };

  // Checks constraints of long and lat verifying and returning in format for the rust server to understand
  const validateAndSanitizeData = (box) => {
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
    <>
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

      <Modal isOpen={isModalOpen} onClose={closeModal}>
        <div className="relative p-6 flex-auto">
            {modalContent}
        </div>
      </Modal>
    </>
  );
};

export default MapBoundingBoxForm;
