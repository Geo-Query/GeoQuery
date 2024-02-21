import React, {useEffect, useRef} from "react";
import L from "leaflet";
import 'leaflet-draw';
import "leaflet/dist/leaflet.css";
import "leaflet-draw/dist/leaflet.draw.css";
import SelectedRegion, {Region} from "../utils/region";
import {QueryState} from "../utils/query";

interface MapProps {
    selectedRegion: SelectedRegion
    setSelectedRegion: React.Dispatch<React.SetStateAction<SelectedRegion>>
}

export default function Map(props: MapProps) {
    const mapRef: React.MutableRefObject<L.Map> = useRef(undefined);
    const mapContainerRef: React.MutableRefObject<HTMLDivElement> = useRef(undefined);
    const drawLayerRef = useRef(undefined);

    // Init map function!
    useEffect(() => {
        const [map, drawLayer] = initialiseLeaflet(mapContainerRef);
        map.on(L.Draw.Event.CREATED, (event) => handleDrawEvent(event, props.setSelectedRegion))
        mapRef.current = map;
        drawLayerRef.current = drawLayer;
    }, []);

    // Init draw function!
    useEffect(() => {
        if (props.selectedRegion.region) {
            draw(props.selectedRegion.region, drawLayerRef, mapRef);
        }
    }, [props.selectedRegion]);

    return (
        <div className="map-container rounded mx-6 my-2 mt-4 p-1">
            <div id="map" className="map" ref={mapContainerRef}></div>
        </div>
    );
}


function handleDrawEvent(
    event: L.LeafletEvent,
    setSelectedRegion: React.Dispatch<React.SetStateAction<SelectedRegion>>,
) {
    const latlngs = event.layer.getLatLngs();
    setSelectedRegion(new SelectedRegion({
        northWest: {
            lat: latlngs[0][1].lat,
            long: latlngs[0][1].lng
        },
        southEast: {
            lat: latlngs[0][3].lat,
            long: latlngs[0][3].lng
        }
    }));
}

function initialiseLeaflet(
    mapContainerRef: React.MutableRefObject<HTMLDivElement>,
): [L.Map, L.FeatureGroup] {
    if (mapContainerRef.current) {
        const map = L.map(mapContainerRef.current);
        map.setView([51.505, -0.9], 10);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap contributors'
        }).addTo(map); // Add OpenStreetMaps!
        const drawLayer = new L.FeatureGroup();
        const drawControls = new L.Control.Draw({
            draw: {
                polyline: false,
                rectangle: {},
                circle: false,
                polygon: false,
                marker: false,
                circlemarker: false,
            },
            edit: {
                featureGroup: drawLayer,
                edit: false,
                remove: false
            }
        });
        map.addLayer(drawLayer);
        map.addControl(drawControls);
        return [map, drawLayer];
    }
}

function draw(region: Region, drawLayerRef: React.MutableRefObject<L.FeatureGroup>, mapRef: React.MutableRefObject<L.Map>) {
    if (drawLayerRef.current) {
        drawLayerRef.current.clearLayers();
        const bounds: L.LatLngBounds = new L.LatLngBounds(
            new L.LatLng(region.northWest.lat, region.northWest.long),
            new L.LatLng(region.southEast.lat, region.southEast.long)
        );
        drawLayerRef.current.addLayer(new L.Rectangle(
            bounds, {
                color: "#3388ff",
                weight: 4,
                opacity: 0.5,
                fillOpacity: 0.2,
                fill: true,
                stroke: true
            }
        ))
        mapRef.current.fitBounds(bounds, {
            padding: [15, 15],
            animate: true
        })
    }
}
