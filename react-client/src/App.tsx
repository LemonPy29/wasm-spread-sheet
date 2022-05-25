import { none } from "fp-ts/lib/Option";
import React from "react";
import "./App.css";
import { DataStatus, DataStatusManager, Metadata, MetadataManager } from "./components.interface";
import { DataHandler } from "./frame/table-ui";
import SideBar from "./layout/side-bar";

export const dataStatusContext = React.createContext({} as DataStatusManager);
export const metadataContext = React.createContext({} as MetadataManager);

function App() {
  const [dataStatus, setDataStatus] = React.useState<DataStatus>("Empty");
  const [metadata, setMetadata] = React.useState<Metadata>({
    headerChecked: false,
    headerCheckBoxDisabled: false,
  });

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
          <SideBar schema={none} />
          <header className="App-header">
            <DataHandler />
          </header>
        </dataStatusContext.Provider>
      </metadataContext.Provider>
    </div>
  );
}

export default App;
