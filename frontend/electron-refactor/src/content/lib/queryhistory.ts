import {Region} from "./region";

const HISTORY_KEY = "queryHistory";

export default class QueryHistory {
    queries: Region[]

    constructor(history: Region[]) {
           this.queries = history;
    }
    saveToStorage() {
        localStorage.setItem(HISTORY_KEY, JSON.stringify(this.queries));
        console.log("Stored LocalHistory!");
    }

    add(region: Region) {
        this.queries.push(region);
        return this;
    }
}

export function loadQueryHistory(): QueryHistory {
    const storedHistory = localStorage.getItem("queryHistory");
    if (!storedHistory) {
        return new QueryHistory([]);
    } else {
        try {
            return new QueryHistory(JSON.parse(storedHistory));
        } catch (e) {
            return new QueryHistory([]);
        }
    }
}