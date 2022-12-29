import "../styles/table-ui.css";
import { DEFAULT_N_COLS } from "../constants";
import { Column } from "./Column";
import { FrameProps } from "./types";

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
