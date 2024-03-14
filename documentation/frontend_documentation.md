# front-end Code Documentation

## Project Overview
This is a TypeScript-based React frontend application that provides a user interface for querying and displaying geographical information. The application interacts with a Flask backend service to offer robust data processing and visualization capabilities.
## Entry Files
### `index.html`
- The index.html file serves as the HTML entry point for the Electron application.
### `app.tsx`
  - **Components**
    - <App>: The root component of the application.
      - Manages application state using React hooks (useState). 
      - Renders child components such as Map, Configurator, Requestor, and History. 
      - Defines event handlers and callback functions for user interactions.- **Return Element**
  - **Functions**
      `setQueryHistory`: Updates the query history and saves it to storage.
  - **Return Element**: 
    - The root element with the class name "App" containing the entire application's UI components.
## Component Files:
### `configurator.tsx`
- **Components**
  - `ConfiguratorProps`: Defines the props accepted by the component, including `selectedRegion`, `setSelectedRegion`, `queryHistory`, and `setQueryHistory`.
- **Hook**
  - Uses the `useState` hook to define four state variables: `northWestLat`, `northWestLong`, `southEastLat`, and `southEastLong`, representing the latitude and longitude of the northwest and southeast corners, respectively.
  - The first `useEffect` listens for changes in the four coordinate state variables. When all coordinates are set, it updates the state of the selected region.
  - The second `useEffect` listens for changes in the selected region to update the values of the coordinate input boxes.
- **Return Element**: 
  - Returns an element containing input boxes for users to input latitude and longitude of the northwest and southeast corners.
  - The northwest and southeast corners are displayed in two rows, each with a title and two input boxes.

### `DirectoryPicker.tsx`
- **Note**
  - ***The component is marked as deprecated due to the implementation of IPC. However, it can still be reused as needed!***
- **Components**
  - `DirectoryPickerProps`: Defines the props accepted by the component, including onDirectorySelect, which is a function that takes a directory path string as an argument.
- **Functions**
  - `handleDirectorySelect`: This function is invoked when the button is clicked. It uses ipcRenderer.invoke to asynchronously request the user to select a directory. Upon selection, it calls the onDirectorySelect callback with the selected directory path.
- **Return Element**:
  - When clicked, it triggers the handleDirectorySelect function.


### `DMDM-DD.tsx`
- **Functions**
  `dmdmToDecimal`: Converts degrees and minutes to decimal degrees based on the given direction.
- **Return Element**:
  - `number`: The resulting decimal degree value.

### `DMS-DD.tsx`
- **Functions**
  `dmsToDecimal`: Converts degrees, minutes, and seconds to decimal degrees based on the given direction.
- **Return Element**:
  - `number`: The resulting decimal degree value.

### `ExportUserFeedback.tsx`
- **Note**
  - ***The component is marked as deprecated due to the implementation of IPC. However, it can still be reused as needed!***
- **Components**
  - `UserFeedbackProps`: Defines the props accepted by the component, including status, which represents the export state.
    - **Functions**
      `UserFeedback` : This functional component receives the status prop and checks its properties (isLoading, isSuccess, message) to determine the feedback to display to the user.
        - If `isLoading` is true, it displays a loading indicator.
        - If `message` exists, it displays the message with a background color indicating success or failure.
        - If there is no `message`, it doesn't render anything.


### `ExportWizard.tsx`
- **Note**
  - ***The component is marked as deprecated due to the implementation of IPC. However, it can still be reused as needed!***
- **Components**
  - `ExportWizardProps`: Defines the props accepted by the component, including isOpen, onClose, and jsonFilePath, which represents the path to the JSON structure file.
- **Functions**
  - `ExportWizard`: This functional component receives props and manages the export process.
    - Uses the useExportState hook to manage the export status state.
    - Provides methods like handleSelectRootDirectory to select the export root directory and handleExport to trigger the export process.
    - Displays user feedback using the UserFeedback component based on the export status.
    - Renders buttons for selecting the export root directory, triggering the export process, and closing the wizard.


### `ExportWizardState.ts`
- **Note**
  - ***The component is marked as deprecated due to the implementation of IPC. However, it can still be reused as needed!***
