import "../styles/side-bar.css";
import { SliderProps } from "./types";

export const Slider = ({ checked, disabled, onChange }: SliderProps) => {
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
