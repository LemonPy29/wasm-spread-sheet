import { WorkerRecMessage } from "../worker/worker.interface";

export interface Store {
  csvReaderStatus: CsvReaderStatus;
  headerBox: boolean;
  disableHeaderBox: boolean;
  workerApi: WorkerApi;
  setCsvReaderDataStatus: (to: CsvReaderStatus) => void;
  toggleHeaderBox: () => void;
  toggleDisableHeaderBox: () => void;
  setSelectedId: (id: number) => void;
  dispatchWorkerAction: (args: WorkerRecMessage) => void;
}

export type CsvReaderStatus = "Empty" | "Waiting" | "HeaderPhase" | "Usable";

export interface WorkerApi {
  progress: number;
  selectedId: number;
  slice: string[][];
  header: string[];
  names: string[];
  equalToOptions: string[];
}
