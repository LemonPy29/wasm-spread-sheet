import "../styles/table-ui.css";
import { match, P } from "ts-pattern";
import { HeaderProps } from "./types";

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
