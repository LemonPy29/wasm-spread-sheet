import { Reader, chain } from "fp-ts/Reader";
import * as O from "fp-ts/Option";
import { flow } from "fp-ts/function";

export type vtx = {
  x: number;
  y: number;
};

export type textNode = {
  v: vtx;
  text: string;
};

type canvasContext = CanvasRenderingContext2D;
type reader<T> = Reader<T, void>;
export type elemOrArray<A> = A | A[];

export interface style {
  R?: number;
  G?: number;
  B?: number;
  alpha?: number;
}

export interface textStyle extends style {
  size?: number;
  font?: string;
  baseline?: string;
  align?: string;
}

interface rectangle {
  x: number;
  y: number;
  width: number;
  height: number;
}

const styleSafeProj = (ts?: textStyle | style): style => {
  return (ts = {
    R: ts?.R || 0,
    G: ts?.G || 0,
    B: ts?.B || 0,
    alpha: ts?.alpha || 1,
  } as style);
};

const textStyleSafeProj = (s?: textStyle): textStyle => {
  return {
    size: s?.size || 12,
    font: s?.font || "Arial",
    baseline: s?.baseline || "alphabetic",
    align: s?.align || "center",
  } as textStyle;
};

const setFillStyle =
  ({ R, G, B, alpha }: style): reader<canvasContext> =>
  (ctx) =>
    (ctx.fillStyle = `rgba(${R}, ${G}, ${B}, ${alpha})`);

const safeSetFillStyle = flow(styleSafeProj, setFillStyle);

const setStrokeStyle =
  ({ R, G, B, alpha }: style): reader<canvasContext> =>
  (ctx) =>
    (ctx.strokeStyle = `rgba(${R}, ${G}, ${B}, ${alpha})`);

const safeSetStrokeStyle = flow(styleSafeProj, setStrokeStyle);

const restoreStyle = (): reader<canvasContext> => (ctx) =>
  (ctx.fillStyle = "rgba(0, 0, 0, 1)");

const setTextBaseline =
  ({ baseline }: textStyle): reader<canvasContext> =>
  (ctx) =>
    (ctx.textBaseline = baseline as CanvasTextBaseline);

const setTextAlign =
  ({ align }: textStyle): reader<canvasContext> =>
  (ctx) =>
    (ctx.textAlign = align as CanvasTextAlign);

const safeSetTextAlign = flow(textStyleSafeProj, setTextAlign);

const setFont =
  ({ size, font }: textStyle): reader<canvasContext> =>
  (ctx) =>
    (ctx.font = `bold ${size}px ${font}`);

const safeSetFont = flow(textStyleSafeProj, setFont);

const safeSetTextBaseline = flow(textStyleSafeProj, setTextBaseline);

const _fillPath = (...paths: Path2D[]): reader<canvasContext>[] =>
  paths.map((path) => (ctx) => ctx.fill(path));

const _fillText = (...nodes: textNode[]): reader<canvasContext>[] =>
  nodes.map(
    ({ v, text }) =>
      (ctx) =>
        ctx.fillText(text, v.x, v.y)
  );

const _drawPath = (...paths: Path2D[]): reader<canvasContext>[] =>
  paths.map((path) => (ctx) => ctx.stroke(path));

export const clear =
  ({ x, y, width, height }: rectangle): reader<canvasContext> =>
  (ctx) =>
    ctx.clearRect(x, y, width, height);

export const chainPipe = <T>(...fns: reader<T>[]): reader<T> =>
  fns.slice(1).reduce((result, fn) => chain(() => fn)(result), fns[0]);

export const fillPath = (
  elems: elemOrArray<Path2D>,
  s?: style
): reader<canvasContext> =>
  chainPipe<canvasContext>(
    safeSetFillStyle(s),
    ...(elems instanceof Array ? _fillPath(...elems) : _fillPath(elems)),
    restoreStyle()
  );

export const drawPath = (
  elems: elemOrArray<Path2D>,
  s?: style
): reader<canvasContext> =>
  chainPipe<canvasContext>(
    safeSetStrokeStyle(s),
    ...(elems instanceof Array ? _drawPath(...elems) : _drawPath(elems)),
    restoreStyle()
  );

export const writeNode = (
  elems: elemOrArray<textNode>,
  ts?: textStyle
): reader<canvasContext> => {
  return chainPipe<canvasContext>(
    safeSetFont(ts),
    safeSetTextBaseline(ts),
    safeSetTextAlign(ts),
    safeSetFillStyle(ts),
    ...(elems instanceof Array ? _fillText(...elems) : _fillText(elems)),
    restoreStyle()
  );
};

export const onPathClick =
  (...paths: Path2D[]) =>
  (ctx: canvasContext) =>
  (e: MouseEvent) =>
    paths.map((path) =>
      ctx.isPointInPath(
        path,
        e.offsetX * window.devicePixelRatio,
        e.offsetY * window.devicePixelRatio
      )
    );

export const findClickedWithFilter =
  (blockedIdx: number[]) =>
  (clicked: boolean[]): O.Option<number> => {
    const idx = clicked.findIndex((elem: boolean) => elem);
    return O.fromPredicate((i: number) => i >= 0 && !blockedIdx.includes(i))(
      idx
    );
  };

export const findClicked = (b: boolean[]): O.Option<number> => {
  const idx = b.findIndex((elem: boolean) => elem);
  return O.fromPredicate((i: number) => i >= 0)(idx);
};

export const moveTo =
  (v: vtx): reader<Path2D> =>
  (path) =>
    path.moveTo(v.x, v.y);

export const lineTo =
  (v: vtx): reader<Path2D> =>
  (path) =>
    path.lineTo(v.x, v.y);

export const bezierCurveTo =
  (c: vtx, d: vtx, v: vtx): reader<Path2D> =>
  (path) =>
    path.bezierCurveTo(c.x, c.y, d.x, d.y, v.x, v.y);

export const quadraticCurveTo =
  (c: vtx, v: vtx): reader<Path2D> =>
  (path) =>
    path.quadraticCurveTo(c.x, c.y, v.x, v.y);
