export type ExtractPayload<T> = T extends { payload: infer U } ? U : never;

export type CommandSendMessage = {
  type: "command";
  payload: {
    id: number;
    command: string;
  };
};

export type ParsingSendMessage = {
  type: "parsing";
  payload: {
    id: number;
    name: string;
    chunk: Uint8Array;
    header: boolean;
  };
};

export type ChunkSendMessage = {
  type: "getChunk";
  payload: {
    id: number;
    offset: number;
    len: number;
  };
};

export type ProcessRemainderSendMessage = {
  type: "processRemainder";
  payload: {
    id: number;
  };
};

export type HeaderSendMessage = {
  type: "getHeader";
  payload: { id: number };
};

export type SumColSendMessage = {
  type: "sumCol";
  payload: {
    id: number;
    columnName: string;
  };
};

export type QueryableNamesSendMessage = { type: "names"; payload: null };

export type DistinctSendMessage = {
  type: "distinct";
  payload: {
    id: number;
    column: string;
  };
};

export type WorkerSendMessage =
  | ParsingSendMessage
  | ChunkSendMessage
  | ProcessRemainderSendMessage
  | HeaderSendMessage
  | SumColSendMessage
  | QueryableNamesSendMessage
  | DistinctSendMessage
  | CommandSendMessage;

type ParsingRecMessage = {
  type: "parsing";
  payload: {
    progress: number;
  };
};

type ChunkRecMessage = { type: "chunk"; payload: string[] };
type HeaderRecMessage = { type: "header"; payload: string[] };
type NamesRecMessage = { type: "names"; payload: string[] };
type SumColRecMessage = { type: "sumCol"; payload: string };
type DistinctRecMessage = { type: "distinct"; payload: string[] };
type AddSourceRecMessage = {
  type: "addSource";
  payload: {
    index: number;
    names: string[];
  };
};

export type WorkerRecMessage =
  | ChunkRecMessage
  | HeaderRecMessage
  | ParsingRecMessage
  | NamesRecMessage
  | SumColRecMessage
  | DistinctRecMessage
  | AddSourceRecMessage;
