import { match, P } from "ts-pattern";
import { GlobalDataHandler } from "./globalDataHandler";
import { WorkerSendMessage } from "./worker.interface";

// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

const gdh = new GlobalDataHandler();

worker.onmessage = ({ data }: { data: WorkerSendMessage }) => {
  match(data)
    .with({ type: "parsing", payload: P.select() }, (payload) =>
      gdh.readPushStreamChunk(worker, payload)
    )
    .with({ type: "getChunk", payload: P.select() }, (payload) => gdh.getChunk(worker, payload))
    .with({ type: "sumCol", payload: P.select() }, (payload) => gdh.sum(worker, payload))
    .with({ type: "getHeader", payload: P.select() }, (payload) => gdh.header(worker, payload))
    .with({ type: "processRemainder", payload: P.select() }, (payload) =>
      gdh.processRemainder(payload)
    )
    .with({ type: "names" }, () => gdh.getNames(worker))
    .run();
};

export default {} as typeof Worker & (new () => Worker);
