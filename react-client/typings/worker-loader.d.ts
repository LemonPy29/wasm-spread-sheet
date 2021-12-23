declare module "*.worker.ts" {
  class ReaderWorker extends Worker {
    constructor();
  }

  export default ReaderWorker;
}
