import React, {useState} from "react";
import ReactDom from "react-dom";
import SelectedRegion from "./lib/region";
import Map from "./components/map";
import "./css/main.css";
import Configurator from "./components/configurator";
import {loadQueryHistory} from "./lib/queryhistory";
import {QueryState} from "./lib/query";
import Requestor from "./components/requestor";

function App() {
    const [selectedRegion, setSelectedRegion] = useState(new SelectedRegion())
    const [queryHistory, setQueryHistory] = useState(loadQueryHistory());
    const [queryState, setQueryState] = useState<QueryState>(QueryState.BUILDING);

    return (
        <div className="App bg-thales-dark min-h-screen flex flex-col text-left">
            <Map selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion}/>
            <div className="flex flex-wrap justify-between items-stretch">
                <div className="flex-grow p-4 rounded mx-6 my-2 bg-[#353744] minwidth xl:flex-grow-0">
                    Content Here Must Be Respected!
                </div>
                <div className="flex-grow p-4 rounded mx-6 my-2 bg-[#353744] gap-4">
                    <div className="bg-blue-600 text-white font-bold py-2 px-4 rounded min-w-full md:min-w-0 md:min-w-200px mb-5 text-left"><span>Query Configuration</span></div>
                    <Configurator selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
                    <Requestor selectedRegion={selectedRegion} queryState={queryState} setQueryState={setQueryState}/>
                </div>
            </div>
        </div>
    );
}

ReactDom.render(<App />, document.body);