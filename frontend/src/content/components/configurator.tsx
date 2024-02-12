import React, {useState, useEffect} from "react";
import SelectedRegion, {validateAndConformCoordinate} from "../lib/region";
import QueryHistory from "../lib/queryhistory";

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

    useEffect(() => {
        if (northWestLat && northWestLong && southEastLong && southEastLat) {
            props.setSelectedRegion({
                region: {
                    northWest: {
                      lat: validateAndConformCoordinate(northWestLat),
                      long: validateAndConformCoordinate(northWestLong)
                    },
                    southEast: {
                        lat: validateAndConformCoordinate(southEastLat),
                        long: validateAndConformCoordinate(southEastLong)
                    }
                }
            })
        }
    }, [northWestLat, northWestLong, southEastLat, southEastLong]);

    useEffect(() => {
        setNWLat(props.selectedRegion.region?.northWest.lat.toString());
        setNWLong(props.selectedRegion.region?.northWest.long.toString());
        setSELat(props.selectedRegion.region?.southEast.lat.toString());
        setSELong(props.selectedRegion.region?.southEast.long.toString());
    }, [props.selectedRegion]);

    return (
        <div className="flex flex-col my-2 text-white font-bold p-1 gap-4 text-left">
            <div className="flex items-center text-left gap-2">
                <h1 style={{width: "15%"}}>North West: </h1>
                <div className="flex gap-3 w-full">
                    <input
                        type="number"
                        value={northWestLat || ''}
                        onChange={(e) => {
                            setNWLat(validateAndConformCoordinate(e.target.value))
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="number"
                        value={northWestLong || ''}
                        onChange={(e) => {
                            setNWLong(validateAndConformCoordinate(e.target.value))
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
            </div>
            <div className="flex items-center text-left gap-2">
                <h1 style={{width: "15%"}}>South East: </h1>
                <div className="flex gap-3 w-full">

                    <input
                        type="number"
                        value={southEastLat || ''}
                        onChange={(e) => {
                            setSELat(validateAndConformCoordinate(e.target.value))
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                        type="number"
                        value={southEastLong || ''}
                        onChange={(e) => {
                            setSELong(validateAndConformCoordinate(e.target.value))
                        }}
                        className="w-full px-4 py-2 text-black border border-gray-300 rounded-md shadow-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                    />
                </div>
            </div>
        </div>
    );
}