import React from "react";
import {QueryResult, QueryState} from "../lib/query";
import Query_progress from "./query_progress";
import QueryProgress from "./query_progress";

export interface ModalProps {
    queryState: QueryState,
    results: Array<QueryResult>,
}
export default function Modal(props: ModalProps) {
    let progressBar;

    if (props.queryState !== QueryState.COMPLETE) {
        progressBar = <QueryProgress queryState={props.queryState}/>;
    } else {
        progressBar = <p className="text-red-300">THIS SHOULD NOT BE REACHABLE!</p>;
    }



    return (
        <div className="flex justify-center items-center bg-thales-dark bg-opacity-20 overflow-x-hidden overflow-y-hidden fixed inset-0 z-50 outline-none focus:outline-none">
            <div className="relative mx-auto">
                {/*content*/}
                <div className="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-thales-dark outline-none focus:outline-none min-w-[1000px] min-h-[800px]">
                    {/*header*/}
                    <div className="flex justify-between w-full items-center p-6 rounded-t">
                        <div className="flex items-center">
                            <h1 className="text-3xl font-semibold text-white">Export Wizard</h1>
                        </div>
                        {}
                        <div className="flex justify-end items-center flex-grow p-6 rounded-t">
                            {progressBar}
                        </div>
                    </div>
                    {/*body*/}
                    <div className="relative flex-auto my-2 mx-6 rounded overflow-y-scroll">
                            {props.results.map((r) => r.file.path + "\n")}
                    </div>
                    {/*footer*/}
                    <div className="flex items-center justify-end p-6 rounded-b">

                        <button
                            className="bg-green-500 text-white active:bg-green-600 font-bold uppercase text-sm px-6 py-3 m-1 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                            type="button"
                        >
                            Save Changes
                        </button>

                        <button
                            className="text-red-500 background-transparent hover:bg-red-500 hover:text-white font-bold uppercase px-6 py-3 m-1 rounded text-sm outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                            type="button"
                        >
                            Close
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}