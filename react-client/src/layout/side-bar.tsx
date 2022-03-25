import Reader from "../reader";
import "./side-bar.css";
import { pipe } from "fp-ts/lib/function";
import { getOrElse, map, Option } from "fp-ts/lib/Option";
import { FunctionComponent } from "react";

type SideBarProps = {
  schema: Option<Record<string, string>>;
};

const SchemaUI: FunctionComponent<SideBarProps> = ({ schema }) => {
  const names: JSX.Element[] = [];
  const types: JSX.Element[] = [];

  return pipe(
    schema,
    map((r: Record<string, string>) => {
      for (const key in r) {
        names.push(<div className="schema__keys">{key}</div>);
        types.push(<div className="schema__keys">{r[key]}</div>);
      }
      return (
        <div className="schema">
          <div className="schema__names">{names}</div>
          <div className="schema__types">{types}</div>
        </div>
      );
    }),
    getOrElse(() => <div>Nothing here</div>)
  );
};

const SideBar: FunctionComponent<SideBarProps> = ({ schema }) => {
  return (
    <nav className="side-bar">
      <Reader />
      <SchemaUI schema={schema} />
    </nav>
  );
};

export default SideBar;
