// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

worker.onmessage = async ({ data: chunk }) => {
  const { parse_and_join, chunks_done } = await import("wasm");
  if (chunk !== undefined) {
    const parsedData = parse_and_join(chunk, true);
    const chunksDone = chunks_done();
    worker.postMessage({ chunksDone, parsedData });
  } else {
    console.log("All done");
  }
};

export default {} as typeof Worker & (new () => Worker);
