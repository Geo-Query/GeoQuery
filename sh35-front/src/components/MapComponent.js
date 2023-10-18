import React, { useState , useEffect } from "react";
import { MapContainer, TileLayer, useMap } from "react-leaflet";
import L from "leaflet";
import "leaflet-draw";
import "leaflet/dist/leaflet.css";
import "leaflet-draw/dist/leaflet.draw.css";

const MapComponent = () => {
  const defaultCenter = [55.869829854, -4.28583219]; // New York City coordinates
  const defaultZoom = 10;
  const [boundingBox, setBoundingBox] = useState(null);
  const [mousePosition, setMousePosition] = useState(null);

    const MousePositionControl = () => {
        const map = useMap();

        useEffect(() => {
            const updateMousePosition = (event) => {
                setMousePosition({
                    latlng: event.latlng,
                    containerPoint: event.containerPoint
                });
            };

            map.on('mousemove', updateMousePosition);

            return () => {
                map.off('mousemove', updateMousePosition);
            };
        }, [map]);

        return null;
    };


  const AddDrawControl = () => {
    const map = useMap();

    useEffect(() => {
      if (map && !map.drawControl) {
        const drawControl = new L.Control.Draw({
          draw: {
            rectangle: true,
            polyline: false,
            circle: false,
            circlemarker: false,
            marker: false,
            polygon: false,
          },
        });
        map.addControl(drawControl);
        map.drawControl = drawControl;
      }

      map.on(L.Draw.Event.CREATED, (e) => {
        const layer = e.layer;
        if (e.layerType === 'rectangle') {
            const bounds = layer.getBounds();
            const southWest = bounds.getSouthWest(); // Bottom-left
            const northEast = bounds.getNorthEast(); // Top-right
            setBoundingBox({
                bottomLeft: [southWest.lat, southWest.lng],
                topRight: [northEast.lat, northEast.lng]
            });
        }
        console.log("Shape created!", e);
    });

      return () => {
        map.off(L.Draw.Event.CREATED);
      };
    }, [map]);

    return null;
  };

  return (
    <div style={{ position: 'relative' }}>
        <MapContainer
            center={defaultCenter}
            zoom={defaultZoom}
            style={{ width: "100%", height: "600px" }}
        >
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
            <AddDrawControl />
            <MousePositionControl /> {/* Add this line to include the MousePositionControl component */}
        </MapContainer>
        
        {boundingBox && (
            <div>
                <p>Bottom Left: {boundingBox.bottomLeft.join(', ')}</p>
                <p>Top Right: {boundingBox.topRight.join(', ')}</p>
            </div>
        )}
        
        {mousePosition && (
            <div style={{
                position: 'absolute',
                top: mousePosition.containerPoint.y,
                left: mousePosition.containerPoint.x,
                backgroundColor: 'white',
                padding: '5px',
                border: '1px solid black',
                pointerEvents: 'none',
                zIndex: 1000
            }}>
                Lat: {mousePosition.latlng.lat.toFixed(4)}, Lng: {mousePosition.latlng.lng.toFixed(4)}
            </div>
        )}
    </div>
);
};

export default MapComponent;
