import "./table-ui.css";
import { dataStatusContext } from "../App";
import React from "react";
import { ColumnProps, FrameProps } from "../components.interface";
import { dataWorker } from "../reader";

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

export const FrameUI = ({ header, data }: FrameProps) => {
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

export const DataHandler = () => {
  const { dataStatus } = React.useContext(dataStatusContext);
  const [localChunk, setLocalChunk] = React.useState<string[][]>([]);
  const [offset, setOffset] = React.useState<number>(0);

  React.useEffect(() => {
    if (dataStatus === "Usable") {
      const action = {
        type: "getChunk",
        payload: {
          offset: offset * DEFAULT_N_ROWS,
          len: DEFAULT_N_ROWS,
        },
      };
      dataWorker.postMessage(action);
      dataWorker.onmessage = ({ data }: { data: string[] }) => {
        const chunk = data.map((column) => column.split("DELIMITER_TOKEN"));
        setLocalChunk(chunk);
      };
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
        <div className="frame">
          <FrameUI data={localChunk} />
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
};
