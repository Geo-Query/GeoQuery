import React from "react";
import {useState} from "react";
import ReactDom from "react-dom";
import SelectedRegion from "./lib/region";
import Map from "./components/map";
import "./css/main.css";
import Configurator from "./components/configurator";
import QueryHistory, {loadQueryHistory} from "./lib/queryhistory";
import {QueryState} from "./lib/query";
import Requestor from "./components/requestor";
import History from "./components/history";

function App() {
    const [selectedRegion, setSelectedRegion] = useState(new SelectedRegion())
    const [queryHistory, setQueryHistoryWrapped] = useState(loadQueryHistory());
    const [queryState, setQueryState] = useState<QueryState>(QueryState.BUILDING);
    const [, setDirectoryPath] = useState("");

    const setQueryHistory = (value: QueryHistory) => {
        value.saveToStorage();
        setQueryHistoryWrapped(value);
    }

    const handleExport = async () => {
        try {
            const directory = await window.electronAPI.selectDirectory();
            if (!directory) {
                alert("Please select a directory first.");
                return;
            }
            setDirectoryPath(directory);
            console.log(`Directory selected: ${directory}`);
            
            const template = [
    {
      "id": 1,
      "name": "Folder 1",
      "relativePath": "Folder 1",
      "filesContained": ["C:/Users/fkgde/Desktop/20230625_171400.jpg"],
      "children": [
        {
          "id": 2,
          "name": "Folder 2",
          "relativePath": "Folder 1/Folder 2",
          "filesContained": ["C:/Users/fkgde/Desktop/3rd year uni/Systems_Programming/AE1/date.c"],
          "children": []
        },
        {
          "id": 3,
          "name": "Folder 3",
          "relativePath": "Folder 1/Folder 3",
          "filesContained": ["C:/Users/fkgde/Downloads/favicon.png"],
          "children": [
            {
              "id": 4,
              "name": "Folder 4",
              "relativePath": "Folder 1/Folder 3/Folder 4",
              "filesContained": ["C:/Users/fkgde/Videos/Desktop/Desktop 2023.09.15 - 15.43.08.01.mp4"],
              "children": []
            }
          ]
        },
        {
          "id": 6,
          "name": "Folder 6",
          "relativePath": "Folder 1/Folder 6",
          "filesContained": ["C:/Users/fkgde/Documents/Books/Ai in games.pdf"],
          "children": []
        }
      ]
    }
  ];
            
            const result = await window.electronAPI.executeExport(directory, template);
            if (result.success) {
                alert(result.message);
            } else {
                throw new Error(result.message);
            }
        } catch (error) {
            console.error("Export failed:", error);
            alert(`Export failed: ${error.toString()}`);
        }
    };
    
      

    return (
        <div className="App bg-thales-dark min-h-screen flex flex-col text-left">
            <Map selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion}/>
            <div className="flex flex-wrap justify-between">
                <History queryHistory={queryHistory} setQueryHistory={setQueryHistory} setSelectedRegion={setSelectedRegion}/>
                <div className="flex-grow p-4 h-min rounded mx-6 my-2 bg-[#353744] gap-4">
                    <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded min-w-full md:min-w-0 md:min-w-200px mb-5 text-left"><span>Query Configuration</span></div>
                    <Configurator selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
                    <Requestor selectedRegion={selectedRegion} queryState={queryState} setQueryState={setQueryState} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
                </div>
                <div className="flex justify-center p-4">

                
                <button onClick={handleExport} className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
                    Start Export
                </button>
            </div>
            </div>
        </div>
    );
}

ReactDom.render(<App />, document.body);