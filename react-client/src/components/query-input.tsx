import { useState } from "react";
import { useCommand } from "../hooks/worker";
import SuggestionTree from "../suggestion-tree";
import "../styles/query-input.css";

const Dropdown = ({ options }: { options: string[] }) => {
  return (
    <ul className="dropdown">
      {options.map((el, idx) => (
        <div>
          <li key={idx}>{el}</li>
          <div className="section-divider"></div>
        </div>
      ))}
    </ul>
  );
};

const QueryInput = () => {
  const [dropdownOptions, setDropdownOptions] = useState<string[]>([]);
  const commandTx = useCommand();
  const suggestionsGenerator = new SuggestionTree();
  suggestionsGenerator.append("Filter");
  suggestionsGenerator.append("Fire");
  suggestionsGenerator.append("Average");

  return (
    <div className="query-bar">
      <input
        placeholder="Write an instruction"
        className="query"
        onChange={(e) => {
          const target = e.target as HTMLInputElement;
          const suggestions = suggestionsGenerator.suggest(target.value);
          setDropdownOptions(suggestions);
          suggestionsGenerator.flush();
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
