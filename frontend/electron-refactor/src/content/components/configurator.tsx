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
                      lat: northWestLat,
                      long: northWestLong
                    },
                    southEast: {
                        lat: southEastLat,
                        long: southEastLong
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
        <div className="configurator">
            <input
                type="number"
                value={northWestLat || ''}
                onChange={(e) => {
                    setNWLat(validateAndConformCoordinate(e.target.value))
                }}
            />
            <input
                type="number"
                value={northWestLong || ''}
                onChange={(e) => {
                    setNWLong(validateAndConformCoordinate(e.target.value))
                }}
            />
            <input
                type="number"
                value={southEastLat || ''}
                onChange={(e) => {
                    setSELat(validateAndConformCoordinate(e.target.value))
                }}
            />
            <input
                type="number"
                value={southEastLong || ''}
                onChange={(e) => {
                    setSELong(validateAndConformCoordinate(e.target.value))
                }}
            />
        </div>
    );
}