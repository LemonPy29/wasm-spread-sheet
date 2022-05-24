import { Action } from "./reader";

// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

type OnMessage = { data: Action };

worker.onmessage = async ({ data }: OnMessage) => {
  const { appendChunkToFrame, appendRemainderToFrame, getChunk, numberOfChunks } = await import(
    "wasm"
  );
  if (data.type === "parse") {
    const { chunk, header, remainder } = data.payload;
    const new_remainder = appendChunkToFrame(chunk, header, remainder);
    const progress = numberOfChunks();
    worker.postMessage({ progress, new_remainder });
  } else if (data.type === "getChunk") {
    const { offset, len } = data.payload;
    const chunk = getChunk(offset, len);
    worker.postMessage(chunk);
  } else if (data.type === "remainder") {
    appendRemainderToFrame(data.payload);
  }

  // match(data)
  //   .with({ type: "parse", payload: P.select() }, ({ chunk, header }) => {
  //     numberOfChunks++;
  //     remainder = appendChunkToFrame(chunk, header, remainder);
  //     worker.postMessage({ progress: numberOfChunks });
  //   })
  //   .with({ type: "getChunk", payload: P.select() }, ({ offset, len }) => {
  //     const chunk = getChunk(offset, len);
  //     worker.postMessage(chunk);
  //   })
  //   .with({ type: "getHeader" }, () => {
  //     const header = getHeader();
  //     worker.postMessage(header);
  //   })
  //   .with({ type: "remainder" }, () => {
  //     appendRemainderToFrame(remainder);
  //   })
  //   .run();
};

export default {} as typeof Worker & (new () => Worker);
