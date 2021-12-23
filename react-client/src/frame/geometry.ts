import {
  moveTo,
  chainPipe,
  quadraticCurveTo,
  lineTo,
  fillPath,
  drawPath,
  textNode,
  writeNode,
} from "./canvas-api";

export const N_ROWS = 100;
export const DEFAULT_COLUMNS = 10;

interface FigConfig {
  x: number;
  y: number;
  width: number;
  height: number;
}

interface IndexConfig extends FigConfig {
  sep: number;
  xoffset: number;
  yoffset: number;
}

function topLeftRounded(
  { x, y, width, height }: FigConfig,
  eps: number = 15
): Path2D {
  const figure = new Path2D();
  const ATop = { x: x + eps, y };
  const AMid = { x: ATop.x - eps, y: ATop.y };
  const ABot = { x: ATop.x - eps, y: ATop.y + eps };

  const B = { x: ABot.x, y: ABot.y + height - eps };
  const C = { x: B.x + width + eps, y: B.y };
  const D = { x: C.x, y: C.y - height };

  chainPipe(
    moveTo(ATop),
    quadraticCurveTo(AMid, ABot),
    lineTo(B),
    lineTo(C),
    lineTo(D),
    lineTo(ATop)
  )(figure);
  return figure;
}

function topRightRounded(
  { x, y, width, height }: FigConfig,
  eps: number = 15
): Path2D {
  const figure = new Path2D();

  const A = { x, y };

  const BTop = { x: A.x + width - eps, y: A.y };
  const BMid = { x: BTop.x + eps, y: BTop.y };
  const BBot = { x: BTop.x + eps, y: BTop.y + eps };

  const C = { x: BMid.x, y: BMid.y + height };
  const D = { x: C.x - width, y: C.y };

  chainPipe(
    moveTo(A),
    lineTo(BTop),
    quadraticCurveTo(BMid, BBot),
    lineTo(C),
    lineTo(D),
    lineTo(A)
  )(figure);
  return figure;
}

export function topRounded(
  { x, y, width, height }: FigConfig,
  eps: number = 15
): Path2D {
  const figure = new Path2D();
  const ATop = { x: x + eps, y };
  const AMid = { x: ATop.x - eps, y: ATop.y };
  const ABot = { x: ATop.x - eps, y: ATop.y + eps };

  const B = { x: ABot.x, y: ABot.y + height - eps };
  const C = { x: B.x + width + eps, y: B.y };

  const DBot = { x: C.x, y: C.y - height + eps };
  const DMid = { x: DBot.x, y: DBot.y - eps };
  const DTop = { x: DBot.x - eps, y: DBot.y - eps };

  chainPipe(
    moveTo(ATop),
    quadraticCurveTo(AMid, ABot),
    lineTo(B),
    lineTo(C),
    lineTo(DBot),
    quadraticCurveTo(DMid, DTop),
    lineTo(ATop)
  )(figure);
  return figure;
}

export function rect({ x, y, width, height }: FigConfig): Path2D {
  const figure = new Path2D();
  figure.rect(x, y, width, height);
  return figure;
}

export function parallelLines({ x, y, width, height }: FigConfig) {
  const figure = new Path2D();

  const A = { x, y };
  const B = { x: A.x, y: A.y + height };
  const C = { x: B.x + width, y: B.y };
  const D = { x: C.x, y: C.y - height };

  chainPipe(moveTo(A), lineTo(B), moveTo(C), lineTo(D))(figure);

  return figure;
}

interface Drawable {
  layout: Path2D[] | Path2D | Drawable[];
  draw: (ctx: CanvasRenderingContext2D) => void;
}

export class Column implements Drawable {
  layout: Path2D[] = [];
  header: { _name: textNode; shape: Path2D; sep: Path2D };
  config: FigConfig = {} as FigConfig;
  protected length: number = N_ROWS;

  constructor({ x, y, width, height }: FigConfig, name: string, offset = 0) {
    this.config = { x, y, width, height };
    for (let i = 0; i < N_ROWS; i++) {
      const ys = y + height * i;
      this.layout.push(rect({ x: x + offset, y: ys, width, height }));
    }

    const _name = {
      v: { x: x + 0.5 * width, y: y + 0.6 * height },
      text: name,
    };

    this.header = { _name, shape: new Path2D(), sep: new Path2D() };
  }

  set headerShape(fig: Path2D) {
    this.header.shape = fig;
  }

  set headerSep(fig: Path2D) {
    this.header.sep = fig;
  }

  draw(ctx: CanvasRenderingContext2D) {
    const drawPipe = chainPipe(
      drawPath(this.layout, { R: 255, G: 255, B: 255 })
    );

    drawPipe(ctx);
  }

