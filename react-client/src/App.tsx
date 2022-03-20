import React from "react";
import "./App.css";
import { DataHandler } from "./frame/table-ui";
import SideBar from "./layout/side-bar";

export type DataStatus = "Empty" | "Waiting" | "ReadyToUse" | "Used";

interface IreadyToPull {
  status: DataStatus;
  setSatus: (input: DataStatus) => void;
}
export const readyToPullContext = React.createContext({} as IreadyToPull);

interface IsDone {
  done: boolean;
  setDone: (input: boolean) => void;
}
export const isDoneContext = React.createContext({} as IsDone);

function App() {
  const [_ready, _setReady] = React.useState<DataStatus>("Empty");
  const readyToPullContextValue: IreadyToPull = {
    status: _ready,
    setSatus: (_input: DataStatus) => _setReady(_input),
  };

  const [_done, _setIsDone] = React.useState<boolean>(false);
  const isDoneValue: IsDone = {
    done: _done,
    setDone: (_input: boolean) => _setIsDone(_input),
  };

  return (
    <div className="App">
      <readyToPullContext.Provider value={readyToPullContextValue}>
      <isDoneContext.Provider value={isDoneValue}>
        <SideBar />
        <header className="App-header">
          <DataHandler />
        </header>
      </isDoneContext.Provider>
      </readyToPullContext.Provider>
    </div>
  );
}

export default App;
