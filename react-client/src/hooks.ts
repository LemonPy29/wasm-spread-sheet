import { useContext, useEffect } from "react";
import {
  AddFilterSendMessage,
  ChunkSendMessage,
  HeaderSendMessage,
  QueryableNamesSendMessage,
} from "./worker.interface";
import { dataStatusContext, metadataContext, workerDataContext } from "./App";
import { dataWorker } from "./reader";

const DEFAULT_N_ROWS = 20;

export function useUIWorkerHandler({ offset }: { offset: number }) {
  const { dataStatus, setDataStatus } = useContext(dataStatusContext);
  const { workerDataState } = useContext(workerDataContext);
  const { metadata } = useContext(metadataContext);

  useEffect(() => {
    if (dataStatus === "headerPhase") {
      const action: HeaderSendMessage = {
        type: "getHeader",
        payload: { id: metadata.selectedId },
      };
      dataWorker.postMessage(action);
      setDataStatus("Usable");
    }
  }, [dataStatus, setDataStatus, metadata]);

  useEffect(() => {
    if (dataStatus === "Usable") {
      const action: ChunkSendMessage = {
        type: "getChunk",
        payload: {
          id: metadata.selectedId,
          offset: offset * DEFAULT_N_ROWS,
          len: DEFAULT_N_ROWS,
        },
      };
      dataWorker.postMessage(action);
    }
  }, [dataStatus, offset, metadata]);

  useEffect(() => {
    const action: QueryableNamesSendMessage = { type: "names" };
    dataWorker.postMessage(action);
  }, [workerDataState.header]);

  return {
    header: workerDataState.header,
    slice: workerDataState.chunk,
    loading: dataStatus === "Waiting",
    names: workerDataState.names,
    selectedId: metadata.selectedId,
  };
}

export function useAddFilter() {
  const { metadata } = useContext(metadataContext);
  const encoder = new TextEncoder();
  const action: AddFilterSendMessage = {
    type: "addFilter",
    payload: {
      id: metadata.selectedId,
      type: "equalTo",
      column: "Type 1",
      bytes: encoder.encode("Water"),
    },
  };

  return () => {
    dataWorker.postMessage(action);
  };
}
