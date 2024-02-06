import { useState } from "react";
import React from "react";
import {QueryResult, QueryState} from "../lib/query";
import Query_progress from "./query_progress";
import QueryProgress from "./query_progress";
import ResultCards from "./result_cards";

export interface ModalProps {
    queryState: QueryState,
    results: Array<QueryResult>,
    onClose: () => void
}
export default function Modal(props: ModalProps) {
     
    let progressBar;

    if (props.queryState !== QueryState.COMPLETE) {
        progressBar = <QueryProgress queryState={props.queryState}/>;
    } else {
        progressBar = <p className="text-red-300">THIS SHOULD NOT BE REACHABLE!</p>;
    }
    const renderContent = () => {
        switch (props.queryState) {
          case QueryState.WAITING:
            return <p className="text-white font-mono">Waiting for results...</p>;
            case QueryState.PROCESSING:
            case QueryState.COMPLETE: // Fall through from PROCESSING to COMPLETE
                return <ResultCards {...props} />; // Pass props correctly
          case QueryState.FAILED:
            return <p>An error occurred.</p>;
          default:
            return <p>Unexpected state: {props.queryState}</p>;
        }
      };
    
      return (
        <>
        <div className="flex justify-center items-center overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none">
            <div className="relative mx-auto">
            <div className="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-thales-dark outline-none focus:outline-none min-w-[1200px] h-[800px] max-h-[800px]">
                <div className="flex items-start justify-between p-6 rounded-t">
                <h3 className="text-3xl text-white font-mono font-semibold">Export Wizard</h3>
                <button onClick={props.onClose} className="p-1 ml-auto bg-transparent border-0 text-green-500 opacity-5 float-right text-3xl leading-none font-semibold outline-none focus:outline-none">
                    <span className="bg-transparent text-green-500 opacity-5 h-6 w-6 text-2xl block outline-none focus:outline-none">×</span>
                </button>
                </div>
                <div className="flex-auto overflow-auto my-2 mx-6 rounded">
                {renderContent()}
                </div>
                <div className="flex items-center justify-end p-6 rounded-b">
                <button onClick={props.onClose} className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150">Close</button>
                </div>
            </div>
            </div>
        </div>
        <div className="backdrop-blur-sm fixed inset-0 z-40"></div>
        </>

      );
    };
    