import { match, P } from "ts-pattern";
import { WorkerSendMessage } from "./worker.interface";

// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

worker.onmessage = async ({ data }: { data: WorkerSendMessage }) => {
  const {
    appendChunkToFrame,
    appendRemainderToFrame,
    getChunk,
    numberOfChunks,
    getHeader,
    sumFrameColumn,
  } = await import("wasm");

  match(data)
    .with({ type: "parsing", payload: P.select() }, ({ chunk, header, remainder }) => {
      const newRemainder = appendChunkToFrame(chunk, header, remainder);
      const progress = numberOfChunks();
      worker.postMessage({ type: "parsing", payload: { progress, newRemainder } });
    })
    .with({ type: "getChunk", payload: P.select() }, ({ offset, len }) => {
      const chunk = getChunk(offset, len);
      worker.postMessage({ type: "chunk", payload: chunk });
    })
    .with({ type: "sumCol", payload: P.select() }, (index) => {
      const result = sumFrameColumn(index);
      worker.postMessage({ type: "sumCol", payload: result });
    })
    .with({ type: "getHeader" }, () => {
      const header = getHeader();
      worker.postMessage({ type: "header", payload: header });
    })
    .with({ type: "processRemainder", payload: P.select() }, (remainder) => {
      appendRemainderToFrame(remainder);
    })
    .run();
};

export default {} as typeof Worker & (new () => Worker);
