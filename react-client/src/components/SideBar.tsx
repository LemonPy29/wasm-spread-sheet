import "../styles/side-bar.css";
import { useGlobalStore } from "../hooks/store";
import { Slider } from "./Slider";
import Reader from "./Reader";

export const SideBar = () => {
  const [headerBox, disableHeaderBox, toggleHeaderBox] = useGlobalStore((state) => [
    state.headerBox,
    state.disableHeaderBox,
    state.toggleHeaderBox,
  ]);

  return (
    <nav className="side-bar">
      <Reader />
      <Slider
        disabled={disableHeaderBox}
        checked={headerBox}
        onChange={() => toggleHeaderBox()}
      />
    </nav>
  );
};
