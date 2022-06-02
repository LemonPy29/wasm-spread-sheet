import "./table-ui.css";
import { dataStatusContext, workerDataContext } from "../App";
import { forwardRef, useContext, useEffect, useImperativeHandle, useRef, useState } from "react";
import { ColumnProps, FrameProps } from "../components.interface";
import { dataWorker } from "../reader";
import { ChunkSendMessage, HeaderSendMessage } from "../worker.interface";

const DEFAULT_N_ROWS = 20;
const DEFAULT_N_COLS = 10;

export const Column = ({ header, data }: ColumnProps) => {
  const column = Array.from({ length: DEFAULT_N_ROWS }, (_, i) => (
    <div className="frame__cell" key={i}>
      <div className="cell__text">{data?.[i] || ""}</div>
    </div>
  ));
  return (
    <>
      <div className="frame__column">
        <div className="frame__header">{header}</div>
        {column}
      </div>
    </>
  );
};

export const FrameTable = ({ header, data }: FrameProps) => {
  const columns = Array.from({ length: data?.length || DEFAULT_N_COLS }, (_, i) => {
    const columnHeader = header?.[i] || String.fromCharCode(65 + i);
    return <Column key={i} header={columnHeader} data={data?.[i]} />;
  });
  return (
    <>
      <div className="frame__table">{columns}</div>
    </>
  );
};

export const DataHandler = forwardRef((_, commandRef) => {
  const { dataStatus, setDataStatus } = useContext(dataStatusContext);
  const { workerDataState } = useContext(workerDataContext);
  const [offset, setOffset] = useState<number>(0);
  const frameRef = useRef<HTMLDivElement>(null);

  useImperativeHandle(
    commandRef,
    () => {
      const currentFrame = frameRef.current!;
      const clickHandler = () => {
        console.log("Here");
        const animation = currentFrame.animate([{ transform: "scaleX(0.3) scaleY(0.4)" }], {
          duration: 2000,
          iterations: 1,
          fill: "both",
        });
        animation.addEventListener("finish", () => {
          animation.commitStyles();
          animation.cancel();
        });
      };
      return { onclick: clickHandler };
    },
    [frameRef]
  );

  useEffect(() => {
    if (dataStatus === "headerPhase") {
      const action: HeaderSendMessage = { type: "getHeader" };
      dataWorker.postMessage(action);
      setDataStatus("Usable");
    }
  }, [dataStatus, setDataStatus]);

  useEffect(() => {
    if (dataStatus === "Usable") {
      const action: ChunkSendMessage = {
        type: "getChunk",
        payload: {
          offset: offset * DEFAULT_N_ROWS,
          len: DEFAULT_N_ROWS,
        },
      };
      dataWorker.postMessage(action);
    }
  }, [dataStatus, offset]);

  const forwardHandler = () => {
    setOffset(offset + 1);
  };

  const backwardHandler = () => {
    setOffset(offset - 1 >= 0 ? offset - 1 : 0);
  };

  return (
    <>
      {dataStatus === "Waiting" ? (
        <div className="frame__spinner"></div>
      ) : (
        <div className="frame" ref={frameRef}>
          <FrameTable data={workerDataState.chunk} header={workerDataState.header} />
          <div className="frame__motions">
            <span className="motion" onClick={backwardHandler}>
              {" "}
              {"<< "}
            </span>
            <span className="motion" onClick={forwardHandler}>
              {" "}
              {">>"}{" "}
            </span>
          </div>
        </div>
      )}
    </>
  );
});
