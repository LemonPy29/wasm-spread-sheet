import { getOrElse, map, none, of, Option, Some } from "fp-ts/lib/Option";
import { match } from "ts-pattern";
import FilterJS from "./filter";
import FrameJS, { Wasm } from "./frame";
import {
  AddFilterSendMessage,
  ChunkSendMessage,
  HeaderSendMessage,
  ParsingSendMessage,
  ProcessRemainderSendMessage,
} from "./worker.interface";

type List<T> = {
  head: Option<Node<T>>;
};

type Node<T> = { inner: T; next: Option<Node<T>> };

export class ListImpl<T extends { id: number }> {
  private data: List<T>;

  constructor() {
    this.data = { head: none };
  }

  isEmpty(): boolean {
    return this.data.head === none;
  }

  push(el: T) {
    const head = {
      inner: el,
      next: this.data.head,
    };

    this.data = { head: of(head) };
  }

  pop(): Option<T> {
    return map((node: Node<T>) => {
      this.data.head = node.next;
      return node.inner;
    })(this.data.head);
  }

  *iter() {
    let ptr = this.data.head;

    while (ptr._tag === "Some") {
      const node = ptr.value;
      ptr = node.next;
      yield node.inner;
    }
  }

  find(id: number): Option<T> {
    for (const el of this.iter()) {
      if (el.id === id) {
        return of(el);
      }
    }
    return none;
  }

  insertAt(id: number, el: T) {
    let ptr = this.data.head;

    while (ptr._tag === "Some") {
      const node = ptr.value;

      if (node.inner.id === id) {
        const next = {
          inner: el,
          next: node.next,
        };

        ptr.value.next = of(next);
        break;
      }

      ptr = node.next;
    }
  }

  replaceAt(id: number, el: T) {
    let ptr = this.data.head;

    while (ptr._tag !== "None") {
      const node = ptr.value;

      if (node.inner.id === id) {
        ptr.value.inner = el;
        break;
      }

      ptr = node.next;
    }
  }
}

type Queryable = FrameJS | FilterJS;

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

  getNames(worker: Worker) {
    worker.postMessage({ type: "names", payload: this.names });
  }

  readPushStreamChunk({ id, name, chunk, header }: ParsingSendMessage["payload"]) {
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
      .with({ _tag: "filter" }, (filter) => {
        const frame = this.unsafeGetFrame(filter.frameRef) as FrameJS;
        return filter.slice(frame.wasmPtr, offset, len);
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

  processRemainder({ id }: ProcessRemainderSendMessage["payload"]) {
    const frame = this.unsafeGetFrame(id) as FrameJS;
    frame.processStreamTail();
  }

  addFilter({ id, column, bytes }: AddFilterSendMessage["payload"]) {
    const queryable = this.unsafeGetFrame(id);
    const index = match(queryable)
      .with({ _tag: "filter" }, (filter) => {
        const frame = this.unsafeGetFrame(filter.frameRef) as FrameJS;
        return filter.pushEqualToFilter(bytes, frame.wasmPtr, column);
      })
      .with({ _tag: "frame" }, (frame) => {
        const filter = new FilterJS(frame.id, this.wasm!, id + 1, `${frame.name}_${column}`);
        this.push(filter);
        return filter.pushEqualToFilter(bytes, frame.wasmPtr, column);
      })
      .run();

    this.worker!.postMessage({
      type: "addFilter",
      payload: {
        index,
        names: this.names,
      },
    });
  }
}
