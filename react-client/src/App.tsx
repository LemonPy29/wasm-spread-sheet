import { fold, fromNullable, none } from "fp-ts/lib/Option";
import React, { MouseEvent, useRef } from "react";
import { match, P } from "ts-pattern";
import "./App.css";
import { DataStatus, DataStatusManager, Metadata, MetadataManager } from "./components.interface";
import { DataHandler } from "./frame/table-ui";
import SideBar from "./layout/side-bar";
import { dataWorker } from "./reader";
import { WorkerRecMessage } from "./worker.interface";

export const dataStatusContext = React.createContext({} as DataStatusManager);
export const metadataContext = React.createContext({} as MetadataManager);
export const workerDataContext = React.createContext({} as WorkerOnMessageManager);

interface WorkerDataState {
  progress: number;
  remainder: Uint8Array;
  chunk: string[][];
  header: string[];
}
interface WorkerOnMessageManager {
  workerDataState: WorkerDataState;
  setWorkerDataState: (action: WorkerRecMessage) => void;
}
const reducer = (state: WorkerDataState, action: WorkerRecMessage): WorkerDataState => {
  return match(action)
    .with({ type: "parsing", payload: P.select() }, ({ progress, remainder }) => {
      return { ...state, progress, remainder };
    })
    .with({ type: "chunk", payload: P.select() }, (payload) => {
      const chunk = payload.map((column) => column.split("DELIMITER_TOKEN"));
      return { ...state, chunk };
    })
    .with({ type: "header", payload: P.select() }, (header) => {
      return { ...state, header };
    })
    .run();
};

function App() {
  const [dataStatus, setDataStatus] = React.useState<DataStatus>("Empty");
  const [metadata, setMetadata] = React.useState<Metadata>({
    headerChecked: false,
    headerCheckBoxDisabled: false,
  });
  const [state, dispatch] = React.useReducer(reducer, {
    progress: 0,
    remainder: new Uint8Array(),
    chunk: [],
    header: [],
  });
  const commandRef = useRef<HTMLDivElement>(null);
  const onCommandClick = (ev: MouseEvent) => {
    const optRef = fromNullable(commandRef.current);
    fold(
      () => {},
      (el: HTMLDivElement) => el.onclick!(ev.nativeEvent)
    )(optRef);
  };

  const dataStatusManager: DataStatusManager = {
    dataStatus: dataStatus,
    setDataStatus: (input: DataStatus) => setDataStatus(input),
  };

  const metadataManager: MetadataManager = {
    metadata: metadata,
    setMetadata: (input: Metadata) => setMetadata(input),
  };

  dataWorker.onmessage = ({ data }: { data: WorkerRecMessage }) => {
    match(data)
      .with({ type: "chunk", payload: P.select() }, (payload) => {
        dispatch({ type: "chunk", payload });
      })
      .with({ type: "header", payload: P.select() }, (payload) => {
        dispatch({ type: "header", payload });
      })
      .with({ type: "parsing", payload: P.select() }, (payload) => {
        dispatch({ type: "parsing", payload });
      })
      .otherwise(() => console.log("Unexpected action"));
  };

  const workerDataManager: WorkerOnMessageManager = {
    workerDataState: state,
    setWorkerDataState: dispatch,
  };

  return (
    <div className="App">
      <metadataContext.Provider value={metadataManager}>
        <dataStatusContext.Provider value={dataStatusManager}>
          <workerDataContext.Provider value={workerDataManager}>
            <SideBar schema={none} onClick={onCommandClick} ref={commandRef} />
            <header className="App-header">
              <DataHandler ref={commandRef} />
            </header>
          </workerDataContext.Provider>
        </dataStatusContext.Provider>
      </metadataContext.Provider>
    </div>
  );
}

export default App;
