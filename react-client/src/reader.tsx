import React from "react";
import "./reader.css";
import { dataStatusContext, metadataContext } from "./App";
import ReaderWorker from "./read.worker.ts";
import {
  ParsingSendMessage,
  ProcessRemainderSendMessage,
  WorkerRecMessage,
} from "./worker.interface";
import { match, P } from "ts-pattern";

export const dataWorker = new ReaderWorker();

const Reader = () => {
  const [progress, setProgress] = React.useState<number>(0);
  const [remainder, setRemainder] = React.useState<Uint8Array>(new Uint8Array());
  const { dataStatus, setDataStatus } = React.useContext(dataStatusContext);
  const { metadata, setMetadata } = React.useContext(metadataContext);

  dataWorker.onmessage = ({ data }: { data: WorkerRecMessage }) => {
    match(data)
      .with({ type: "parsing", payload: P.select() }, ({ progress, remainder }) => {
        setProgress(progress);
        setRemainder(remainder);
      })
      .otherwise(() => console.log("Unexpected action"));
  };

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setDataStatus("Waiting");
    setMetadata({ headerChecked: metadata.headerChecked, headerCheckBoxDisabled: true });
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        const action: ParsingSendMessage = {
          type: "parsing",
          payload: { chunk: value, header: metadata.headerChecked && progress === 0, remainder },
        };
        dataWorker.postMessage(action);
      }
      if (done) {
        const action: ProcessRemainderSendMessage = {
          type: "processRemainder",
          payload: remainder,
        };
        dataWorker.postMessage(action);
        break;
      }
    }
  };

  React.useEffect(() => {
    if (progress === 1 && dataStatus === "Waiting") {
      setTimeout(() => {
        setDataStatus("headerPhase");
      }, 1000);
    }
  }, [progress, dataStatus, setDataStatus]);

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
