import React, { useState, useEffect } from "react";
import SelectedRegion, { checkFormat, checkValid, validateAndConformCoordinate } from "../utils/region";
import QueryHistory from "../utils/queryhistory";
import Toastify from "toastify-js";

interface ConfiguratorProps {
    selectedRegion: SelectedRegion,
    setSelectedRegion: React.Dispatch<React.SetStateAction<SelectedRegion>>,
    queryHistory: QueryHistory,
    setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>
}

export default function Configurator(props: ConfiguratorProps) {
    const [northWestLatInput, setNWLatInput] = useState(undefined);
    const [northWestLongInput, setNWLongInput] = useState(undefined);
    const [southEastLatInput, setSELatInput] = useState(undefined);
    const [southEastLongInput, setSELongInput] = useState(undefined);


    const [northWestLatReal, setNorthWestLatReal] = useState(undefined);
    const [northWestLongReal, setNorthWestLongReal] = useState(undefined);
    const [southEastLatReal, setSouthEastLatReal] = useState(undefined);
    const [southEastLongReal, setSouthEastLongReal] = useState(undefined);


    useEffect(() => {
        console.log("FIRED!");
        if (southEastLongReal && southEastLatReal && northWestLongReal && northWestLatReal) {
            props.setSelectedRegion({
                enteredManually: true,
                region: {
                    northWest: {
                        lat: northWestLatReal,
                        long: northWestLongReal
                    },
                    southEast: {
                        lat: southEastLatReal,
                        long: southEastLongReal
                    }
                }
            })
        }

    }, [northWestLatReal, northWestLongReal, southEastLatReal, southEastLongReal]);

    // if box drawn plot, if entered leave box
    useEffect(() => {
        if (props.selectedRegion) {
            if (!props.selectedRegion.enteredManually) {
                setNWLatInput(props.selectedRegion.region?.northWest.lat.toString());
                setNWLongInput(props.selectedRegion.region?.northWest.long.toString());
                setSELatInput(props.selectedRegion.region?.southEast.lat.toString());
                setSELongInput(props.selectedRegion.region?.southEast.long.toString());
                setNorthWestLatReal(props.selectedRegion.region?.northWest.lat);
                setNorthWestLongReal(props.selectedRegion.region?.northWest.long);
                setSouthEastLatReal(props.selectedRegion.region?.southEast.lat);
                setSouthEastLongReal(props.selectedRegion.region?.southEast.long);
            }
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
                        value={northWestLatInput || ''}
                        onChange={(e) => {
                            setNWLatInput(e.target.value)
                        }}
                        onBlur={(e) => {
                            console.log("BLURRED?!?!?");
                            const converted = checkFormat(e.target.value);
                            if (converted) {
                                if (checkValid('lat', converted).isValid) {
                                    setNorthWestLatReal(converted);
                                    setNWLatInput(converted.toString());
                                } else {
                                    Toastify({
                                        text: "Failed to parse coordinate input!",
                                        duration: 3000,
                                        gravity: "bottom", // `top` or `bottom`
                                        position: "right", // `left`, `center` or `right`
                                        stopOnFocus: true, // Prevents dismissing of toast on hover
                                        style: {
                                            background: "red",
                                        },
                                    }).showToast();
                                }
                            } else {
                                Toastify({
                                    text: "Failed to parse coordinate input!",
                                    duration: 3000,
                                    gravity: "bottom", // `top` or `bottom`
                                    position: "right", // `left`, `center` or `right`
                                    stopOnFocus: true, // Prevents dismissing of toast on hover
                                    style: {
                                        background: "red",
                                    },
                                }).showToast();
                            }
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="text"
                        value={northWestLongInput || ''}
                        onChange={(e) => {
                            setNWLongInput(e.target.value)
                        }}
                        onBlur={(e) => {
                            console.log("BLURRED!");
                            const converted = checkFormat(e.target.value);
                            if (converted) {
                                if (checkValid('long', converted).isValid) {
                                    console.log("BLURRED!")
                                    setNorthWestLongReal(converted);
                                    setNWLongInput(converted.toString());
                                } else {
                                    Toastify({
                                        text: "Failed to parse coordinate input!",
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
                        value={southEastLatInput || ''}
                        onChange={(e) => {
                            setSELatInput(e.target.value)
                        }}
                        onBlur={(e) => {
                            console.log("BLURRED!")

                            const converted = checkFormat(e.target.value);
                            if (converted) {
                                if (checkValid('lat', converted).isValid) {
                                    setSouthEastLatReal(converted);
                                    setSELatInput(converted.toString());
                                } else {
                                    Toastify({
                                        text: "Failed to parse coordinate input!",
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
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="text"
                        value={southEastLongInput || ''}
                        onChange={(e) => {
                            setSELongInput(e.target.value)
                        }}
                        onBlur={(e) => {
                            console.log("BLURRED!")

                            const converted = checkFormat(e.target.value);
                            if (converted) {
                                if (checkValid('long', converted).isValid) {
                                    setSouthEastLongReal(converted);
                                    setSELongInput(converted.toString());
                                } else {
                                    Toastify({
                                        text: "Failed to parse coordinate input!",
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
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
            </div>
        </div>
    );
}