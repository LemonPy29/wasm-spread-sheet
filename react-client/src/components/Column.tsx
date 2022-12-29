import "../styles/table-ui.css";
import { DEFAULT_N_ROWS } from "../constants";
import { Header } from "./Header";
import { ColumnProps } from "./types";

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
