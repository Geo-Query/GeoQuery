export enum QueryState {
    BUILDING,
    WAITING,
    PROCESSING,
    COMPLETE,
    FAILED
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
export interface QueryResult {
    file: QueryResultFile
}

export default interface QueryResponse {
    status: string
    results: QueryResult[]
}