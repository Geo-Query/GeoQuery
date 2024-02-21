import QueryHistory from "../utils/queryhistory";
import SelectedRegion from "../utils/region";

export interface QueryProps {
    queryHistory: QueryHistory
    setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>,
    setSelectedRegion: React.Dispatch<React.SetStateAction<SelectedRegion>>,
}


function clearHistory(setQueryHistory: React.Dispatch<React.SetStateAction<QueryHistory>>) {
    setQueryHistory(new QueryHistory([]))
}


export default function History(props: QueryProps) {
    return (
        <div className="flex-grow p-4 rounded mx-6 my-2 bg-[#353744] minwidth xl:flex-grow-0">
            <div
                className="bg-blue-600 text-white font-bold py-2 px-4 rounded min-w-full md:min-w-0 md:min-w-200px flex justify-between items-center">
                <span className="font-bold">Query History</span>
                <button
                    className="text-gray-300 hover:bg-red-500 hover:text-white font-bold uppercase px-6 py-0 m-1 rounded text-sm outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                    type="button"
                    onClick={() => clearHistory(props.setQueryHistory)}
                >
                    Clear
                </button>
            </div>
            <div className="flex flex-col-reverse gap-1 mt-2">
                {props.queryHistory.queries.map((query, index) => (
                    <div
                        className="bg-[#525461] text-white font-bold py-2 px-4 rounded w-full md:w-auto md:min-w-200px hover:bg-[#526071] flex gap-4 text-left justify-between"
                        key={index} onClick={() => props.setSelectedRegion(new SelectedRegion(query))}>
                        <span className="coordinate"><b>North West:</b>   <span
                            className="darker"> {query.northWest.lat.toFixed(8)}, {query.northWest.long.toFixed(8)}</span></span>
                        <span className="coordinate"><b>South East:</b>   <span
                            className="darker"> {query.southEast.lat.toFixed(8)}, {query.southEast.long.toFixed(8)}</span></span>
                    </div>
                ))}
                {/*<button onClick={clearCoordinates}>Clear List</button>*/}
            </div>
        </div>

    )
}