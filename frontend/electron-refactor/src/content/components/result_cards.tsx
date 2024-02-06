import React, { useEffect, useState } from "react";
import {QueryResult, QueryState, queryString} from "../lib/query";
import { ModalProps } from "./modal";

//populated result cards that will be take an array of results to go on into modal.tsx for the results

export default function ResultCards(response: ModalProps) {
  const [visibleResults, setVisibleResults] = useState<Array<QueryResult>>([]);
  const [removedResults, setRemovedResults] = useState<Array<string>>([]); // Track removed results by unique identifier


    // Update visibleResults when props.results changes
  useEffect(() => {
    // Filter out results that are already visible or have been removed
    const newUniqueResults = response.results.filter(result => 
        !visibleResults.some(visibleResult => visibleResult.file.path === result.file.path) &&
        !removedResults.includes(result.file.path)
    );
    setVisibleResults(prevResults => [...prevResults, ...newUniqueResults]);
  }, [response.results, removedResults]);
    

  const removeResult = (index: number) => {
    const resultToRemove = visibleResults[index];
    setRemovedResults(prevRemoved => [...prevRemoved, resultToRemove.file.path]);
    setVisibleResults(prevResults => prevResults.filter((_, i) => i !== index));
  };


  const undoLastRemove = () => {
    const lastRemovedPath = removedResults.pop();
    if (lastRemovedPath) {
        const resultToUndo = response.results.find(result => result.file.path === lastRemovedPath);
        if (resultToUndo) {
            setVisibleResults(prevResults => [...prevResults, resultToUndo]);
            setRemovedResults([...removedResults]);
        }
    }
  };

  const hasResults = response.results && response.results.length > 0;

  return (
    <div>
        <div className="flex bg-blue-600 py-2 px-4 rounded items-center space-x-2">
          <p className="font-mono">Status:</p>
          <p className="font-mono">{queryString(response.queryState)}</p>

          {/* Shows a loading circle */} 
          {(response.queryState === QueryState.PROCESSING)  && (
            
            <div className="animate-spin inline-block w-6 h-6 border-[3px] border-current border-t-transparent text-green-500 rounded-full dark:text-green-500" role="status" aria-label="loading">
              <span className="sr-only">Loading...</span>
            </div>
            )}
            {/* Shows a green tick if complete */}
            {response.queryState === QueryState.COMPLETE && (
              <div>
                <svg className="w-8 h-8 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" xmlns="http://www.w3.org/2000/svg">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="sr-only">Complete</span>
              </div>
            )}
          </div>
          <br></br>
          {visibleResults.length > 0 ? (
          <div>
            <h3 className="text-lg font-bold text-white mb-4">Results:</h3>
            <div className="grid grid-cols-1 gap-4 overflow-x-auto">
              {visibleResults.map((result, index) => (
                <div key={index} className="bg-[#525461] rounded-lg shadow-lg p-4 transition-transform duration-300 ease-in-out hover:scale-105 hover:bg-[#526071] hover:shadow-xl">
                  <div className="flex justify-between items-center mb-2">
                    <span className="font-mono text-sm text-white">
                      {result.file.path}
                    </span>
                    <span className="text-xs font-semibold text-white">
                      {result.type}
                    </span>
                    <button onClick={() => removeResult(index)} className="text-red-500">
                        Remove
                    </button>
                  </div>
                  <div className="border-t border-gray-200 my-2"></div>
                  {result.region ? ( // Check if region is defined
                    <div className="text-sm text-white">
                      <div className="flex justify-between items-center">
                        <span className="font-bold">NW:</span>
                        <span className="font-mono">{result.region.top_left[1]}, {result.region.top_left[0]}</span>
                      </div>
                      <div className="flex justify-between items-center">
                        <span className="font-bold">SE:</span>
                        <span className="font-mono">{result.region.bottom_right[1]}, {result.region.bottom_right[0]}</span>
                      </div>
                    </div>
                  ) : (
                    <p className="text-gray-200">Region data unavailable.</p>
                  )}
                </div>
              ))}
            </div>
            {removedResults.length > 0 && (
                <button onClick={undoLastRemove} className="mt-4 bg-blue-500 text-white px-4 py-2 rounded">
                    Undo Last Remove
                </button>
            )}
          </div>
        ) : (
          <p className="text-gray-200">No results found.</p>
        )}
  
          
        </div>
  );
}