- **Hook**
  - `useExportState`: This custom hook initializes and manages the export state using the useState hook.
      - Accepts an initial state of type ExportState.
      - Returns an object containing the exportStatus state and a function setExportStatus to update the export status.
- **Interface**
  - `ExportState`: Defines the structure of the export state, which includes properties like isLoading, isSuccess, and message.
  - `isLoading`: A boolean indicating whether the export process is in progress.
  - `isSuccess`: An optional boolean indicating whether the export process was successful.
  - `message`: An optional string message providing feedback on the export process.



### `expression_builder.tsx`
- **Components**
  - `TagBuilder`: allows users to build tags based on predefined attribute options. Users can add, update, and remove tags dynamically. It consists of input fields and buttons for managing tags.- **Hook**
- **Function/Method**
  - `addTag`: Adds a new tag to the list of tags. It sets the default attribute, operator, value, and logical operator for the new tag.
  - `updateTag`: Updates a tag at a specified index with new attribute, operator, value, or logical operator.
  - `removeTag`: Removes a tag from the list of tags based on its index.
  - `constructExpression`: Constructs an expression string based on the selected tags, operators, and values. It concatenates attributes, operators, and values of each tag to form an expression string.
- **Return Element**:
  - Returns an element containing input boxes for users to input latitude and longitude of the northwest and southeast corners.
  - The northwest and southeast corners are displayed in two rows, each with a title and two input boxes.

### `FileSelector.tsx`
- **Components**
  - `FileSelector`:This component is responsible for displaying a list of selected files. It receives an array of FileHandle objects as props, representing the selected files. If files are selected, it renders each file's name in a separate div element. If no files are selected, it displays a message indicating that no files are selected.
- **Return Element**
  - If there are selected files (selectedFiles.length > 0), it maps through the array of selected files and renders each file's name in a separate div element with the class name "text-white my-2".
  - If there are no selected files (selectedFiles.length === 0), it displays a message "No files selected." in a div element with the class name "text-gray-500".

### `folder_card.tsx`
- **Components**
  - `FolderCard`: This component represents a card for displaying folder information. It receives the following props:
    - `folder`: The folder object containing properties like id, name, tags, and children.
    - `depth`: The depth level of the folder in the folder hierarchy.
    - `onAddChild`: A function to add a child folder.
    - `onDelete`: A function to delete the folder.
    - `onRename`: A function to rename the folder.
    - `onSelect`: A function to select the folder.
    - `onEditTags`: A function to edit the tags associated with the folder.
    - `children`: Any child elements to be rendered inside the folder card.
- **Function/Method**
  - `isValidName`: This function checks whether a folder name is valid. It returns true if the name is valid, false otherwise.
  - `handleRename`: This function handles the renaming of the folder. It validates the new name and triggers the onRename function if the name is valid.
  - `toggleTagBuilder`: This function toggles the visibility of the TagBuilder component.
- **Return Element**
  - It conditionally renders an input field and buttons for renaming the folder if isEditing state is true. Otherwise, it displays the folder name with an option to edit.
  - It displays the folder's tags.
  - It renders buttons for deleting the folder, adding a child folder, and toggling the TagBuilder component.
  - It conditionally renders the TagBuilder component based on the showTagBuilder state.
  - It renders any child elements passed as props.

### `folders_template.tsx`
- **Components**
  - `FoldersTemplate`:This component represents a template manager for folders. It allows users to select, edit, and export folder templates. It receives the following props:
    - results: An array of QueryResult objects representing the results of a query.
- **Function/Method**
  - `handleTemplateEdit`: This function handles the editing of a template. It receives the updated template as an argument.
  - `handleExport`: This function handles the export of data based on the selected template and query results.
  - `exportData`: This asynchronous function performs the export operation. It receives the selected template and query results as arguments.
  - `parseExpression`: This function parses the expression associated with folder tags and returns a function that evaluates it against file tags.
  - `fileMatchesExpression`: This function checks if a file matches the expression associated with a folder.
  - `buildExportTemplate`: This function constructs the export template structure recursively based on the selected template and query results.
  - `onSelectTemplate`: This function handles the selection of a template.
  - `addNewTemplate`: This function adds a new template.
  - `handleDelete`: This function handles the deletion of a template.
