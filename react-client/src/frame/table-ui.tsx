import "./table-ui.css";
import { isDoneContext, readyToPullContext } from "../App";
import React from "react";

const DEFAULT_N_ROWS = 20;
const DEFAULT_N_COLS = 10;

type ColumnProps = { header: string; data?: string[] };
type FrameProps = { data?: string[][] };
type Chunk = { data: string[][] };
type State = { number: number; data: string[][] };
type Increment = { type: "increment" };
type Decrement = { type: "decrement" };
type DataSetter = { type: "dataSetter"; payload: string[][] };
type Action = Increment | Decrement | DataSetter;

const reducer = (state: State, action: Action): State => {
  if (action.type === "increment") {
    return {
      number: state.number + 1,
      data: state.data,
    };
  } else if (action.type === "decrement") {
    const _state = state.number - 1;
    return {
      number: _state < 0 ? 0 : _state,
      data: state.data,
    };
  } else if (action.type === "dataSetter") {
    return {
      number: state.number,
      data: action.payload,
    };
  } else {
    return state;
  }
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
        <div className="frame__header">{header}</div>
        {column}
      </div>
    </>
  );
};

export const FrameUI = ({ data }: FrameProps) => {
  const columns = Array.from(
    { length: data?.length || DEFAULT_N_COLS },
    (_, i) => {
      const header = String.fromCharCode(65 + i);
      return <Column key={i} header={header} data={data?.[i]} />;
    }
  );
  return (
    <>
      <div className="frame__table">{columns}</div>
    </>
  );
};

export const DataHandler = () => {
  const [state, dispatch] = React.useReducer(reducer, {
    number: 0,
    data: [],
  } as State);
  const { status, setSatus } = React.useContext(readyToPullContext);
  const { done } = React.useContext(isDoneContext);

  React.useEffect(() => {
    const fetchChunk = async () => {
      const { get_chunk } = await import("wasm");
      const timer = setInterval(() => {
        const offset = state.number * DEFAULT_N_ROWS;
        const chunk = get_chunk(offset, DEFAULT_N_ROWS) as Chunk;
        if (!chunk.data.some((el) => el.length === 0)) {
          dispatch({ type: "dataSetter", payload: chunk.data });
          clearInterval(timer);
        }
      }, 250);
    };

    const createFrame = async () => {
      const { Frame } = await import("wasm");
      const frame = new Frame();
      console.log(frame.schema);
    }

    if (status === "ReadyToUse") {
      fetchChunk();
      setSatus("Used");
    }

    if (done) {
      createFrame()
    }
  }, [done, state, status, setSatus]);

  const forwardHandler = () => {
    dispatch({ type: "increment" });
    setSatus("ReadyToUse");
  };

  const backwardHandler = () => {
    dispatch({ type: "decrement" });
    setSatus("ReadyToUse");
  };

  return (
    <>
      {status === "Waiting" ? (
        <div className="frame__spinner"></div>
      ) : (
        <div className="frame">
          <FrameUI data={state.data} />
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
