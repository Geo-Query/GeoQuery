import React, {useState} from "react";
import ReactDom from "react-dom";
import SelectedRegion from "./lib/region";
import Map from "./components/map";
import "./css/main.css";
import Configurator from "./components/configurator";
import {loadQueryHistory} from "./lib/queryhistory";
import Query from "./lib/query";

function App() {
    const [selectedRegion, setSelectedRegion] = useState(new SelectedRegion())
    const [queryHistory, setQueryHistory] = useState(loadQueryHistory());
    const [query, setQuery] = useState(new Query());

    return (
        <div className="App">
            <Map queryState={selectedRegion} setQueryState={setSelectedRegion}></Map>
            <Configurator selectedRegion={selectedRegion} setSelectedRegion={setSelectedRegion} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>

        </div>
    );
}

const rootElement = document.createElement('div');
document.body.innerHTML = "";
document.body.appendChild(rootElement);
ReactDom.render(<App />, rootElement);