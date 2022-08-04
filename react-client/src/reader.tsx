import React from "react";
import "./reader.css";
import { dataStatusContext, metadataContext, workerDataContext } from "./App";
import ReaderWorker from "./read.worker.ts";
import { ParsingSendMessage, ProcessRemainderSendMessage } from "./worker.interface";

export const dataWorker = new ReaderWorker();

const Reader = () => {
  const { workerDataState } = React.useContext(workerDataContext);
  const { dataStatus, setDataStatus } = React.useContext(dataStatusContext);
  const { metadata, setMetadata } = React.useContext(metadataContext);

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setDataStatus("Waiting");
    setMetadata({ ...metadata, headerCheckBoxDisabled: true });
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        const action: ParsingSendMessage = {
          type: "parsing",
          payload: {
            id: metadata.selectedId,
            name: file.name,
            chunk: value,
            header: metadata.headerChecked && workerDataState.progress === 0,
          },
        };
        dataWorker.postMessage(action);
      }
      if (done) {
        const action: ProcessRemainderSendMessage = {
          type: "processRemainder",
          payload: {
            id: metadata.selectedId,
          }
        };
        dataWorker.postMessage(action);
        break;
      }
    }
  };

  React.useEffect(() => {
    if (workerDataState.progress === 1 && dataStatus === "Waiting") {
      setTimeout(() => {
        setDataStatus("headerPhase");
      }, 1000);
    }
  }, [workerDataState.progress, dataStatus, setDataStatus]);

  return (
    <>
      <label htmlFor={"upload-button"}>
        <div className="file-input">Choose your file</div>
      </label>
      <input
        type="file"
        id="upload-button"
        style={{ display: "none" }}
        onChange={fileHandler}
      ></input>
    </>
  );
};

export default Reader;
