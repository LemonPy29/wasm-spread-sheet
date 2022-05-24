import Reader from "../reader";
import "./side-bar.css";
import { pipe } from "fp-ts/lib/function";
import { getOrElse, map } from "fp-ts/lib/Option";
import { FunctionComponent } from "react";
import { CheckBoxProps, SchemaUIProps, SideBarProps } from "../components.interface";

export const CheckBox: FunctionComponent<CheckBoxProps> = ({ checked, disabled, onChange }) => {
  return (
    <div className="checkBox">
      <label>
        <input type="checkbox" checked={checked} disabled={disabled} onChange={onChange} />
        Has a header?
      </label>
    </div>
  );
};

const SchemaUI: FunctionComponent<SchemaUIProps> = ({ schema }) => {
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
