import React, { useState, useEffect } from "react";
import { MapContainer, TileLayer, useMap } from "react-leaflet";
import L from "leaflet";
import "leaflet-draw";
import "leaflet/dist/leaflet.css";
import "leaflet-draw/dist/leaflet.draw.css";
import "./MapComponent.css";
import leafletImage from "leaflet-image";

const MapComponent = ({ setBoundingBox, boundingBox }) => {
  const defaultCenter = [55.869829854, -4.28583219];
  const defaultZoom = 10;
  const [mousePosition, setMousePosition] = useState(null);
  const [staticImageUrl, setStaticImageUrl] = useState(null); // New state for static image URL

  const goBackToDynamicMap = () => {
    setStaticImageUrl(null);
  };

  const captureMap = (map, boundingBox) => {
    leafletImage(map, (err, canvas) => {
      const img = document.createElement("img");
      const dimensions = map.getSize();
      img.width = dimensions.x;
      img.height = dimensions.y;
      img.src = canvas.toDataURL();

      img.onload = () => {
        // Ensure the image is loaded before drawing the bounding box
        const ctx = canvas.getContext("2d");
        ctx.strokeStyle = "red";
        ctx.lineWidth = 5;
        const bottomLeft = map.latLngToContainerPoint(boundingBox.bottomLeft);
        const topRight = map.latLngToContainerPoint(boundingBox.topRight);
        ctx.strokeRect(
          bottomLeft.x,
          topRight.y,
          topRight.x - bottomLeft.x,
          bottomLeft.y - topRight.y
        );

        setStaticImageUrl(canvas.toDataURL()); // Update the state with the new image URL
      };
    });
  };

  const MousePositionControl = () => {
    const map = useMap();

    useEffect(() => {
      const updateMousePosition = (event) => {
        setMousePosition({
          latlng: event.latlng,
          containerPoint: event.containerPoint,
        });
      };

      const hideMousePosition = () => {
        setMousePosition(null);
      };

      map.on("mousemove", updateMousePosition);
      map.on("mouseout", hideMousePosition);

      return () => {
        map.off("mousemove", updateMousePosition);
        map.off("mouseout", hideMousePosition);
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
        if (e.layerType === "rectangle") {
          const bounds = layer.getBounds();
          // const southWest = bounds.getSouthWest();
          // const northEast = bounds.getNorthEast();

          // setBoundingBox(newBoundingBox);
          // captureMap(map, newBoundingBox);

          const southWest = bounds.getSouthWest(); // Bottom-left
          const northEast = bounds.getNorthEast(); // Top-right

          const newBoundingBox = {
            bottomLeft: { lat: southWest.lat, lng: southWest.lng },
            topRight: { lat: northEast.lat, lng: northEast.lng },
          };

          setBoundingBox(newBoundingBox);

          captureMap(map, newBoundingBox);
        }
      });

      return () => {
        map.off(L.Draw.Event.CREATED);
      };
    }, [map]);

    return null;
  };

  /*
  Once box is drawn there is a small error in the accuracy of the coordinates. It is minimal (off by .0003) places. Fix after demo.
  */
  const updateMousePositionStatic = (event) => {
    const rect = event.target.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    if (boundingBox) {
      // const latRange = boundingBox.topRight.lat - boundingBox.bottomLeft.lng;
      // const lngRange = boundingBox.topRight.lat - boundingBox.bottomLeft.lng;
      const latRange = boundingBox.topRight.lat - boundingBox.bottomLeft.lat;
      const lngRange = boundingBox.topRight.lng - boundingBox.bottomLeft.lng;
      const lat = boundingBox.topRight.lat - (y / rect.height) * latRange;
      const lng = boundingBox.bottomLeft.lng + (x / rect.width) * lngRange;

      setMousePosition({
        latlng: { lat, lng },
        containerPoint: { x, y },
      });
    }
  };

  return (
    <div style={{ position: "relative" }}>
      {staticImageUrl ? ( // Conditionally display static image or dynamic map
        <div>
          <img
            src={staticImageUrl}
            alt="Static Map"
            className="map-container"
            onMouseMove={updateMousePositionStatic}
          />
          <button
            className="bg-blue-600 hover:bg-blue-700 text-white font-bold m-4 py-2 px-4 rounded"
            onClick={goBackToDynamicMap}
          >
            Back to Dynamic Map
          </button>
        </div>
      ) : (
        <MapContainer
          center={defaultCenter}
          zoom={defaultZoom}
          className="map-container"
        >
          <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
          <AddDrawControl />
          <MousePositionControl />
        </MapContainer>
      )}

      {/* {boundingBox && (
        <div className="bounding-box">
          <p>Bottom Left: {boundingBox.bottomLeft.join(", ")}</p>
          <p>Top Right: {boundingBox.topRight.join(", ")}</p>
          <p>Bottom Left: {boundingBox.bottomLeft.lat}, {boundingBox.bottomLeft.lng}</p>
          <p>Top Right: {boundingBox.topRight.lat}, {boundingBox.topRight.lng}</p>
        </div>
      )} */}

      {mousePosition && (
        <div
          className="mouse-position-tooltip"
          style={{
            top: mousePosition.containerPoint.y + 15, // Offset by 15 pixels to avoid covering the cursor
            left: mousePosition.containerPoint.x + 15,
          }}
        >
          Lat: {mousePosition.latlng.lat.toFixed(4)}, Lng:{" "}
          {mousePosition.latlng.lng.toFixed(4)}
        </div>
      )}
    </div>
  );
};

export default MapComponent;
