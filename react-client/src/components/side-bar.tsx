import Reader from "./reader";
import "../styles/side-bar.css";
import { FunctionComponent } from "react";
import { CheckBoxProps } from "./components.interface";
import { useGlobalStore } from "../hooks/store";

const CheckBox: FunctionComponent<CheckBoxProps> = ({ checked, disabled, onChange }) => {
  return (
    <div className="has-header">
      <label className="switch">
        <input type="checkbox" checked={checked} disabled={disabled} onChange={onChange} />
        <span className="slider"></span>
      </label>
      <span className="has-header__text sidebar__text">Has a header</span>
    </div>
  );
};

const SideBar = () => {
  const [headerBox, disableHeaderBox, toggleHeaderBox] = useGlobalStore((state) => [
    state.headerBox,
    state.disableHeaderBox,
    state.toggleHeaderBox,
  ]);

  return (
    <nav className="side-bar">
      <Reader />
      <CheckBox
        disabled={disableHeaderBox}
        checked={headerBox}
        onChange={() => toggleHeaderBox()}
      />
    </nav>
  );
};

export default SideBar;
