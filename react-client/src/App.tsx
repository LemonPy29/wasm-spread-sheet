import { map } from "fp-ts/lib/Option";
import React from "react";
import { Frame } from "wasm";
import "./App.css";
import { DataHandler } from "./frame/table-ui";
import { useTaskEither } from "./hooks";
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

  const getFrame = async () => {
    const { Frame } = await import("wasm");
    return new Frame();
  };

  const frame = useTaskEither(getFrame());
  const schema = map((f: Frame) => f.schema as Record<string, string>)(frame);

  return (
    <div className="App">
      <readyToPullContext.Provider value={readyToPullContextValue}>
        <isDoneContext.Provider value={isDoneValue}>
          <SideBar schema={schema} />
          <header className="App-header">
            <DataHandler />
          </header>
        </isDoneContext.Provider>
      </readyToPullContext.Provider>
    </div>
  );
}

export default App;
