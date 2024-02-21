import React, {useEffect, useState} from "react";
import SelectedRegion, {Region} from "../utils/region";
import {QueryResult, QueryState, queryStateFromString} from "../utils/query";
import axios from 'axios';
import Toastify from 'toastify-js';
import "toastify-js/src/toastify.css";
import Modal from "./modal";
import QueryHistory from "../utils/queryhistory";

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
    setPollCount: React.Dispatch<React.SetStateAction<number>>,
    currentPage: number = 1, // Add currentPage parameter
) {
    if (queryToken) {
        try {
            let shouldContinue = true;
            while (shouldContinue) {
                const resp = await axios.get(`${BACKEND_URL}/results`, {
                    params: {
                        uuid: queryToken,
                        page: currentPage, // Use currentPage in request
                    }
                });

                if (resp.status === 200 && resp.data?.results && resp.data?.status) {
                    const state = queryStateFromString(resp.data.status);
                    const pagination = resp.data.pagination; // Capture pagination info

                    if (resp.data.results) {
                        const build: QueryResult[] = resp.data.results.map((node: any) => {
                            let files: string[] = [];
                            for (const mapTypeKey in node.map) {
                                const mapType = node.map[mapTypeKey];
                                if (typeof mapType === 'object' && mapType !== null) {
                                    for (const fileKey in mapType) {
                                        const filePath = mapType[fileKey];
                                        if (typeof filePath === 'string') {
                                            files.push(filePath);
                                        }
                                    }
                                }
                            }
                            return {
                                file: { paths: files },
                                type: node.metadata.tags.map((tag: [string, string]) => tag.join(': ')).join(', '),
                                region: {
                                    top_left: node.metadata.region.top_left,
                                    bottom_right: node.metadata.region.bottom_right
                                },
                                tags: node.metadata.tags.flatMap((tag: [string, string]) => tag)
                            };
                        }).filter((result: QueryResult) => {
                            const pathsString = result.file.paths.join(',');
                            return !seen.has(pathsString);
                        });

                        build.forEach(result => setSeen(seen.add(result.file.paths.join(','))));
                        setResults(prevResults => [...prevResults, ...build]);
                    }
                    
                    if (state !== queryState) {
                        setQueryState(state);
                    }

                    // Check if we need to request the next page
                    if (pagination && currentPage < pagination.current_page && pagination.count > 0) {
                        currentPage++; // Prepare to request the next page
                    } else {
                        shouldContinue = false; // Stop the loop if we're on the last page or no pagination data
                    }
                } else {
                    console.log("Request failed; or unexpected response!");
                    console.log(resp);
                    arbitraryFailure();
                    setQueryState(QueryState.FAILED);
                    shouldContinue = false; // Stop the loop on failure
                }
            }

            if (queryState === QueryState.WAITING || queryState === QueryState.PROCESSING) {
                setPollCount(pollCount + 1);
            }
        } catch (e) {
            console.log("Request failed; or unexpected response!");
            console.log(e);
            arbitraryFailure();
            setQueryState(QueryState.FAILED);
        }
    }
}

function isQueryUnique(newRegion: Region, queryHistory: QueryHistory): boolean {
    return !queryHistory.queries.some(query =>
        query.northWest.lat === newRegion.northWest.lat &&
        query.northWest.long === newRegion.northWest.long &&
        query.southEast.lat === newRegion.southEast.lat &&
        query.southEast.long === newRegion.southEast.long
    );
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
        // Only add to history if unique
        if (isQueryUnique(selectedRegion.region, queryHistory)) {
            setQueryHistory(queryHistory.add(selectedRegion.region));
        }
        // Proceed with making the query as before
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
            console.error("Request failed; or unexpected response!", e);
            arbitraryFailure();
            setQueryState(QueryState.FAILED);
        }
    } else {
        // Existing no region selected notification
        Toastify({
            text: "No Region Selected!",
            duration: 3000,
            gravity: "bottom",
            position: "right",
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

    console.log("RERENDER!");
    console.log(results);
    console.log(queryToken);


    useEffect(() => {
        setTimeout(() => {pollQuery(props.queryState, props.setQueryState, queryToken, seen, setSeen, results, setResults, pollCount, setPollCount)}, POLL_INTERVAL);
    }, [pollCount]);

    useEffect(() => {
        if (props.queryState === QueryState.BUILDING) {
            setQueryToken(undefined);
            setResults(new Array<QueryResult>());
            setPollCount(0);
            setSeen(new Set<string>())
        }
    }, [props.queryState]);
    useEffect(() => {
        if (props.queryState === QueryState.BUILDING) {
            setQueryToken(undefined);
            setResults(new Array<QueryResult>());
            setPollCount(0);
            setSeen(new Set<string>())
        }
    }, [props.queryState]);

    if (props.queryState == QueryState.BUILDING) {
        return (
            <div className="flex flex-row justify-start">
                <button
                    onClick={() => {
                        makeQuery(props.selectedRegion, props.setQueryState, setQueryToken, pollCount, setPollCount, props.queryHistory, props.setQueryHistory);
                    }}
                    className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                >
                    Make Request
                </button>
                {/* <Modal queryState={props.queryState} results={results}></Modal> */}
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
                <Modal queryState={props.queryState} results={results} setQueryState={props.setQueryState} />

            </div>
        )
    }
}