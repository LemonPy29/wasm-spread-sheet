import { Filter, Frame, PollSource } from "../../../wasm-lib";

export interface Wasm {
  newFrame: () => Frame;
  processStreamChunk: (
    frame: Frame,
    chunk: Uint8Array,
    header: boolean,
  ) => void,
  processStreamTail: (frame: Frame) => void;
  addEqualtoFilter: (filter: Filter, frame: Frame, bytes: Uint8Array, column: string) => void;
  newFilter: () => Filter;
  processCommand: (command: string, frame: Frame) => PollSource;
}

export default class FrameJS {
  _tag: "frame" = "frame";
  private wasm?: Wasm;
  private columnOrder: Map<string, number> = new Map();
  private _frame?: Frame;
  readonly id: number;
  readonly name: string;

  constructor(id: number, name: string, wasm: Wasm) {
    this.wasm = wasm;
    this._frame = this.wasm!.newFrame();
    this.id = id;
    this.name = name;
  }

  processStreamChunk(chunk: Uint8Array, header: boolean) {
    return this.wasm!.processStreamChunk(this._frame!, chunk, header);
  }

  initColumnOrder() {
    const columnNamesIter = this.header.entries();
    let result = columnNamesIter.next();
    while (!result.done) {
      const [index, name] = result.value;
      this.columnOrder.set(name, index);
      result = columnNamesIter.next();
    }
  }

  processStreamTail() {
    return this.wasm!.processStreamTail(this._frame!);
  }

  slice(offset: number, len: number) {
    return this._frame!.slice(offset, len);
  }

  distinct(column: string): string {
    return this._frame!.distinct(column);
  }

  get numberOfChunks() {
    return this._frame!.numberOfChunks;
  }

  get header() {
    return this._frame!.header;
  }

  get dtypes() {
    return this._frame!.dtypes;
  }

  get wasmPtr() {
    return this._frame!;
  }
}
