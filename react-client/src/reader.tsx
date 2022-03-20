import React from "react";
import "./reader.css";
import { readyToPullContext, isDoneContext } from "./App";
import ReaderWorker from "./read.worker.ts";

type messageData = {
  data: {
    parsedData: string[];
  };
};

const readerWorker = new ReaderWorker();

const Reader = () => {
  const [chunksDone, setChunksDone] = React.useState<number>(0);
  const { status, setSatus } = React.useContext(readyToPullContext);
  const { setDone } = React.useContext(isDoneContext);

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setSatus("Waiting");
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();
    const {
      add_chunk,
      process_remainder,
      set_header,
      chunks_done,
    } = await import("wasm");
    let skipHeader = true;

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        if (skipHeader) {
          set_header(value);
          skipHeader = false;
        }

        readerWorker.postMessage(value);
        readerWorker.onmessage = async ({
          data: { parsedData },
        }: messageData) => {
          add_chunk(parsedData);
          const chunksDone = chunks_done();
          setChunksDone(chunksDone);
        };
      }
      if (done) {
        process_remainder();
        setDone(true);
        break;
      }
    }
  };

  React.useEffect(() => {
    if (chunksDone === 1 && status === "Waiting") {
      setTimeout(() => setSatus("ReadyToUse"), 1000);
    }
  }, [chunksDone, status, setSatus]);

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
