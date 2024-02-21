import React, { useState, useEffect } from "react";
import SelectedRegion, { checkFormat, checkValid, validateAndConformCoordinate } from "../utils/region";
import QueryHistory from "../utils/queryhistory";

interface ConfiguratorProps {
    selectedRegion: SelectedRegion,
    setSelectedRegion: React.Dispatch<React.SetStateAction<SelectedRegion>>,
    queryHistory: QueryHistory,
    setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>
}

export default function Configurator(props: ConfiguratorProps) {
    const [northWestLat, setNWLat] = useState(undefined);
    const [northWestLong, setNWLong] = useState(undefined);
    const [southEastLat, setSELat] = useState(undefined);
    const [southEastLong, setSELong] = useState(undefined);

    // Once populated, and every change after
    useEffect(() => {

        // validates present coordinates
        if (northWestLat && northWestLong && southEastLat && southEastLong) {
            console.log("Coords present");

            // converts all formats to decimal degrees
            const formattedNorthWestLat = checkFormat(northWestLat);
            const formattedNorthWestLong = checkFormat(northWestLong);
            const formattedSouthEastLat = checkFormat(southEastLat);
            const formattedSouthEastLong = checkFormat(southEastLong);
            console.log("Coords formatted");

            const validNorthWestLat = checkValid("lat", formattedNorthWestLat);
            const validNorthWestLong = checkValid("long", formattedNorthWestLong);
            const validSouthEastLat = checkValid("lat", formattedSouthEastLat);
            const validSouthEastLong = checkValid("long", formattedSouthEastLong);
            console.log("Coords validated");

            console.log(validNorthWestLat)
            console.log(validNorthWestLong)
            console.log(validSouthEastLat)
            console.log(validSouthEastLong)

            if(validNorthWestLat.isValid && validNorthWestLong.isValid && validSouthEastLat.isValid && validSouthEastLong.isValid){
            console.log("Attempting to plot");
                // check valid and convert formats
                // if not valid 'grey out' make request button
                // ACHIEVE BY SHARING SOME STATE BETWEEN REQUESTOR & CONFIGURATOR COMPONENT!
                // ONLY SETSELECTEDREGION if valid.
                props.setSelectedRegion({
                    enteredManually: true,
                    region: {
                        northWest: {
                            lat: validNorthWestLat.result,
                            long: validNorthWestLong.result
                        },
                        southEast: {
                            lat: validSouthEastLat.result,
                            long: validSouthEastLong.result
                        }
                    }
                })
            }
        }
    }, [northWestLat, northWestLong, southEastLat, southEastLong]);

    // if box drawn plot, if entered leave box
    useEffect(() => {
        if(!props.selectedRegion.enteredManually){
            setNWLat(props.selectedRegion.region?.northWest.lat.toString());
            setNWLong(props.selectedRegion.region?.northWest.long.toString());
            setSELat(props.selectedRegion.region?.southEast.lat.toString());
            setSELong(props.selectedRegion.region?.southEast.long.toString());
        }

    }, [props.selectedRegion]);

    // every change
    return (
        <div className="flex flex-col my-2 text-white font-bold p-1 gap-4 text-left">
            <div className="flex items-center text-left gap-2">
                <h1 style={{ width: "15%" }}>North West: </h1>
                <div className="flex gap-3 w-full">
                    <input
                        type="text"
                        value={northWestLat || ''}
                        onChange={(e) => {
                            setNWLat(e.target.value)
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="text"
                        value={northWestLong || ''}
                        onChange={(e) => {
                            setNWLong(e.target.value)
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
            </div>
            <div className="flex items-center text-left gap-2">
                <h1 style={{ width: "15%" }}>South East: </h1>
                <div className="flex gap-3 w-full">

                    <input
                        type="text"
                        value={southEastLat || ''}
                        onChange={(e) => {
                            setSELat(e.target.value)
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="text"
                        value={southEastLong || ''}
                        onChange={(e) => {
                            setSELong(e.target.value)
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
            </div>
        </div>
    );
}