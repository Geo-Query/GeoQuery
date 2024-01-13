import React, {useRef, useEffect, useState} from "react";
import L from "leaflet";
import "leaflet-draw";
import "leaflet/dist/leaflet.css";
import "leaflet-draw/dist/leaflet.draw.css";
import "./MapComponent.css";
import LongLatBoxes from "./InputBoxes";
import QueryHistory from "./QueryHistory";
import QueryConfigurator from "./QueryConfigurator";

const MapComponent2 = React.memo(({boundingBox, setBoundingBox}) => {
  let mapRef = useRef(null);
  let drawLayerRef = useRef(null);
  let [queryHistory, setQueryHistory] = useState([]);

  const redraw = () => {
    if (drawLayerRef.current && boundingBox.southEast.lat && boundingBox.southEast.lng && boundingBox.northWest.lat && boundingBox.northWest.lng) {
      let rect = new L.Rectangle(
        L.latLngBounds(
          L.latLng(boundingBox.northWest.lat, boundingBox.northWest.lng),
          L.latLng(boundingBox.southEast.lat, boundingBox.southEast.lng)
        ), {color: "#3388ff", weight: 4, opacity: 0.5, fill_opacity: 0.2, fill: true, stroke: true}
      );
      drawLayerRef.current.clearLayers();
      drawLayerRef.current.addLayer(rect);
    }
  }
  useEffect(() => {
    if (mapRef.current) {
      let drawLayer = new L.FeatureGroup();
      let drawControl = new L.Control.Draw({
        draw: {
          position: 'topleft',
          polyline: false,
          rectangle: true,
          circle: false,
          polygon: false,
          marker: false,
          circlemarker: false,

        },
        edit: {
          featureGroup: drawLayer,
          edit: false,
          remove: false,
        }
      });

      let map = L.map(mapRef.current).setView([51.505, -0.09], 10);
      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '© OpenStreetMap contributors'
      }).addTo(map);
      map.addLayer(drawLayer);
      map.addControl(drawControl);

      map.on(L.Draw.Event.CREATED, (v) => {
        console.log(v);
        console.log(v.layer.getLatLngs()) // 0: [0: SW, 1: NW, 2: NE, 3: SE]
        setBoundingBox({
          northWest: v.layer.getLatLngs()[0][1],
          southEast: v.layer.getLatLngs()[0][3]
        });
        drawLayer.clearLayers();
        console.log(v.layer);
        drawLayer.addLayer(v.layer);
      })
      drawLayerRef.current = drawLayer;
    }
  }, []);


  return (
    <div id="map-container" className="foo">
      <div className="flex-grow p-2 border-2 border-white rounded-xl mx-6 my-2">
        <div id="map" className="map" ref={mapRef} style={{height: "600px", width: "100%"}}></div>
      </div>
      {/* Query History */}
      <div className="flex flex-wrap justify-between items-start">
        <QueryHistory queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
        <QueryConfigurator boundingBox={boundingBox} setBoundingBox={setBoundingBox} redraw={redraw} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
      </div>

    </div>
  );
});

export default MapComponent2;
