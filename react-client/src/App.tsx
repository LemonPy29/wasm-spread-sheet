import "./App.css";
import { DataHandler } from "./components/table-ui";
import SideBar from "./components/side-bar";

function App() {
  return (
    <div className="App">
      <SideBar />
      <header className="App-header">
        <DataHandler />
      </header>
    </div>
  );
}

export default App;
