import { useState } from "react";
import { useCommand } from "../hooks/worker";
import "../styles/query-input.css";
import Trie from "../trie/trie";
import { Dropdown } from "./Dropdown";

const QueryInput = () => {
  const [dropdownOptions, setDropdownOptions] = useState<string[]>([]);
  const commandTx = useCommand();
  const trie = new Trie();
  trie.append("Filter");
  trie.append("Fire");
  trie.append("Average");

  return (
    <div className="query-bar">
      <input
        placeholder="Write an instruction"
        className="query"
        onChange={(e) => {
          const target = e.target as HTMLInputElement;
          const suggestions = trie.suggest(target.value);
          setDropdownOptions(suggestions);
          trie.flush();
        }}
        onBlur={() => setDropdownOptions([])}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            commandTx(e.currentTarget.value);
          }
        }}
      />
      <Dropdown options={dropdownOptions} />
    </div>
  );
};

export default QueryInput;