  drawNodes(nodes: string[], ctx: CanvasRenderingContext2D) {
    let _nodes = nodes.map((s, i) => ({
      v: {
        x: this.config.x + 0.5 * this.config.width,
        y: this.config.y + this.config.height * i + 0.6 * this.config.height,
      },
      text: s,
    }));
    writeNode(_nodes, { R: 255, G: 255, B: 255, size: 16, align: "center" })(
      ctx
    );
  }

  drawHeader(ctx: CanvasRenderingContext2D) {
    const drawPipe = chainPipe(
      fillPath(this.header.shape, { R: 179, G: 102, B: 255 }),
      drawPath(this.header.sep, { R: 255, G: 255, B: 255 }),
      writeNode(this.header._name, {
        R: 255,
        G: 255,
        B: 255,
        font: "monospace",
        align: "center",
        size: 16,
      })
    );

    drawPipe(ctx);
  }
}

export class LeftColumn extends Column {
  constructor({ x, y, width, height }: FigConfig, name: string, r: number) {
    super({ x, y, width, height }, name);
    this.headerShape = topLeftRounded({ x, y, width, height }, r);
  }
}

export class RightColumn extends Column {
  constructor({ x, y, width, height }: FigConfig, name: string, r: number) {
    super({ x, y, width, height }, name);
    this.headerShape = topRightRounded({ x, y, width, height }, r);
  }
}

export class CentralColumn extends Column {
  constructor(config: FigConfig, name: string) {
    super(config, name);
    this.headerShape = rect(config);
    this.headerSep = parallelLines(config);
  }
}

export class Frame implements Drawable {
  nCols: number;
  private config: FigConfig;
  private names: string[];
  layout: Column[] = [];

  constructor(
    x: number,
    y: number,
    width: number,
    height: number,
    nCols?: number,
    names?: string[]
  ) {
    this.config = { x, y, width, height };
    this.nCols = nCols || DEFAULT_COLUMNS;
    this.names =
      names ||
      Array.from({ length: this.nCols }, (_, i) => String.fromCharCode(65 + i));

    this.reshape();
  }

  private reshape() {
    this.layout = [];
    const lastIdx = this.nCols - 1;
    const eps = this.config.height / 2;

    const left = new LeftColumn(this.config, this.names[0], eps);
    this.layout.push(left);

    for (let i = 1; i < lastIdx; i++) {
      const xs = this.config.x + i * this.config.width;
      const col = new CentralColumn(
        {
          x: xs,
          y: this.config.y,
          width: this.config.width,
          height: this.config.height,
        },
        this.names[i]
      );
      this.layout.push(col);
    }

    const right = new RightColumn(
      {
        x: this.config.x + lastIdx * this.config.width,
        y: this.config.y,
        width: this.config.width,
        height: this.config.height,
      },
      this.names[lastIdx],
      eps
    );
    this.layout.push(right);
  }

  set headers(names: string[]) {
    this.nCols = names.length;
    this.names = names;
    this.reshape();
  }

  get headers(): string[] {
    return this.names;
  }

  draw(ctx: CanvasRenderingContext2D) {
    for (const elem of this.layout) {
      elem.draw(ctx);
    }
  }

  drawColumnChunk(idx: number, nodes: string[], ctx: CanvasRenderingContext2D) {
    let col = this.layout[idx];
    col.drawNodes(nodes, ctx);
  }

  drawHeader(ctx: CanvasRenderingContext2D) {
    for (const elem of this.layout) {
      elem.drawHeader(ctx);
    }
  }
}

export class LeftIndex implements Drawable {
  layout: Path2D[] = [];
  textNodes: textNode[] = [];

  constructor({ x, y, width, height, xoffset, yoffset, sep }: IndexConfig) {
    for (let i = 0; i < N_ROWS; i++) {
      this.textNodes.push({
        v: { x: x + xoffset, y: y + i * height + yoffset },
        text: `${i}`,
      });
    }

    for (let i = 0; i < N_ROWS; i++) {
      this.layout.push(
        rect({ x, y: y + i * height, width, height: height - sep })
      );
    }
  }

  draw(ctx: CanvasRenderingContext2D) {
    const drawPipe = chainPipe(
      fillPath(this.layout, { R: 60, G: 127, B: 138, alpha: 0.5 }),
      writeNode(this.textNodes, { font: "monospace", R: 255, G: 255, B: 255 })
    );
    drawPipe(ctx);
  }
}

export const frame = new Frame(50, 0, 120, 30);

export const leftIndex = new LeftIndex({
  x: 5,
  y: 0,
  width: 30,
  height: 30,
  sep: 5,
  xoffset: 7,
  yoffset: 12,
});
