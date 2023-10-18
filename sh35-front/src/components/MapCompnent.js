import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'; // Import Leaflet CSS for proper map styling

const defaultCenter = [40.7128, -74.0060]; // Latitude and Longitude for New York City
const defaultZoom = 13; // Default zoom level

const tileURL = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";

function MapComponent() {
    return (
        <MapContainer center={defaultCenter} zoom={defaultZoom} style={{ width: '100%', height: '600px' }}>
            <TileLayer url={tileURL} />
            {/* You can also add a Marker to indicate a specific location */}
            <Marker position={defaultCenter}>
                <Popup>
                    New York City
                </Popup>
            </Marker>
        </MapContainer>
    );
}

export default MapComponent;
