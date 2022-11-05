import React from "react";
import { readyToPullContext } from "./App";
import ReaderWorker from "./read.worker.ts";

type messageData = {
  data: {
    chunksDone: number;
    parsedData: string[];
  };
};

const readerWorker = new ReaderWorker();

const Reader = () => {
  const { ready, setReady } = React.useContext(readyToPullContext);
  const [chunksDone, setChunksDone] = React.useState(0);

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();
    const { process_remainder, set_header, add_chunk } = await import("wasm");
    let skipHeader = true;

    while (true) {
      const { done, value } = await reader.read();
      if (value !== undefined) {
        if (skipHeader) {
          set_header(value);
          skipHeader = false;
        }

        readerWorker.postMessage(value);
        readerWorker.onmessage = ({ data: { chunksDone, parsedData } }: messageData) => {
          add_chunk(parsedData);
          setChunksDone(chunksDone);
        };
      }
      if (done) {
        process_remainder();
        break;
      }
    }
  };

  React.useEffect(() => {
    if (chunksDone === 1 && ready === "Empty") {
      setReady("ReadyToUse");
    }
  }, [chunksDone, ready, setReady]);

  return (
    <>
      <input type="file" className="fileInput" onChange={fileHandler}></input>
    </>
  );
};

export default Reader;
