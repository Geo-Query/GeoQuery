import React from "react";
import {QueryResult, QueryState} from "../lib/query";

export interface ModalProps {
    queryState: QueryState,
    results: Array<QueryResult>,
}
export default function Modal(props: ModalProps) {
    return (
        <div className="flex justify-center items-center bg-thales-dark bg-opacity-20 overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none">
            <div className="relative mx-auto">
                {/*content*/}
                <div className="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-thales-dark outline-none focus:outline-none min-w-[1000px] min-h-[800px]">
                    {/*header*/}
                    <div className="flex items-start justify-between p-6  rounded-t">
                        <h3 className="text-3xl font-semibold">
                            Export Wizard
                        </h3>
                        <button
                            className="p-1 ml-auto bg-transparent border-0 text-green-500 opacity-5 float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
                        >
                        <span className="bg-transparent text-green-500 opacity-5 h-6 w-6 text-2xl block outline-none focus:outline-none">
                            ×
                        </span>
                        </button>
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