import React from "react";
import { useGlobalStore } from "../hooks/store";
import { useParseFileChunk, useParseFileTail, useWorkerStore } from "../hooks/worker";
import "../styles/reader.css";

const Reader = () => {
  const [csvReaderStatus, setCsvReaderStatus, toggleDisableHeaderBox] = useGlobalStore((state) => [
    state.csvReaderStatus,
    state.setCsvReaderDataStatus,
    state.toggleDisableHeaderBox,
  ]);
  const { progress } = useWorkerStore();
  const fileParser = useParseFileChunk();
  const tailParser = useParseFileTail();

  const fileHandler = async (input: React.ChangeEvent<HTMLInputElement>) => {
    setCsvReaderStatus("Waiting");
    toggleDisableHeaderBox();

    const file = input.currentTarget.files![0];
    const reader = file.stream().getReader();

    while (true) {
      const { done, value } = await reader.read();
      if (value) {
        fileParser(file.name, value);
      }
      if (done) {
        tailParser();
        break;
      }
    }
  };

  React.useEffect(() => {
    if (progress === 1 && csvReaderStatus === "Waiting") {
      setTimeout(() => {
        setCsvReaderStatus("HeaderPhase");
      }, 1000);
    }
  }, [progress, csvReaderStatus, setCsvReaderStatus]);

  return (
    <>
      <label htmlFor={"upload-button"}>
        <div className="file-input">Choose your file</div>
      </label>
      <input
        type="file"
        id="upload-button"
        style={{ display: "none" }}
        onChange={fileHandler}
      ></input>
    </>
  );
};

export default Reader;
