import React, {useRef, useEffect, useState} from "react";
import L from "leaflet";
import "leaflet-draw";
import "leaflet/dist/leaflet.css";
import "leaflet-draw/dist/leaflet.draw.css";
import "./MapComponent.css";
import LongLatBoxes from "./InputBoxes";
import QueryHistory from "./QueryHistory";
import QueryConfigurator from "./QueryConfigurator";

const MapComponent2 = React.memo(() => {
  let mapRef = useRef(null);
  let mapContainerRef = useRef(null);
  let [boundingBox, setBoundingBox] = useState(null);
  useEffect(() => {draw(boundingBox)}, [boundingBox]);


  // Init Draw Layer Reference!
  let drawLayerRef = useRef(null);

  // Init Query History!
  let existingHistory = localStorage.getItem("queryHistory");
  if (!existingHistory) {
    existingHistory = [];
  } else {
    try {
      existingHistory = JSON.parse(existingHistory);
    } catch (e) {
      console.log(e);
      console.log("Could not parse history, hence resetting!");
      existingHistory = [];
      localStorage.setItem("queryHistory", null);
    }
  }
  let [queryHistory, setQueryHistoryWrapped] = useState(existingHistory);
  const setQueryHistory = (v) => {
    localStorage.setItem("queryHistory", JSON.stringify(v));
    setQueryHistoryWrapped(v);
  }

  const draw = (box) => {
    if (drawLayerRef.current) {
      drawLayerRef.current.clearLayers();
      let bounds = L.latLngBounds(
        L.latLng(boundingBox.northWest.lat, boundingBox.northWest.lng),
        L.latLng(boundingBox.southEast.lat, boundingBox.southEast.lng)
      );
      drawLayerRef.current.addLayer(new L.Rectangle(
        bounds, {color: "#3388ff", weight: 4, opacity: 0.5, fill_opacity: 0.2, fill: true, stroke: true}
      ));
      mapRef.current.fitBounds(bounds, {padding: [15, 15], animate: true});
    } else {
      console.log("Did not draw as no layer!");
    }
  }

  useEffect(() => {
    if (mapContainerRef.current) {
      // Init Map!
      let map = L.map(mapContainerRef.current).setView([51.505, -0.09], 10);
      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '© OpenStreetMap contributors'
      }).addTo(map);

      // Init Draw Layer.
      let drawLayer = new L.featureGroup();
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
      })
      map.addLayer(drawLayer);
      map.addControl(drawControl);
      // Add drawLayer!
      map.on(L.Draw.Event.CREATED, (event) => {
        // Set bounding box (wrapped so as to not redraw)
        setBoundingBox({
          northWest: event.layer.getLatLngs()[0][1],
          southEast: event.layer.getLatLngs()[0][3]
        });
      });

      drawLayerRef.current = drawLayer;
      mapRef.current = map;
    }
  }, []);



  //
  // const redraw = () => {
  //   if (drawLayerRef.current && boundingBox.southEast.lat && boundingBox.southEast.lng && boundingBox.northWest.lat && boundingBox.northWest.lng) {
  //     let rect = ;
  //     drawLayerRef.current.clearLayers();
  //     drawLayerRef.current.addLayer(rect);
  //   }
  // }
  // useEffect(() => {
  //   if (mapRef.current) {
  //     let drawLayer = new L.FeatureGroup();
  //     let drawControl = new L.Control.Draw({
  //
  //     });
  //
  //     let map = L.map(mapRef.current).setView([51.505, -0.09], 10);
  //     L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
  //       maxZoom: 19,
  //       attribution: '© OpenStreetMap contributors'
  //     }).addTo(map);
  //     map.addLayer(drawLayer);
  //     map.addControl(drawControl);
  //
  //     map.on(L.Draw.Event.CREATED, (v) => {
  //       console.log(v);
  //       console.log(v.layer.getLatLngs()) // 0: [0: SW, 1: NW, 2: NE, 3: SE]
  //       setBoundingBox({
  //         northWest: v.layer.getLatLngs()[0][1],
  //         southEast: v.layer.getLatLngs()[0][3]
  //       });
  //       drawLayer.clearLayers();
  //       console.log(v.layer);
  //       drawLayer.addLayer(v.layer);
  //     })
  //     drawLayerRef.current = drawLayer;
  //   }
  // }, []);


  return (
    <div id="map-container" className="foo">
      <div className="flex-grow rounded mx-6 my-2 mt-4 p-1">
        <div id="map" className="map" ref={mapContainerRef}></div>
      </div>
      {/* Query History */}
      <div className="flex flex-wrap justify-between items-stretch">
        <QueryHistory queryHistory={queryHistory} setQueryHistory={setQueryHistory} setBoundingBox={setBoundingBox}/>
        <QueryConfigurator boundingBox={boundingBox} setBoundingBox={setBoundingBox} queryHistory={queryHistory} setQueryHistory={setQueryHistory}/>
      </div>

    </div>
  );
});

export default MapComponent2;
