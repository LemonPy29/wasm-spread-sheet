import "./table-ui.css";
import { dataStatusContext, metadataContext, workerDataContext } from "../App";
import { Dispatch, SetStateAction, useContext, useEffect, useRef, useState } from "react";
import { ColumnProps, FrameProps, HeaderProps } from "../components.interface";
import { dataWorker } from "../reader";
import {
  ChunkSendMessage,
  HeaderSendMessage,
  QueryableNamesSendMessage,
} from "../worker.interface";
import { match, P } from "ts-pattern";

const DEFAULT_N_ROWS = 20;
const DEFAULT_N_COLS = 10;

type ContextMenuStatus = "Open" | "Closed";
interface ContextMenuProps {
  xPos: number;
  yPos: number;
  status: ContextMenuStatus;
  setStatus: Dispatch<SetStateAction<ContextMenuStatus>>;
}

const ContextMenu = ({ xPos, yPos, status, setStatus }: ContextMenuProps) => {
  const ref = useRef<HTMLUListElement>(null);

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
          <li className="context-menu__first">Operation 1</li>
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
          <li>Operation 2</li>

const TopBar = () => {
  const { workerDataState } = useContext(workerDataContext);

  useEffect(() => {
    const action: QueryableNamesSendMessage = { type: "names" };
    dataWorker.postMessage(action);
  }, []);

  return (
    <div className="top-bar">
      {workerDataState.names.map((name, id) => (
        <div className="top-bar__item" key={id}>
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

export const Column = ({ header, data }: ColumnProps) => {
  const column = Array.from({ length: DEFAULT_N_ROWS }, (_, i) => (
    <div className="frame__cell" key={i}>
      <div className="cell__text">{data?.[i] || ""}</div>
    </div>
  ));
  return (
    <>
      <div className="frame__column">
        <Header {...header} />
        {column}
      </div>
    </>
  );
};

export const FrameTable = ({ header, data }: FrameProps) => {
  const [contextMenuXY, setContextMenuXY] = useState<{ xPos: number; yPos: number }>({
    xPos: 0,
    yPos: 0,
  });
  const [contextMenuStatus, setContextMenuStatus] = useState<ContextMenuStatus>("Closed");
  const onRightClick = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault();
    setContextMenuXY({
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
  const { dataStatus, setDataStatus } = useContext(dataStatusContext);
  const { metadata } = useContext(metadataContext);
  const { workerDataState } = useContext(workerDataContext);
  const [offset, setOffset] = useState<number>(0);

  useEffect(() => {
    if (dataStatus === "headerPhase") {
      const action: HeaderSendMessage = { type: "getHeader", payload: { id: metadata.selectedId } };
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
          <TopBar />
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
};
