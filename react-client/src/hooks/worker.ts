import { useCallback, useEffect } from "react";
import { useGlobalStore } from "./store";
import {
  ChunkSendMessage,
  CommandSendMessage,
  DistinctSendMessage,
  HeaderSendMessage,
  ParsingSendMessage,
  ProcessRemainderSendMessage,
  QueryableNamesSendMessage,
  WorkerRecMessage,
} from "../worker/worker.interface";
import WasmWorker from "../worker/read.worker.ts";
import { DEFAULT_N_ROWS } from "../constants";

export const wasmWorker = new WasmWorker();

export function useWorkerStore() {
  const [state, dispatch] = useGlobalStore((state) => [
    state.workerApi,
    state.dispatchWorkerAction,
  ]);

  wasmWorker.onmessage = ({ data }: { data: WorkerRecMessage }) => {
    return dispatch(data);
  };

  return state;
}

export function useUIWorkerHandler({ offset }: { offset: number }) {
  const [csvReaderStatus, setCsvReaderStatus, setSelecteId, workerApi] = useGlobalStore((state) => [
    state.csvReaderStatus,
    state.setCsvReaderDataStatus,
    state.setSelectedId,
    state.workerApi,
  ]);

  useEffect(() => {
    if (csvReaderStatus === "HeaderPhase") {
      const action: HeaderSendMessage = {
        type: "getHeader",
        payload: { id: workerApi.selectedId },
      };
      wasmWorker.postMessage(action);
      setCsvReaderStatus("Usable");
    }
  }, [csvReaderStatus, setCsvReaderStatus, workerApi.selectedId]);

  useEffect(() => {
    if (csvReaderStatus === "Usable") {
      const action: ChunkSendMessage = {
        type: "getChunk",
        payload: {
          id: workerApi.selectedId,
          offset: offset * DEFAULT_N_ROWS,
          len: DEFAULT_N_ROWS,
        },
      };
      wasmWorker.postMessage(action);
    }
  }, [csvReaderStatus, offset, workerApi.selectedId]);

  useEffect(() => {
    const action: QueryableNamesSendMessage = { type: "names", payload: null };
    wasmWorker.postMessage(action);
  }, [workerApi.header]);

  return {
    loading: csvReaderStatus === "Waiting",
    header: workerApi.header,
    slice: workerApi.slice,
    names: workerApi.names,
    selectedId: workerApi.selectedId,
    setSelectedId: setSelecteId,
  };
}

export function useCommand() {
  const { selectedId } = useWorkerStore();

  return (command: string) => {
    const action: CommandSendMessage = {
      type: "command",
      payload: {
        id: selectedId,
        command,
      },
    };
    wasmWorker.postMessage(action);
  };
}

export function useParseFileChunk() {
  const headerBox = useGlobalStore((state) => state.headerBox);
  const { selectedId, progress } = useWorkerStore();

  return useCallback(
    (name: string, value: Uint8Array) => {
      const action: ParsingSendMessage = {
        type: "parsing",
        payload: {
          id: selectedId,
          name: name,
          chunk: value,
          header: headerBox && progress === 0,
        },
      };
      wasmWorker.postMessage(action);
    },
    [selectedId, headerBox, progress]
  );
}

export function useParseFileTail() {
  const { selectedId } = useWorkerStore();

  return useCallback(() => {
    const action: ProcessRemainderSendMessage = {
      type: "processRemainder",
      payload: {
        id: selectedId,
      },
    };
    wasmWorker.postMessage(action);
  }, [selectedId]);
}

export function useDistinctColumnValues() {
  const { selectedId } = useWorkerStore();

  return useCallback(
    (column: string) => {
      const action: DistinctSendMessage = {
        type: "distinct",
        payload: {
          id: selectedId,
          column,
        },
      };
      wasmWorker.postMessage(action);
    },
    [selectedId]
  );
}
