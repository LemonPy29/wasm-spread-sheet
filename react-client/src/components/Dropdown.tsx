export const Dropdown = ({ options }: { options: string[] }) => {
  return (
    <ul className="dropdown">
      {options.map((el, idx) => (
        <div key={idx}>
          <li>{el}</li>
        </div>
      ))}
    </ul>
  );
};
