# front-end Code Documentation

## Project Overview
This is a React-based frontend application providing a user interface for geographical information querying and display. The application interacts with a Flask backend service to offer rich data processing and visualization capabilities.

### `App.js`
- **methods/components**
  - `boundingBox`: A state variable initialized with useState. It is an object containing northWest and southEast coordinates, each with lat (latitude) and long (longitude) properties. 
  ({
    northWest: {lat: float, lng: float},
    southEast: {lat: float, lng: float}
  })
  This state is used to manage the bounding box information in the component.
  - `setBoundingBox`:Setter funtion of `boundingBox`. Within the `Mapcomponent.js`, when a user interacts with the map, the `setBoundingBox` function can be called to update the boundingBox value.
- **Return Element**
  - Pass the initialized `setBoundingBox` function and the boundingBox object as props to the `MapComponent2` component (in `MapComponent.js`), allowing it to utilize these properties and functions for map-related operations and display within the `MapComponent2` component.

### `index.js`
- **methods/components**
  - root : Created by ReactDOM.createRoot, targets the DOM element with the ID 'root'. It serves as the root of the React application.
- **Return Element**: Renders the App component into the root DOM node.

### `MapComponent.js`
- **methods/components**
  - `const setQueryHistory = (v) =>`:It manage `queryHistory`using the `useState`hook. It also creates a function called `setQueryHistoryWrapped` for updating the value of `queryHistory`.It saves the `v` parameter as a JSON string in the browser's local storage using the `localStorage.setItem` method. This allows the query history to be persistently stored on the client-side.It calls the `setQueryHistoryWrapped` function, passing the `v` parameter to it, thereby updating the state value of `queryHistory`.

  - `useEffect`:This function initializes an interactive map using Leaflet, allowing users to draw rectangles on it. When users complete drawing a rectangle, the function captures its coordinates and updates the `boundingBox` property of the component. Any previously drawn rectangles on the map are cleared.

  - `redraw`:Redraws a rectangle on the map using coordinates from `boundingBox`.
- **Return Element**
  - The component returns a map and sections for query history and configurator. 

### `QueryConfigurator.js`
- **methods/components**
  - `const handleManualInput = (n, v) =>`:
    - Parameters: 
    `n `:(Number,1(lat of north-west)|2(lng of north-west)|3(lat of south-east)|4(lng of south-east)), 
    `v `:(The longitude or latitude values entered by the user in the input box.).
    - Explanation: Updates the boundingBox state based on manual input in different fields (latitude and longitude for north-west and south-east points).
- **Return Element**
  - returns a input fields for manually setting the north-west and south-east points of the bounding box and a MapBoundingBoxForm component that handles further query configurations.

### `QueryHistory.js`
- **methods/components**
  - `queryHistory` : A state variable to store the Array of `boundingBox`, with details about each query.
  - `clearCoordinates`: Clears the query history by setting it to an empty array using `setQueryHistory`(which is used only to update the queryHistory state in `QueryHistory.js`).
- **Return Element**
  - returns a list of query history items and a button to clear the history. 

### `InputBoxes.js`
- **Return Element**
  - The component returns two input elements. Each input box allows text input, with one box having a placeholder "Lat" for latitude and the other "Lng" for longitude.

### `FlaskEndpoints.js`
- **methods/components**
  - `const startPolling = (currentToken) =>`
    Parameters: currentToken (UUID)
    Explanation: Create a timed polling operation that calls the `fetchResults` function every 1 second (1000 milliseconds) and records the identifier of the timer.
  - `const fetchResults = async (currentToken, intervalId) =>`
    Parameters: currentToken (UUID), intervalId (Number).
    Explanation: Polls a backend service for results and updates the modal content.
  - `const sendCoordinates = async (data) =>`
    Parameters: data(QueryRegion:{
    "top_left_long": float,
    "top_left_lat": float,
    "bottom_right_long": float,
    "bottom_right_lat": float
  })
    Explanation: sendCoordinates method is responsible for sending the geographical coordinates of the bounding box to a backend server for processing.Upon receiving a response from the server, it initiates further actions, which include starting a polling process to check for results (`startPolling`) and handling any immediate response data.
  - `const validateAndSanitizeData = (box) =>`
    Parameters: box (`boundingBox`),
    Explanation: The function accepts an `boundingBox` input and performs validation and sanitization on the provided data, then converts it into the `QueryRegion` format.
  - `const handleSubmit = async (event) =>`
    Explanation:Prevents default form submission behavior.  Resets any existing error messages. Validates and sanitizes the `boundingBox` data. If validation fails, sets an error message and stops further execution. Appends the current `boundingBox` to `queryHistory`. Calls `sendCoordinates` with sanitized data.
- **Return Element**
  A form for submitting coordinates and a modal for displaying status and results.

### `Modal.js`
- **methods/components**
  - `isOpen (Boolean)`: Determines if the modal is open.
- **Return Element**
  - The component conditionally renders a modal based on `isOpen`. If true, it displays a styled modal.


## Link to other documentation
[Link to backtend File](./backend_documentation.md)
[Link to web-api File](./web-api_documentation.md)
