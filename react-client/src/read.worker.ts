// eslint-disable-next-line no-restricted-globals
const worker: Worker = self as any;

worker.onmessage = async ({ data: chunk }) => {
  const { parse_and_join } = await import("wasm");
  if (chunk) {
    const parsedData = parse_and_join(chunk, true);
    worker.postMessage({ parsedData });
  } else {
    console.log("All done");
  }
};

export default {} as typeof Worker & (new () => Worker);
