import { useContext, useEffect, useReducer } from "react";
import {
  AddFilterSendMessage,
  ChunkSendMessage,
  HeaderSendMessage,
  QueryableNamesSendMessage,
  WorkerRecMessage,
} from "./worker.interface";
import { dataStatusContext, workerDataContext } from "./App";
import { dataWorker } from "./reader";
import { match, P } from "ts-pattern";

const DEFAULT_N_ROWS = 20;

interface WorkerDataState {
  progress: number;
  selectedId: number;
  slice: string[][];
  header: string[];
  names: string[];
}

const reducer = (state: WorkerDataState, action: WorkerRecMessage): WorkerDataState => {
  return match(action)
    .with({ type: "parsing", payload: P.select() }, ({ progress }) => {
      return { ...state, progress };
    })
    .with({ type: "chunk", payload: P.select() }, (payload) => {
      console.log(payload);
      const slice = payload.map((column) => column.split("DELIMITER_TOKEN"));
      return { ...state, slice };
    })
    .with({ type: "header", payload: P.select() }, (header) => {
      return { ...state, header };
    })
    .with({ type: "names", payload: P.select() }, (names) => {
      return { ...state, names };
    })
    .with({ type: "addFilter", payload: P.select() }, ({ names }) => {
      return { ...state, names, selectedId: state.selectedId + 1 };
    })
    .run();
};

export function useWorkerStore(worker: Worker) {
  const [state, dispatch] = useReducer(reducer, {
    progress: 0,
    selectedId: 0,
    slice: [],
    header: [],
    names: [],
  });

  worker.onmessage = ({ data }: { data: WorkerRecMessage }) => {
    return dispatch(data);
  };

  return state;
}

export function useUIWorkerHandler({ offset }: { offset: number }) {
  const { dataStatus, setDataStatus } = useContext(dataStatusContext);
  const { selectedId, header, slice, names } = useContext(workerDataContext);

  useEffect(() => {
    if (dataStatus === "headerPhase") {
      const action: HeaderSendMessage = {
        type: "getHeader",
        payload: { id: selectedId },
      };
      dataWorker.postMessage(action);
      setDataStatus("Usable");
    }
  }, [dataStatus, setDataStatus, selectedId]);

  useEffect(() => {
    if (dataStatus === "Usable") {
      const action: ChunkSendMessage = {
        type: "getChunk",
        payload: {
          id: selectedId,
          offset: offset * DEFAULT_N_ROWS,
          len: DEFAULT_N_ROWS,
        },
      };
      dataWorker.postMessage(action);
    }
  }, [dataStatus, offset, selectedId]);

  useEffect(() => {
    const action: QueryableNamesSendMessage = { type: "names" };
    dataWorker.postMessage(action);
  }, [header]);

  return {
    loading: dataStatus === "Waiting",
    header,
    slice,
    names,
    selectedId,
  };
}

export function useAddFilter() {
  const { selectedId } = useContext(workerDataContext);
  const encoder = new TextEncoder();
  const action: AddFilterSendMessage = {
    type: "addFilter",
    payload: {
      id: selectedId,
      type: "equalTo",
      column: "Type 1",
      bytes: encoder.encode("Water"),
    },
  };

  return () => {
    dataWorker.postMessage(action);
  };
}
