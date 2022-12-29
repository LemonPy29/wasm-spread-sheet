import "../styles/table-ui.css";

export const TableSelector = ({
  names,
  selectedId,
  onClick,
}: {
  names: string[];
  selectedId: number;
  onClick: (index: number) => void;
}) => {
  const color = (id: number) => (id === selectedId ? "#fc81a5" : "#04a7a7");

  return (
    <div className="top-bar">
      {names.map((name, index) => (
        <div
          className="top-bar__item"
          key={index}
          style={{
            backgroundColor: color(index),
            borderColor: color(index),
            zIndex: names.length - index,
          }}
          onClick={() => onClick(index)}
        >
          <span>{name}</span>
        </div>
      ))}
    </div>
  );
};
