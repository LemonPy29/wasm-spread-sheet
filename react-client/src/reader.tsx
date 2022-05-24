import React from "react";
import "./reader.css";
import { dataStatusContext } from "./App";
import ReaderWorker from "./read.worker.ts";

type Message = {
  data: {
    progress: number;
    remainder: Uint8Array;
  };
};
export type Parse = {
  type: "parse";
  payload: { chunk: Uint8Array; header: boolean; remainder: Uint8Array };
};
export type GetChunk = { type: "getChunk"; payload: { offset: number; len: number } };
export type Remainder = { type: "remainder"; payload: Uint8Array };
export type GetHeader = { type: "getHeader" };
export type Action = Parse | GetChunk | Remainder | GetHeader;

export const dataWorker = new ReaderWorker();

const Reader = () => {
  const [progress, setProgress] = React.useState<number>(0);
  const [remainder, setRemainder] = React.useState<Uint8Array>(new Uint8Array());
  const { dataStatus, setDataStatus } = React.useContext(dataStatusContext);

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setDataStatus("Waiting");
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        const action: Parse = {
          type: "parse",
          payload: { chunk: value, header: true && progress === 0, remainder },
        };
        dataWorker.postMessage(action);
        dataWorker.onmessage = ({ data }: Message) => {
          setProgress(data.progress);
          setRemainder(data.remainder);
        };
      }
      if (done) {
        const action: Remainder = { type: "remainder", payload: remainder };
        dataWorker.postMessage(action);
        break;
      }
    }
  };

  React.useEffect(() => {
    if (progress === 1 && dataStatus === "Waiting") {
      setTimeout(() => {
        setDataStatus("Usable");
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
