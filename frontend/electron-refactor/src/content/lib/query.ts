export enum QueryState {
    BUILDING, //When drawing bounding box
    WAITING, //Job been sent - Waiting on result
    PROCESSING, //Server is starting to return results
    COMPLETE, //Returned all results
    EDITOR, //Shows template editor
    EXPORTING, //Template has been selected and exporting started
    EXPORTED, //Exporting complete - show success
    FAILED //Shows an error component
}

export function queryString(state: QueryState): string {
    switch (state) {
        case QueryState.BUILDING:
            return "Building";
        case QueryState.WAITING:
            return "Waiting";
        case QueryState.PROCESSING:
            return "Processing";
        case QueryState.COMPLETE:
            return "Complete";
        case QueryState.EDITOR:
            return "Editor";
        case QueryState.EXPORTING:
            return "Exporting";
        case QueryState.EXPORTED:
            return "Exported";
        case QueryState.FAILED:
            return "Failed";
        default:
            return "Unknown State";
    }
}

export function queryStateFromString(state: string) {
    switch (state) {
        case "Waiting":
            return QueryState.WAITING;
        case "Processing":
            return QueryState.PROCESSING;
        case "Complete":
            return QueryState.COMPLETE;
        default:
            throw new Error(`Unexpected QueryState String from Server: ${state}`);
    }
}

interface QueryResultFile {
    path: string
}

interface QueryResultRegion {
    top_left: [number, number]
    bottom_right: [number, number]
}


export interface QueryResult {
    file: QueryResultFile
    type: string
    region: QueryResultRegion
    tags: string[]
}

export default interface QueryResponse {
    status: string
    results: QueryResult[]
}