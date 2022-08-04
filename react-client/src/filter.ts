import { Filter, Frame } from "wasm";
import { Wasm } from "./frame";

export default class FilterJS {
  _tag: "filter" = "filter";
  private wasm: Wasm;
  private filter: Filter;
  readonly id: number;
  readonly name: string;
  readonly frameRef: number;

  constructor(frameRef: number, wasm: Wasm, id: number, name: string) {
    this.filter = wasm.newFilter();
    this.wasm = wasm;
    this.id = id;
    this.name = name;
    this.frameRef = frameRef;
  }

  pushEqualToFilter(bytes: Uint8Array, frame: Frame, column: string) {
    return this.wasm.addEqualtoFilter(this.filter, frame, bytes, column);
  }

  slice(frame: Frame, offset: number, len: number) {
    return this.filter.slice(frame, offset, len);
  }
}
