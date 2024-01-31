import React from "react";
import {QueryResult, QueryState} from "../lib/query";

export interface ModalProps {
    queryState: QueryState,
    results: Array<QueryResult>,
}
export default function Modal(props: ModalProps) {
    return (
        <div className="modal">
            <div className="modal-content">
                {props.results.map((r) => r.file.path + "\n")}
            </div>
        </div>
    )
}