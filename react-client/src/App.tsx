import { none, Option } from "fp-ts/lib/Option";
import React from "react";
import "./App.css";
import { DataHandler } from "./frame/table-ui";
import SideBar from "./layout/side-bar";

export type DataStatus = "Empty" | "Waiting" | "Usable";

interface DataStatusInterface {
  dataStatus: DataStatus;
  setDataStatus: (input: DataStatus) => void;
}
export const dataStatusContext = React.createContext({} as DataStatusInterface);

export type ChunkStatus = "Available" | "Pending";
type ChunkStatusInterface = {
  chunkStatus: ChunkStatus;
  setChunkStatus: (input: ChunkStatus) => void;
};
export const chunkStatusContext = React.createContext({} as ChunkStatusInterface);

function App() {
  const [dataStatus, setDataStatus] = React.useState<DataStatus>("Empty");
  const [chunkStatus, setChunkStatus] = React.useState<ChunkStatus>("Available");

  const dataStatusValue: DataStatusInterface = {
    dataStatus: dataStatus,
    setDataStatus: (_input: DataStatus) => setDataStatus(_input),
  };

  const chunkStatusValue: ChunkStatusInterface = {
    chunkStatus: chunkStatus,
    setChunkStatus: (_input: ChunkStatus) => setChunkStatus(_input),
  };

  const schema: Option<Record<string, string>> = none;

  return (
    <div className="App">
      <dataStatusContext.Provider value={dataStatusValue}>
        <chunkStatusContext.Provider value={chunkStatusValue}>
          <SideBar schema={schema} />
          <header className="App-header">
            <DataHandler />
          </header>
        </chunkStatusContext.Provider>
      </dataStatusContext.Provider>
    </div>
  );
}

export default App;
