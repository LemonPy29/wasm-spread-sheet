import "./table-ui.css";
import { Dispatch, SetStateAction, useEffect, useRef, useState } from "react";
import { ColumnProps, FrameProps, HeaderProps } from "../components.interface";
import { match, P } from "ts-pattern";
import { useAddFilter, useUIWorkerHandler } from "../hooks";

const DEFAULT_N_COLS = 10;
const DEFAULT_N_ROWS = 20;

type ContextMenuStatus = "Open" | "Closed";
interface ContextMenuProps {
  xPos: number;
  yPos: number;
  columnClickedName: string;
  status: ContextMenuStatus;
  setStatus: Dispatch<SetStateAction<ContextMenuStatus>>;
}

const ContextMenu = ({ xPos, yPos, status, setStatus }: ContextMenuProps) => {
  const ref = useRef<HTMLUListElement>(null);
  const onClickAddFilter = useAddFilter();

  useEffect(() => {
    const closeContextMenu = (event: MouseEvent) => {
      if (!ref.current?.contains(event.target as Node)) {
        setStatus("Closed");
      }
    };
    document.addEventListener("mousedown", closeContextMenu);
    return () => document.removeEventListener("mousedown", closeContextMenu);
  }, [setStatus]);

  return (
    <>
      {status === "Open" ? (
        <ul className="context-menu" style={{ top: yPos, left: xPos }} ref={ref}>
          <li className="context-menu__first" onClick={onClickAddFilter}>
            Operation 1
          </li>
          <li className="context-menu__divider"></li>
          <li>Operation 2</li>
          <li>Operation 3</li>
          <li className="context-menu__divider"></li>
          <li>Operation 4</li>
          <li>Operation 5</li>
        </ul>
      ) : null}
    </>
  );
};

const TopBar = ({ names, selectedId }: { names: string[]; selectedId: number }) => {
  const color = (id: number) => (id === selectedId ? "#fc81a5" : "#04a7a7");
  return (
    <div className="top-bar">
      {names.map((name, id) => (
        <div
          className="top-bar__item"
          key={id}
          style={{ backgroundColor: color(id), borderColor: color(id), zIndex: names.length - id }}
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
  const [contextMenuXY, setContextMenuXY] = useState<{
    xPos: number;
    yPos: number;
    columnClickedName: string;
  }>({
    xPos: 0,
    yPos: 0,
    columnClickedName: "",
  });
  const [contextMenuStatus, setContextMenuStatus] = useState<ContextMenuStatus>("Closed");
  const onRightClick = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();
    setContextMenuXY({
      ...contextMenuXY,
      xPos: e.pageX,
      yPos: e.pageY,
    });
    setContextMenuStatus("Open");
  };
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
        onContextMenu={(e) => {
          e.preventDefault();
          setContextMenuXY({ ...contextMenuXY, columnClickedName: header?.[i] || "" });
        }}
      />
    );
  });
  return (
    <>
      <ContextMenu {...contextMenuXY} status={contextMenuStatus} setStatus={setContextMenuStatus} />
      <div className="frame__table" onContextMenu={onRightClick}>
        {columns}
      </div>
    </>
  );
};

export const DataHandler = () => {
  const [offset, setOffset] = useState<number>(0);

  const { header, slice, loading, names, selectedId } = useUIWorkerHandler({ offset });

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
          <TopBar names={names} selectedId={selectedId} />
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
