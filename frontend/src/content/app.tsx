import React from "react";
import {useState} from "react";
import ReactDom from "react-dom";
import SelectedRegion from "./utils/region";
import Map from "./components/map";
import "./css/main.css";
import Configurator from "./components/configurator";
import QueryHistory, {loadQueryHistory} from "./utils/queryhistory";
import {QueryState} from "./utils/query";
import Requestor from "./components/requestor";
import History from "./components/history";

function App() {
    const [selectedRegion, setSelectedRegionWrapped] = useState(undefined);

    const setSelectedRegion = (v) => {
        console.log("SETTER CALLED!");
        console.trace();
        setSelectedRegionWrapped(v);
    }
    const [queryHistory, setQueryHistoryWrapped] = useState(loadQueryHistory());
    const [queryState, setQueryState] = useState<QueryState>(QueryState.BUILDING);

    const setQueryHistory = (value: QueryHistory) => {
        value.saveToStorage();
        setQueryHistoryWrapped(value);
    }
      

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

                
                {/* <button onClick={handleExport} className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
                    Start Export
                </button> */}
            </div>
            </div>
        </div>
    );
}

ReactDom.render(<App />, document.body);