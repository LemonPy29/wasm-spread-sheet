import "../styles/table-ui.css";
import { useState } from "react";
import { useUIWorkerHandler } from "../hooks/worker";
import { TableSelector } from "./TableSelector";
import { FrameTable } from "./Frame";
import QueryInput from "./QueryInput";

export const Table = () => {
  const [offset, setOffset] = useState<number>(0);

  const { header, slice, loading, names, selectedId, setSelectedId } = useUIWorkerHandler({
    offset,
  });

  const forwardHandler = () => {
    setOffset(offset + 1);
  };

  const backwardHandler = () => {
    setOffset(offset - 1 >= 0 ? offset - 1 : 0);
  };

  return (
    <>
      {loading ? (
        <div className="frame__spinner"></div>
      ) : (
        <div className="frame">
          <QueryInput />
          <TableSelector names={names} selectedId={selectedId} onClick={setSelectedId} />
          <FrameTable data={slice} header={header} />
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
