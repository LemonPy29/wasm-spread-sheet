import { Action } from "./reader";

// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

type OnMessage = { data: Action };

worker.onmessage = async ({ data }: OnMessage) => {
  const { parse_lines,  progress, get_chunk, process_remainder } = await import("wasm");

  if (data.type === "parse") {
    const { payload } = data;
    const { chunk, header } = payload;
    parse_lines(chunk, header);
    const prog = progress();
    worker.postMessage({ progress: prog });
  } else if (data.type === "getChunk") {
    const { payload } = data;
    const { offset, len } = payload;
    const chunk = get_chunk(offset, len);
    worker.postMessage(chunk);
  } else if (data.type === "remainder") {
    process_remainder();
  }
};

export default {} as typeof Worker & (new () => Worker);
