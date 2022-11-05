import { getOrElse, Option, Some } from "fp-ts/lib/Option";
import { match } from "ts-pattern";
import Source from "./filter";
import FrameJS, { Wasm } from "./frame";
import ListImpl from "./list";
import {
  ChunkSendMessage,
  CommandSendMessage,
  DistinctSendMessage,
  ExtractPayload,
  HeaderSendMessage,
  ParsingSendMessage,
  ProcessRemainderSendMessage,
} from "./worker.interface";

type Queryable = FrameJS | Source;

export class GlobalDataHandler {
  private wasm?: Wasm;
  private data?: ListImpl<Queryable>;
  private names?: string[];
  private worker?: Worker;

  constructor(worker: Worker) {
    (async () => {
      this.data = new ListImpl();
      this.names = [];
      this.wasm = await import("wasm");
      this.worker = worker;
    })();
  }

  private unsafeGetFrame(id: number): Queryable {
    return (this.data!.find(id) as Some<Queryable>).value;
  }

  private push(queryable: Queryable) {
    this.data!.push(queryable);
    this.names!.push(queryable.name);
  }

  getNames() {
    this.worker!.postMessage({ type: "names", payload: this.names });
  }

  readPushStreamChunk({ id, name, chunk, header }: ExtractPayload<ParsingSendMessage>) {
    const frame = getOrElse(() => {
      const frame = new FrameJS(id, name, this.wasm!);
      this.push(frame);
      return frame;
    })(this.data!.find(id) as Option<FrameJS>);

    frame.processStreamChunk(chunk, header);
    const progress = frame.numberOfChunks;

    this.worker!.postMessage({ type: "parsing", payload: { progress } });
  }

  getChunk({ id, offset, len }: ChunkSendMessage["payload"]) {
    const queryable = this.unsafeGetFrame(id);
    const chunk = match(queryable)
      .with({ _tag: "source" }, (source) => {
        const frame = this.unsafeGetFrame(source.frameRef) as FrameJS;
        return source.slice(frame.wasmPtr, offset, len);
      })
      .with({ _tag: "frame" }, (frame) => frame.slice(offset, len))
      .run();
    this.worker!.postMessage({ type: "chunk", payload: chunk });
  }

  header({ id }: HeaderSendMessage["payload"]) {
    const frame = this.unsafeGetFrame(id) as FrameJS;
    frame.initColumnOrder();
    const header = frame.header;
    this.worker!.postMessage({ type: "header", payload: header });
  }

  distinct({ id, column }: DistinctSendMessage["payload"]) {
    const frame = this.unsafeGetFrame(id) as FrameJS;
    const slice = frame.distinct(column).split("DELIMITER_TOKEN");
    this.worker!.postMessage({ type: "distinct", payload: slice });
  }

  processRemainder({ id }: ProcessRemainderSendMessage["payload"]) {
    const frame = this.unsafeGetFrame(id) as FrameJS;
    frame.processStreamTail();
  }

  execCommand({ id, command }: CommandSendMessage["payload"]) {
    const queryable = this.unsafeGetFrame(id);
    const frame = match(queryable)
      .with({ _tag: "frame" }, (frame: FrameJS) => {
        return frame;
      })
      .with({ _tag: "source" }, (source: Source) => {
        const id = source.frameRef;
        return this.unsafeGetFrame(id) as FrameJS;
      })
      .run();

    const pollSource = this.wasm!.processCommand(command, frame.wasmPtr);
    const name = frame.name + "_" + pollSource.source_type();
    const source = new Source(pollSource, id, id + 1, name);
    this.push(source);

    this.worker!.postMessage({
      type: "addSource",
      payload: {
        index: id + 1,
        names: this.names,
      },
    });
  }
}