- **Return Element**
  - a TemplateEditor component and export buttons if a template is selected
  - a list of existing templates with the option to add a new template if no template is selected.

### `history.tsx`
- **Components**
  - `History`:This component represents a query history panel. It displays a list of past queries along with their coordinates. Users can clear the query history. It receives the following props:
    - `queryHistory`: An instance of QueryHistory representing the history of queries.
    - `setQueryHistory`: A function to update the query history state.
    - `setSelectedRegion`: A function to update the selected region state.
- **Function/Method**
  - `clearHistory`: This function clears the query history. It receives the setQueryHistory function as an argument and updates the query history state to contain an empty array.
- **Return Element**
  - It displays the title "Query History" along with a "Clear" button to clear the query history. Below that, it iterates over the queries in the query history and displays each query's coordinates, allowing users to select a query to set it as the selected region.

### `map.tsx`
- **Components**
  - `Map`:This component renders a Leaflet map where users can draw regions. It receives the following props:
    - `selectedRegion`: An instance of SelectedRegion representing the currently selected region.
    - `setSelectedRegion`: A function to update the selected region state.
- **Function/Method**
  - `handleDrawEvent`: This function handles the Leaflet draw event triggered when a region is drawn on the map. It extracts the coordinates of the drawn region and updates the selected region state accordingly.
  - `initialiseLeaflet`: This function initializes the Leaflet map and draw controls. It sets up the map view, adds a tile layer from OpenStreetMap, configures draw controls to allow drawing rectangles, and returns the map and draw layer.
  - `draw`: This function draws the selected region on the map using Leaflet. It clears the previous layers, creates a rectangle based on the region's coordinates, adds it to the draw layer, and adjusts the map view to fit the region.- **Return Element**
- **Return Element**
  - an id of "map" and a className of "map". The map is rendered within the mapContainerRef div using Leaflet.


### `modal.tsx`
- **Components**
  - `Modal`:This component represents a modal dialog used for displaying query progress, query results, and folder template editing. It receives the following props:
    - `queryState`: The current state of the query (e.g., `WAITING`, `PROCESSING`, `COMPLETE`).
    - `results`: An array of `QueryResult` objects representing the query results.
    - `setQueryState`: A function to update the query state.
- **Function/Method**
  - `handleClose`: This function handles the close button click event on the modal. It sets the query state to BUILDING.
- **Return Element**
  - The return element is a modal dialog containing different content based on the query state. It displays the export wizard title, query progress or results, and buttons for continuing or closing the modal. Additionally, it includes a backdrop element for overlay effects.

### `query_progress.tsx`
- **Components**
  - `QueryProgress`:It receives the following props:
    - `queryState`: The current state of the query (e.g., WAITING, PROCESSING, COMPLETE).
- **Return Element**
  - representing the query progress component.

### `requestor.tsx`
- **Components**
  - `Requestor`:It is responsible for initiating and managing queries to the backend server. It includes functionalities for making requests based on the selected region, polling for query results, handling changes in query state, and displaying notifications on failed or successful requests.
- **Function/Method**
  - `arbitraryFailure`:This function is used to display a failure notification.
  - `pollQuery`:This function is responsible for polling the backend to retrieve query results and updating the query state based on the response.
  - `isQueryUnique`:This function checks if the query region is unique in the query history. 
  - `makeQuery`:This function initializes a new query based on the selected region.
- **Return Element**
  - If the query state is `BUILDING`, it renders a button to initiate the request.
  - If the query state is not `BUILDING`, it renders a disabled button and the Modal component to display the query results.

### `result_cards.tsx`
- **Components**
  - `ResultCards`:It renders the populated result cards that will be displayed in the modal component. It takes an array of query results as input and manages the visibility and removal of results.
    - `response`: A `ModalProps` object containing the query state and results.
