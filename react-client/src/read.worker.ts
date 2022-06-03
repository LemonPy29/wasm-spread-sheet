import { match, P } from "ts-pattern";
import FrameJS from "./frame";
import { WorkerSendMessage } from "./worker.interface";

// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

const frame = new FrameJS();

worker.onmessage = ({ data }: { data: WorkerSendMessage }) => {
  match(data)
    .with({ type: "parsing", payload: P.select() }, ({ chunk, header, remainder }) => {
      const newRemainder = frame.readPushStreamChunk(chunk, header, remainder);
      const progress = frame.numberOfChunks;
      worker.postMessage({ type: "parsing", payload: { progress, newRemainder } });
    })
    .with({ type: "getChunk", payload: P.select() }, async ({ offset, len }) => {
      const chunk = frame.sliceAsJsStrings(offset, len);
      worker.postMessage({ type: "chunk", payload: chunk });
    })
    .with({ type: "sumCol", payload: P.select() }, (index) => {
      const result = frame.sumFrameColumn(index);
      worker.postMessage({ type: "sumCol", payload: result });
    })
    .with({ type: "getHeader" }, () => {
      const header = frame.header;
      worker.postMessage({ type: "header", payload: header });
    })
    .with({ type: "processRemainder", payload: P.select() }, (remainder) => {
      frame.readPushRemainingStream(remainder);
    })
    .run();
};

export default {} as typeof Worker & (new () => Worker);
