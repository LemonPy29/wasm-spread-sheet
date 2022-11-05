declare module "*.worker.ts" {
  class WasmWorker extends Worker {
    constructor();
  }

  export default WasmWorker;
}
