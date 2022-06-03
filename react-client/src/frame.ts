import { Frame } from "wasm";

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
  sumFrameColumn: (frame: Frame, index: number) => string;
}

export default class FrameJS {
  private _frame?: Frame;
  private wasm?: Wasm;

  constructor() {
    (async () => {
      this.wasm = await import("wasm");
      this._frame = this.wasm!.newFrame();
    })();
  }

  readPushStreamChunk(chunk: Uint8Array, header: boolean, remainder: Uint8Array) {
    return this.wasm!.readPushStreamChunk(this._frame!, chunk, header, remainder);
  }

  readPushRemainingStream(remainder: Uint8Array) {
    return this.wasm!.readPushRemainingStream(this._frame!, remainder);
  }

  sliceAsJsStrings(offset: number, len: number) {
    return this.wasm!.sliceAsJsStrings(this._frame!, offset, len);
  }

  sumFrameColumn(index: number) {
    return this.wasm!.sumFrameColumn(this._frame!, index);
  }

  get numberOfChunks() {
    return this.wasm!.numberOfChunks(this._frame!);
  }

  get header() {
    return this.wasm!.getHeader(this._frame!);
  }
}
