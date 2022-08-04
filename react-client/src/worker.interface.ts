export type EqualToFilter = {
  type: "equalTo";
  id: number;
  column: string;
  bytes: Uint8Array;
};

export type FilterTypes = EqualToFilter;

export type AddFilterSendMessage = {
  type: "addFilter";
  payload: FilterTypes;
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

export type QueryableNamesSendMessage = { type: "names" };

export type WorkerSendMessage =
  | ParsingSendMessage
  | ChunkSendMessage
  | ProcessRemainderSendMessage
  | HeaderSendMessage
  | SumColSendMessage
  | QueryableNamesSendMessage
  | AddFilterSendMessage;

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
type AddFilterRecMessage = {
  type: "addFilter";
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
  | AddFilterRecMessage;
