import React from "react";
import "./App.css";
import { DataStatus, DataStatusManager, Metadata, MetadataManager } from "./components.interface";
import { DataHandler } from "./frame/table-ui";
import { useWorkerStore } from "./hooks";
import SideBar from "./layout/side-bar";
import { dataWorker } from "./reader";

export const dataStatusContext = React.createContext({} as DataStatusManager);
export const metadataContext = React.createContext({} as MetadataManager);
export const workerDataContext = React.createContext({} as ReturnType<typeof useWorkerStore>);

function App() {
  const [dataStatus, setDataStatus] = React.useState<DataStatus>("Empty");
  const [metadata, setMetadata] = React.useState<Metadata>({
    headerChecked: true,
    headerCheckBoxDisabled: false,
  });

  const workerStore = useWorkerStore(dataWorker);

  const dataStatusManager: DataStatusManager = {
    dataStatus: dataStatus,
    setDataStatus: (input: DataStatus) => setDataStatus(input),
  };

  const metadataManager: MetadataManager = {
    metadata: metadata,
    setMetadata: (input: Metadata) => setMetadata(input),
  };

  return (
    <div className="App">
      <metadataContext.Provider value={metadataManager}>
        <dataStatusContext.Provider value={dataStatusManager}>
          <workerDataContext.Provider value={workerStore}>
            <SideBar />
            <header className="App-header">
              <DataHandler />
            </header>
          </workerDataContext.Provider>
        </dataStatusContext.Provider>
      </metadataContext.Provider>
    </div>
  );
}

export default App;
