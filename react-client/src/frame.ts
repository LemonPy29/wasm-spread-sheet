import { Frame } from "wasm";
import { Queryable } from "./globalDataHandler";

export interface Wasm {
  newFrame: () => Frame;
  width: (frame: Frame) => number;
  height: (frame: Frame) => number;
  numberOfChunks: (frame: Frame) => number;
  readPushStreamChunk: (
    frame: Frame,
    chunk: Uint8Array,
    header: boolean,
    remainder: Uint8Array
  ) => Uint8Array;
  readPushRemainingStream: (frame: Frame, remainder: Uint8Array) => void;
  sliceAsJsStrings: (frame: Frame, offset: number, size: number) => string[];
  getHeader: (frame: Frame) => string[];
  getDtypes: (frame: Frame) => string[];
  sumFrameColumn: (frame: Frame, index: number) => string;
}

export default class FrameJS implements Queryable {
  private _frame?: Frame;
  private wasm?: Wasm;
  private columnOrder: Map<string, number> = new Map();
  readonly id: number;
  readonly name: string;

  constructor(id: number, name: string, wasm: Wasm) {
    this.wasm = wasm;
    this._frame = this.wasm!.newFrame();
    this.id = id;
    this.name = name;
  }

  readPushStreamChunk(chunk: Uint8Array, header: boolean, remainder: Uint8Array) {
    return this.wasm!.readPushStreamChunk(this._frame!, chunk, header, remainder);
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

  readPushRemainingStream(remainder: Uint8Array) {
    return this.wasm!.readPushRemainingStream(this._frame!, remainder);
  }

  sliceAsJsStrings(offset: number, len: number) {
    return this.wasm!.sliceAsJsStrings(this._frame!, offset, len);
  }

  sumFrameColumn(name: string) {
    const index = this.columnOrder.get(name);
    return this.wasm!.sumFrameColumn(this._frame!, index!);
  }

  get numberOfChunks() {
    return this.wasm!.numberOfChunks(this._frame!);
  }

  get header() {
    return this.wasm!.getHeader(this._frame!);
  }

  get dtypes() {
    return this.wasm!.getDtypes(this._frame!);
  }
}
