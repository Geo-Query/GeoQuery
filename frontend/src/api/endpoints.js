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
        <div className="flex bg-blue-600 py-2 px-4 rounded items-center space-x-2">
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
        
        {responseData.data.results && responseData.data.results.length > 0 ? (
        <div>
          <h3 className="text-lg font-bold text-white mb-4">Results:</h3>
          <div className="grid grid-cols-1 gap-4 overflow-x max-h-40">
            {responseData.data.results.map((result, index) => (
              <div key={index} className="bg-[#525461] rounded-lg shadow-lg p-4 transition-transform duration-300 ease-in-out hover:scale-105 hover:bg-[#526071] hover:shadow-xl">
                <div className="flex justify-between items-center mb-2">
                  <span className="font-mono text-sm text-white">
                    {result.file.path}
                  </span>
                  <span className="text-xs font-semibold text-white">
                    {result.type}
                  </span>
                </div>
                <div className="border-t border- my-2"></div>
                <div className="text-sm text-white">
                  <div className="flex justify-between items-center">
                    <span className="font-bold">NW:</span>
                    <span className="font-mono">{result.metadata.region.top_left[1]}, {result.metadata.region.top_left[0]}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="font-bold">SE:</span>
                    <span className="font-mono">{result.metadata.region.bottom_right[1]}, {result.metadata.region.bottom_right[0]} </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <p className="text-gray-200">No results found.</p>
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
    <form onSubmit={handleSubmit} className="flex items-center mt-1">
      <button
        className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
        type="submit"
      >
        Send Coordinates
      </button>
      {errorMessage && (
        <div className="text-white font-bold py-2 px-4 mr-2">
          Error: {errorMessage}
        </div>
      )}
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
