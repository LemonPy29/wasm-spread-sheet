import React from "react";
import Reader from "../reader";
import "./side-bar.css";
import { FunctionComponent } from "react";
import { CheckBoxProps } from "../components.interface";
import { metadataContext } from "../App";

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
  const { metadata, setMetadata } = React.useContext(metadataContext);

  return (
    <nav className="side-bar">
      <Reader />
      <CheckBox
        disabled={metadata.headerCheckBoxDisabled}
        checked={metadata.headerChecked}
        onChange={() =>
          setMetadata({
            ...metadata,
            headerChecked: !metadata.headerChecked,
          })
        }
      />
    </nav>
  );
};

export default SideBar;
