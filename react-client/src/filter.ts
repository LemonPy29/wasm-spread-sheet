import FrameJS from "./frame";

export default class LazyFilter {
  frameRef: WeakRef<FrameJS>;
  lazyFilter: unknown;

  constructor(frame: FrameJS, lazyFilter: unknown) {
    this.frameRef = new WeakRef(frame);
    this.lazyFilter = lazyFilter;
  }
}
