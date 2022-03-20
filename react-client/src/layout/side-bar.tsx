import React from "react";
import Reader from "../reader";
import "./side-bar.css";

const SideBar: React.FC = () => {
  return (
    <nav className="side-bar">
      <Reader />
    </nav>
  );
};

export default SideBar;
