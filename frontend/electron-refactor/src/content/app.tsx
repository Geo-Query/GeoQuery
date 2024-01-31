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
        <div className="App">
            <Map selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion}/>
            <div className="bottom">
                <div className="history">
                    Content Here Must Be Respected!
                </div>
                <div className="control">
                    <div className="control-header"><span>Query Configuration</span></div>
                    <Configurator selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>

                    <Requestor selectedRegion={selectedRegion} queryState={queryState} setQueryState={setQueryState}/>
                </div>
            </div>
        </div>
    );
}

ReactDom.render(<App />, document.body);