import { WorkerRecMessage } from "../worker/worker.interface";
import { match, P } from "ts-pattern";
import create from "zustand";
import { CsvReaderStatus, Store, WorkerApi } from "./hooks.interface";

const workerApiDefault = {
  progress: 0,
  selectedId: 0,
  slice: [],
  header: [],
  names: [],
  equalToOptions: [],
};

export const useGlobalStore = create<Store>()((set) => ({
  csvReaderStatus: "Empty",
  headerBox: true,
  disableHeaderBox: false,
  workerApi: workerApiDefault,
  setCsvReaderDataStatus: (to: CsvReaderStatus) => set(() => ({ csvReaderStatus: to })),
  toggleHeaderBox: () => set((state) => ({ headerBox: !state.headerBox })),
  toggleDisableHeaderBox: () => set((state) => ({ disableHeaderBox: !state.disableHeaderBox })),
  setSelectedId: (id) => set((state) => ({ workerApi: { ...state.workerApi, selectedId: id } })),
  dispatchWorkerAction: (args: WorkerRecMessage) =>
    set((state) => ({ workerApi: reducer(state.workerApi, args) })),
}));

const reducer = (state: WorkerApi, action: WorkerRecMessage): WorkerApi => {
  return match(action)
    .with({ type: "parsing", payload: P.select() }, ({ progress }) => {
      return { ...state, progress };
    })
    .with({ type: "chunk", payload: P.select() }, (payload) => {
      const slice = payload.map((column) => column.split("DELIMITER_TOKEN"));
      return { ...state, slice };
    })
    .with({ type: "header", payload: P.select() }, (header) => {
      return { ...state, header };
    })
    .with({ type: "names", payload: P.select() }, (names) => {
      return { ...state, names };
    })
    .with({ type: "addSource", payload: P.select() }, ({ index, names }) => {
      return { ...state, names, selectedId: index };
    })
    .with({ type: "distinct", payload: P.select() }, (payload) => {
      return { ...state, equalToOptions: payload };
    })
    .run();
};
