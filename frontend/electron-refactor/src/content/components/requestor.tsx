import React from "react";
import SelectedRegion from "../lib/region";
import Query from "../lib/query";
import axios from 'axios';

const BACKEND_URL = "http://127.0.0.1:3000"
const POLL_INTERVAL = 300;

interface RequestorProps {
    selectedRegion: SelectedRegion,
    query: Query,
    setQuery: React.Dispatch<React.SetStateAction<Query>>
}


async function checkResults() {

}

async function makeQuery(selectedRegion: SelectedRegion, query: Query, setQuery: React.Dispatch<React.SetStateAction<Query>>) {
    axios.get(`${BACKEND_URL}/search`, {
        params: {
            top_left_lat: selectedRegion.region.northWest.lat,
            top_left_long: selectedRegion.region.northWest.long,
            bottom_right_lat: selectedRegion.region.southEast.lat,
            bottom_right_long: selectedRegion.region.southEast.long
        }
    }).then((response) => {
        if (response.status == 200 && response.data && response.data.token) {
            setQuery(query.nextState());
            axios.get(`${BACKEND_URL}/results`, {
                params: {
                    uuid: response.data.token
                },
            }).then((results_response) => {
                if (results_response.status == 200 && response.data) {
                    if (results_response.data.status == "Waiting") {
                        console.log('Query still waiting to begin!');
                    }
                }
            })
        } else {
            console.log(response);
        }
    } )
}

export default function Requestor(props: RequestorProps) {

    return (
        <div className="requestor">
            <button onClick={() => {makeQuery(props.selectedRegion, props.query, props.setQuery)}}>Make Request</button>
        </div>
    )
}