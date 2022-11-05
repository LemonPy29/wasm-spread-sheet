import "../styles/table-ui.css";
import { useState } from "react";
import { match, P } from "ts-pattern";
import { useUIWorkerHandler } from "../hooks/worker";
import { ColumnProps, FrameProps, HeaderProps } from "./components.interface";
import QueryInput from "./query-input";

const DEFAULT_N_COLS = 10;
const DEFAULT_N_ROWS = 20;

const TopBar = ({
  names,
  selectedId,
  onClick,
}: {
  names: string[];
  selectedId: number;
  onClick: (index: number) => void;
}) => {
  const color = (id: number) => (id === selectedId ? "#fc81a5" : "#04a7a7");
  return (
    <div className="top-bar">
      {names.map((name, index) => (
        <div
          className="top-bar__item"
          key={index}
          style={{
            backgroundColor: color(index),
            borderColor: color(index),
            zIndex: names.length - index,
          }}
          onClick={() => onClick(index)}
        >
          <span>{name}</span>
        </div>
      ))}
    </div>
  );
};

export const Header = (props: HeaderProps) => {
  return match(props)
    .with({ type: "left", name: P.select() }, (name) => (
      <div className="frame__header header-left">{name}</div>
    ))
    .with({ type: "right", name: P.select() }, (name) => (
      <div className="frame__header header-right">{name}</div>
    ))
    .with({ type: "center", name: P.select() }, (name) => (
      <div className="frame__header">{name}</div>
    ))
    .run();
};

export const Column = ({ header, data, onContextMenu }: ColumnProps) => {
  const column = Array.from({ length: DEFAULT_N_ROWS }, (_, i) => (
    <div className="frame__cell" key={i}>
      <div className="cell__text">{data?.[i] || ""}</div>
    </div>
  ));
  return (
    <>
      <div className="frame__column" onContextMenu={onContextMenu}>
        <Header {...header} />
        {column}
      </div>
    </>
  );
};

export const FrameTable = ({ header, data }: FrameProps) => {
  const len = data?.length || DEFAULT_N_COLS;
  const columns = Array.from({ length: len }, (_, i) => {
    const columnHeader = header?.[i] || String.fromCharCode(65 + i);
    return (
      <Column
        key={i}
        header={{
          type: i === 0 ? "left" : i === len - 1 ? "right" : "center",
          name: columnHeader,
        }}
        data={data?.[i]}
      />
    );
  });

  return (
    <>
      <div className="frame__table">{columns}</div>
    </>
  );
};

export const DataHandler = () => {
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
          <TopBar names={names} selectedId={selectedId} onClick={setSelectedId} />
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
