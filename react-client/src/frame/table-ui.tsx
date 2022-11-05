import React from "react";
import { readyToPullContext } from "../App";
import { frame, leftIndex } from "./geometry";
import "./table-ui.css";

const WIDTH = 1600;
const HEIGHT = 5000;
const CHUNK_SIZE = 25;

const splitRow = (row: string): string[] => row.split("DELIMITER_TOKEN");

const generateCanvas = (
  canvasRef: React.RefObject<HTMLCanvasElement>,
  width: number,
  height: number
) => {
  const canvas = canvasRef.current!;
  const ratio = window.devicePixelRatio;
  canvas.width = width * ratio;
  canvas.height = height * ratio;
  canvas.style.width = width + "px";
  canvas.style.height = height + "px";
  canvas.getContext("2d")!.scale(ratio, ratio);

  const ctx = canvas.getContext("2d")!;
  return ctx;
};

const FrameUI = () => {
  const gridCanvasRef = React.useRef<HTMLCanvasElement>(null);
  const headerCanvasRef = React.useRef<HTMLCanvasElement>(null);
  const { ready, setReady } = React.useContext(readyToPullContext);
  const [offset, setOffset] = React.useState<number>(-1);

  React.useEffect(() => {
    const gridCtx = generateCanvas(gridCanvasRef, WIDTH, HEIGHT);
    const waitForFirstChunk = async () => {
      if (ready === "ReadyToUse") {
        const { get_header } = await import("wasm");
        const header = get_header();
        frame.headers = splitRow(header!);
        setOffset(0);
        setReady("Used");
      }
    };

    waitForFirstChunk();

    frame.draw(gridCtx);
    leftIndex.draw(gridCtx);

    const drawChunk = async () => {
      if (offset >= 0) {
        const { get_chunk } = await import("wasm");
        for (let i = 0; i < frame.nCols; i++) {
          const data = get_chunk(i, 0, CHUNK_SIZE);
          frame.drawColumnChunk(i, data, gridCtx);
        }
      }
    };

    drawChunk();
  }, [ready, setReady, offset]);

  React.useEffect(() => {
    const headerCtx = generateCanvas(headerCanvasRef, WIDTH, 30);
    frame.drawHeader(headerCtx);
  }, [ready]);

  let headerDivRef = React.useRef<HTMLDivElement>(null);
  const onScrollListener = (e: React.UIEvent<HTMLDivElement>) => {
    const div = headerDivRef.current!;
    div.scrollTo(e.currentTarget.scrollLeft, 0);
  };

  const gridRef = React.useRef<HTMLDivElement>(null);
  React.useEffect(() => {
    const div = gridRef.current!;
    let oldOffset = 0;
    const scrollCallback = () => {
      let scrollOffset = (div.scrollTop / (CHUNK_SIZE * 30)) | 0;
      if (oldOffset !== scrollOffset) {
        console.log(scrollOffset);
        oldOffset = scrollOffset;
      }
    };

    div.addEventListener("scroll", scrollCallback);

    return () => {
      div.removeEventListener("scroll", scrollCallback);
    };
  }, []);

  return (
    <>
      <div ref={headerDivRef} className="header">
        <canvas
          className="canvas-header"
          ref={headerCanvasRef}
          width={WIDTH}
          height={30}
        ></canvas>
      </div>
      <div ref={gridRef} className="grid" onScroll={onScrollListener}>
        <canvas
          className="canvas-grid"
          ref={gridCanvasRef}
          width={WIDTH}
          height={HEIGHT}
        ></canvas>
        ;
      </div>
    </>
  );
};

export default FrameUI;
