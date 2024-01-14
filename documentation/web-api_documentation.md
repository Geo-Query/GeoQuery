# API Documentation for Rust Web Service using Axum

This documentation outlines the API endpoints provided by a Rust web service using the Axum framework. The service includes functionalities for indexing, searching, and retrieving results.


---

## Endpoints

### 1. Index Endpoint

- **URL**: `/`
- **Method**: `GET`
- **Description**: This endpoint serves as the root of the API, providing a basic response to indicate that the service is running.
- **Response**:
  - **Content-Type**: `text/plain`
  - **Body**: A static string response, e.g., `"INDEX ROOT"`.

### 2. Search Endpoint

- **URL**: `/search`
- **Method**: `GET`
- **Query Parameters**:
  - **region**: The query region for the search. It should conform to the `QueryRegion` structure.
- **Description**: Initiates a search task based on the provided query parameters.
- **Response**:
  - **Content-Type**: `application/json`
  - **Success Response**:
    - **Code**: `200 OK`
    - **Content**: 
      ```json
      {
        "token": "UUID"
      }
      ```
    - **Description**: The response includes a UUID token representing the search task.
  - **Error Response**:
    - **Code**: `500 INTERNAL SERVER ERROR`
    - **Content**: Error description in text format.
    - **Description**: Returned when there is an internal error in processing the search request.

### 3. Results Endpoint

- **URL**: `/results`
- **Method**: `GET`
- **Query Parameters**:
  - **uuid**: The UUID token of the search task.
- **Description**: Retrieves the results of a search task identified by the provided UUID.
- **Response**:
  - **Content-Type**: `application/json`
  - **Success Response**:
    - **Code**: `200 OK`
    - **Content**: 
      ```json
      {
        "status": "QueryState",
        "pagination": {
          "count": "total number of results",
          "current_page": "number of results in the current page",
          "per_page": "maximum number of results per page"
        },
        "results": "Array of Nodes"
      }
      ```
    - **Description**: The response includes the status of the search, pagination information, and the actual search results.
  - **Error Response**:
    - **Code**: `404 NOT FOUND`
    - **Content**: `"Task not found"`
    - **Description**: Returned when no task is found for the provided UUID.

---

## Models

### Node(index.rs)
- Represents a single search result node.
- **Fields**:
  - `region`: `Region` - The geographic region of the search result.
  - `file`: `Arc<FileMeta>` - File metadata of the search result, shared in a thread-safe manner using Arc.

### QueryState(worker.rs)
- Enum representing the state of a search query.
- **Possible Values**:
  - `Waiting`: The query is awaiting processing.
  - `Processing`: The query is currently being processed.
  - `Complete`: The query has been completed.

### SearchQueryResponse(io.rs)
- Represents the response for a search query.
- **Fields**:
  - `token`: `Uuid` - A UUID representing the search task.

### PaginatedQueryResponse(io.rs)
- Structure for paginated response of search results.
- **Fields**:
  - `status`: `QueryState` - The state of the query.
  - `pagination`: `Pagination` - Contains pagination information.
  - `results`: `Vec<Node>` - An array of search result items.

### Pagination(io.rs)
- Structure for pagination information.
- **Fields**:
  - `count`: `usize` - Total number of results.
  - `current_page`: `usize` - Number of results on the current page.
  - `per_page`: `usize` - Maximum number of results per page.

### ResultQuery(io.rs)
- Structure for querying the results of a specific search task.
- **Fields**:
  - `uuid`: `Uuid` - UUID of the search task.

### QueryRegion(io.rs)
- Represents the area for the search query.
- **Fields**:
  - `top_left_long`: `f64` - Longitude of the top-left corner.
  - `top_left_lat`: `f64` - Latitude of the top-left corner.
  - `bottom_right_long`: `f64` - Longitude of the bottom-right corner.
  - `bottom_right_lat`: `f64` - Latitude of the bottom-right corner.


## Link to other documentation
[Link to backtend File](./backend_documentation.md)
[Link to frontend File](./frontend_documentation.md)