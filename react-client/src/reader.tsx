import React from "react";
import "./reader.css";
import { chunkStatusContext, dataStatusContext } from "./App";
import ReaderWorker from "./read.worker.ts";

type Message = {
  data: {
    progress: number;
  };
};
export type Parse = { type: "parse"; payload: { chunk: Uint8Array; header: boolean } };
export type getChunk = { type: "getChunk"; payload: { offset: number; len: number } };
export type Remainder = { type: "remainder" };
export type Action = Parse | getChunk | Remainder;

export const dataWorker = new ReaderWorker();

const Reader = () => {
  const [progress, setProgress] = React.useState<number>(0);
  const { dataStatus, setDataStatus } = React.useContext(dataStatusContext);
  const { chunkStatus, setChunkStatus } = React.useContext(chunkStatusContext);

  const process = async (action: Action) => {
    dataWorker.postMessage(action);
    dataWorker.onmessage = async ({ data }: Message) => {
      console.log(data.progress);
      setProgress(data.progress);
    };
  };

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setDataStatus("Waiting");
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        const action: Parse = { type: "parse", payload: { chunk: value, header: true } };
        process(action);
        if (chunkStatus === "Pending") {
          setChunkStatus("Available");
        }
      }
      if (done) {
        const action: Remainder = { type: "remainder" };
        dataWorker.postMessage(action);
        break;
      }
    }
  };

  React.useEffect(() => {
    if (progress === 1 && dataStatus === "Waiting") {
      setTimeout(() => setDataStatus("Usable"), 1000);
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
