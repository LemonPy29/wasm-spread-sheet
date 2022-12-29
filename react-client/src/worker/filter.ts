import { Frame, PollSource } from "../../../wasm-lib";

export default class Source {
  _tag: "source" = "source";
  private source: PollSource;
  readonly id: number;
  readonly name: string;
  readonly frameRef: number;

  constructor(source: PollSource, frameRef: number, id: number, name: string) {
    this.source = source;
    this.frameRef = frameRef;
    this.id = id;
    this.name = name;
  }

  slice(frame: Frame, offset: number, len: number) {
    return this.source.slice(frame, offset, len);
  }
}
