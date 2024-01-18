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
        "status": "Waiting" | "Processing" | "Complete",
        "pagination": {
          "count": "total number of results",
          "current_page": "number of results in the current page",
          "per_page": "maximum number of results per page"
        },
        "results": [
          {
            "file": {
              "path": "/filepath"
            },
            "region":{
              "top_left": (float64,float64)
              "bottom_right": (float64,float64)
            }
          }, ...
        ]
      }
      ```
    - **Description**: The response includes the status of the search, pagination information, and the actual search results.
  - **Error Response**:
    - **Code**: `404 NOT FOUND`
    - **Content**: `"Task not found"`
    - **Description**: Returned when no task is found for the provided UUID.



## Link to other documentation
[Link to backtend File](./backend_documentation.md)
[Link to frontend File](./frontend_documentation.md)