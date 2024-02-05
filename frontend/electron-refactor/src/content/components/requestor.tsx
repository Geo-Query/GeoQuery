import React, {useEffect, useState} from "react";
import SelectedRegion from "../lib/region";
import {QueryResult, QueryState, queryStateFromString} from "../lib/query";
import axios from 'axios';
import Toastify from 'toastify-js';
import "toastify-js/src/toastify.css";
import Modal from "./modal";
import QueryHistory from "../lib/queryhistory";

const BACKEND_URL = "http://127.0.0.1:42069"
const POLL_INTERVAL = 3000;

interface RequestorProps {
    selectedRegion: SelectedRegion
    queryState: QueryState,
    setQueryState: React.Dispatch<React.SetStateAction<QueryState>>
    queryHistory: QueryHistory,
    setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>
}

function arbitraryFailure() {
    Toastify({
        text: "Request Failed. see Console.",
        duration: 3000,
        gravity: "bottom", // `top` or `bottom`
        position: "right", // `left`, `center` or `right`
        stopOnFocus: true, // Prevents dismissing of toast on hover
        style: {
            background: "red",
        },
    }).showToast();
}
async function pollQuery(
    queryState: QueryState,
    setQueryState: React.Dispatch<React.SetStateAction<QueryState>>,
    queryToken: string,
    seen: Set<string>,
    setSeen: React.Dispatch<React.SetStateAction<Set<string>>>,
    results: Array<QueryResult>,
    setResults: React.Dispatch<React.SetStateAction<Array<QueryResult>>>,
    pollCount: number,
    setPollCount: React.Dispatch<React.SetStateAction<number>>
) {
    if (queryToken) {
        try {
            const resp = await axios.get(`${BACKEND_URL}/results`, {
                params: {
                    uuid: queryToken
                }
            });
            if (resp.status === 200 && resp.data?.results && resp.data?.status) {
                const state = queryStateFromString(resp.data.status);

                if (resp.data.results) {
                    const build = [];
                    for (const result of resp.data.results) {
                        if (!seen.has(result.file.path)) {
                            build.push(result);
                            setSeen(seen.add(result.file.path));
                        }
                    }
                    setResults([...build, ...results]);
                }
                if (state !== queryState) {
                    setQueryState(state);
                }
                if (state === QueryState.WAITING || state === QueryState.PROCESSING) {
                    setPollCount(pollCount + 1);
                }
            } else {
                console.log("Request failed; or unexpected response!");
                console.log(resp);
                arbitraryFailure();
                setQueryState(QueryState.FAILED);
            }
        } catch (e) {
            console.log("Request failed; or unexpected response!");
            console.log(e);
            arbitraryFailure();
            setQueryState(QueryState.FAILED);
        }
    }
}

async function makeQuery(
    selectedRegion: SelectedRegion,
    setQueryState: React.Dispatch<React.SetStateAction<QueryState>>,
    setQueryToken: React.Dispatch<React.SetStateAction<string>>,
    pollCount: number,
    setPollCount: React.Dispatch<React.SetStateAction<number>>,
    queryHistory: QueryHistory,
    setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>
) {
    if (selectedRegion.region) {
        console.log(selectedRegion.region);
        setQueryHistory(queryHistory.add(selectedRegion.region));
        try {
            const resp = await axios.get(`${BACKEND_URL}/search`, {
                params: {
                    top_left_lat: selectedRegion.region.northWest.lat,
                    top_left_long: selectedRegion.region.northWest.long,
                    bottom_right_lat: selectedRegion.region.southEast.lat,
                    bottom_right_long: selectedRegion.region.southEast.long
                }
            });
            if (resp.status == 200 && resp.data?.token) {
                setQueryToken(resp.data.token);
                setQueryState(QueryState.WAITING);
                setPollCount(pollCount + 1);
            } else {
                arbitraryFailure();
                setQueryState(QueryState.FAILED);
            }
        } catch (e) {
            console.log("Request failed; or unexpected response!");
            console.log(e);
            arbitraryFailure();
            setQueryState(QueryState.FAILED);
        }
    } else {
        Toastify({
            text: "No Region Selected!",
            duration: 3000,
            gravity: "bottom", // `top` or `bottom`
            position: "right", // `left`, `center` or `right`
            stopOnFocus: true, // Prevents dismissing of toast on hover
            style: {
                background: "red",
            },
        }).showToast();
    }
}


export default function Requestor(props: RequestorProps) {
    const [queryToken, setQueryToken] = useState<string>(undefined);
    const [seen, setSeen] = useState(new Set<string>());
    const [results, setResults] = useState(new Array<QueryResult>());
    const [pollCount, setPollCount] = useState(0);


    useEffect(() => {
        setTimeout(() => {pollQuery(props.queryState, props.setQueryState, queryToken, seen, setSeen, results, setResults, pollCount, setPollCount)}, POLL_INTERVAL);
    }, [pollCount]);



    if (props.queryState == QueryState.BUILDING) {
        return (
            <div className="flex flex-row justify-start">
                <button
                    onClick={() => {
                        makeQuery(props.selectedRegion, props.setQueryState, setQueryToken, pollCount, setPollCount, props.queryHistory, props.setQueryHistory
                    )}}
                    className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                >
                    Make Request
                </button>
                <Modal queryState={props.queryState} results={results}></Modal>
            </div>
        )
    } else {
        return (
            <div className="flex flex-row justify-start">
                <button
                    disabled={true}
                    className="bg-orange-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                >
                    Make Request
                </button>
                <Modal queryState={props.queryState} results={results}></Modal>
            </div>
        )
    }
}