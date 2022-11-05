import React from "react";
import "./App.css";
import FrameUI from "./frame/table-ui";
import Reader from "./reader";

export type readingState = "Empty" | "ReadyToUse" | "Used";

interface IreadyToPull {
  ready: readingState;
  setReady: (input: readingState) => void;
}
export const readyToPullContext = React.createContext({} as IreadyToPull);

function App() {
  const [_ready, _setReady] = React.useState<readingState>("Empty");
  const readyToPullContextValue: IreadyToPull = {
    ready: _ready,
    setReady: (_input: readingState) => _setReady(_input),
  };

  return (
    <div className="App">
      <header className="App-header">
        <readyToPullContext.Provider value={readyToPullContextValue}>
          <Reader />
          <FrameUI />
        </readyToPullContext.Provider>
      </header>
    </div>
  );
}

export default App;
