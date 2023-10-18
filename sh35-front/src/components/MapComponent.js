// import React from 'react';
// import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
// import 'leaflet/dist/leaflet.css'; // Import Leaflet CSS for proper map styling


// const defaultCenter = [40.7128, -74.0060]; // Latitude and Longitude for New York City
// const defaultZoom = 13; // Default zoom level

// const tileURL = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";

// function MapComponent() {
//     return (
//         <MapContainer center={defaultCenter} zoom={defaultZoom} style={{ width: '100%', height: '600px' }}>
//             <TileLayer url={tileURL} />
//             {/* You can also add a Marker to indicate a specific location */}
//             <Marker position={defaultCenter}>
//                 <Popup>
//                     New York City
//                 </Popup>
//             </Marker>
//         </MapContainer>
//     );
// }

// export default MapComponent;

import React, { useEffect } from 'react';
import { MapContainer, TileLayer, useMap } from 'react-leaflet';
import L from 'leaflet';
import 'leaflet-draw';
import 'leaflet/dist/leaflet.css';
import 'leaflet-draw/dist/leaflet.draw.css';

const MapComponent = () => {
    const defaultCenter = [40.7128, -74.0060]; // New York City coordinates
    const defaultZoom = 13;

    const AddDrawControl = () => {
        const map = useMap();

        useEffect(() => {
            const drawControl = new L.Control.Draw({
                draw: {
                    rectangle: true,
                    polyline: false,
                    circle: false,
                    circlemarker: false,
                    marker: false,
                    polygon: false
                }
            });
            map.addControl(drawControl);

            map.on(L.Draw.Event.CREATED, (e) => {
                console.log("Shape created!", e);
            });

            return () => {
                map.off(L.Draw.Event.CREATED);
            };
        }, [map]);

        return null;
    };

    return (
        <MapContainer center={defaultCenter} zoom={defaultZoom} style={{ width: '100%', height: '600px' }}>
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
            <AddDrawControl />
        </MapContainer>
    );
}

export default MapComponent;