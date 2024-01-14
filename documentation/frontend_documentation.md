# front-end Code Documentation

## Project Overview
This is a React-based frontend application providing a user interface for geographical information querying and display. The application interacts with a Flask backend service to offer rich data processing and visualization capabilities.

## Components

### `App.js`
- **Purpose**: The main application component integrating sub-components and managing routing.
- **Interface**:
  - **Props**: N/A (as it's likely the root component)
  - **State**: Descriptions of any state variables.

### `App.css`
- **Purpose**: Contains the global styling for the entire application.
- **Details**:
  - Defines styles that are common across various components, such as fonts, colors, and basic layout properties.
  - Can also include CSS resets or global styles like body margins and padding.

### `index.js`
- **Purpose**: Serves as the entry point of the React application.
- **Interface**: N/A
  - Primarily responsible for rendering the `App` component into the root DOM node.
  - Might include setup for providers like Redux store, Context API, or Router if used.

### `MapComponent.js` / `MapComponent.css`
- **Purpose**: Provides map display functionalities.
- **Interface**:
  - **Props**: 
    - `locationData`: Object containing data to be displayed on the map.
  - **State**: 
    - `mapZoom`: Number representing the zoom level of the map.
    - `markers`: Array of marker objects to be displayed.

### `QueryConfigurator.js`
- **Purpose**: Configures and submits queries.
- **Interface**:
  - **Props**: 
    - `onSubmit`: Function to handle query submission.
  - **State**: 
    - `queryParams`: Object representing user input for query parameters.

### `QueryHistory.js` / `QueryHistory.css`
- **Purpose**: Displays the user's query history.
- **Interface**:
  - **Props**: 
    - `historyData`: Array of objects representing past queries.
  - **State**: N/A

### `InputBoxes.js`
- **Purpose**: Collects user's query parameters.
- **Interface**:
  - **Props**: 
    - `onChange`: Function to handle changes in input fields.
  - **State**: 
    - `inputValue`: String representing user input.

### `FlaskEndpoints.js`
- **Purpose**: Manages interactions with the backend Flask API.
- **Interface**: N/A (utility file)

### `reportWebVitals.js`
- **Purpose**: Monitors application performance.
- **Interface**: N/A (utility file)

### `setupTests.js` / `App.test.js`
- **Purpose**: Sets up testing configuration and defines test cases.
- **Interface**: N/A (testing file)


## Link to other documentation
[Link to backtend File](./backend_documentation.md)
[Link to web-api File](./web-api_documentation.md)