- **Function/Method**
  - `removeResult`: This function handles the removal of a result from the visible results list based on its index.
  - `undoLastRemove`: This function undoes the last removal action by adding back the removed result to the visible results list.
- **Return Element**
  - Status indicator showing the current query state.
  - Loading spinner if the query state is PROCESSING.
  - Green tick mark if the query state is COMPLETE.
  - Results list displayed as cards, each showing the paths, type, and region of the result.
  - Button to remove a result from the list.
  - Button to undo the last removal action.
  - Message indicating if no results are found.

### `template_editor.tsx`
- **Components**
  - `TemplateEditor`:It represents a template editor used for editing folder templates. It allows users to add, delete, rename folders, edit folder tags, and save the current template. The component receives the following props:
    - 1folder1: The root folder of the template.
    - `onUpdateTemplate`: A function to update the template when changes occur.
- **Function/Method**
  - `updateFolderTags`: Recursively updates the tags of a folder and its children.
  - `onEditTags`: Handles editing tags for a specific folder.
  - `addChildFolder`: Adds a new child folder to the specified parent folder.
  - `deleteFolder`: Deletes the specified folder from the template.
  - `undoDelete`: Undoes the last folder deletion action.
  - `renameFolder`: Renames the specified folder.
  - `renderFolderCards`: Recursively renders the folder cards for the template.
  - `saveCurrentTemplate`: Saves the current template.- **Return Element**
- **Return Element**
  - Input field for entering the template name.
  - Button to save the current template.
  - Folder cards representing the folder structure of the template.
  - Button to undo the last folder deletion action, if applicable.

## Tool functions and utilities
### `template_editor.tsx`
- **Function/Method**
  - `add(template: FolderTemplate)`: Adds a new folder template to the storage.
  - `delete(templateId: number)`: Deletes a folder template from the storage based on the provided ID.
  - `loadFromStorage()`: FolderTemplatesStorage: Loads folder templates from local storage and returns a new instance of FolderTemplatesStorage.
  - `saveToStorage()`: Saves the current state of the folder templates to local storage.

### `query.ts`
- **Enums**
  - `QueryState`: Defines various states of a query process, including `BUILDING`, `WAITING`, `PROCESSING`, `COMPLETE`, `EDITOR`, `EXPORTING`, `EXPORTED`, and `FAILED`.
- **Function/Method**
  - `queryString(state: QueryState)`: string: Converts a QueryState enum value to its corresponding string representation.
  - `queryStateFromString(state: string)`: Converts a string representation of a query state to its corresponding QueryState enum value.
- **Interfaces**
  - `QueryResultFile`: Describes the file associated with a query result, containing an array of file paths.
  - `QueryResultRegion`: Describes the region associated with a query result, containing coordinates for the top-left and bottom-right corners.
  - `QueryResult`: Represents a query result, containing information about the file, type, region, and tags.
  - `QueryResponse`: Represents a response object from a query, containing status information and an array of query results.



### `queryhistory.ts`
- **Function/Method**
  - `saveToStorage()`: Saves the current query history to the local storage.
  - `add(region: Region)`: Adds a region to the query history.  - **Interfaces**
  - `loadQueryHistory()`: QueryHistory: Loads the query history from local storage and returns a QueryHistory instance. If no history is found, it initializes a new instance with an empty array.


### `region.ts`
- **Function/Method**
  - `validateAndConformCoordinate(initial: string)`: number: Validates and converts a string representation of a coordinate to a number.
  - `checkFormat(initial: string)`: number | undefined: Checks the format of the coordinate string and converts it to decimal degrees if possible. Returns undefined if the format is invalid.
  - `checkValid(coordType: 'lat' | 'long', initial: number): { isValid: boolean; result?: number; error?: string}`: Checks the validity of latitude or longitude coordinates. Returns an object indicating whether the coordinate is valid, along with any error message if applicable.
- **Interfaces**
  - `Coordinate`: Represents geographic coordinates with `latitude` and `longitude`.
  - `Region`: Interface representing a geographical region defined by its northwest and southeast coordinates.
## Link to other documentation
- [Link to backtend File](./backend_documentation.md)
- [Link to web-api File](./web-api_documentation.md)
