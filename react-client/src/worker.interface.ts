export type ParsingSendMessage = {
  type: "parsing";
  payload: { id: number; name: string; chunk: Uint8Array; header: boolean; remainder: Uint8Array };
};
export type ChunkSendMessage = {
  type: "getChunk";
  payload: { id: number; offset: number; len: number };
};
export type ProcessRemainderSendMessage = {
  type: "processRemainder";
  payload: { id: number; remainder: Uint8Array };
};
export type HeaderSendMessage = {
  type: "getHeader";
  payload: { id: number };
};
export type SumColSendMessage = {
  type: "sumCol";
  payload: { id: number; columnName: string };
};
export type QueryableNamesSendMessage = { type: "names" };
export type WorkerSendMessage =
  | ParsingSendMessage
  | ChunkSendMessage
  | ProcessRemainderSendMessage
  | HeaderSendMessage
  | SumColSendMessage
  | QueryableNamesSendMessage;

type ParsingRecMessage = {
  type: "parsing";
  payload: {
    progress: number;
    remainder: Uint8Array;
  };
};
type ChunkRecMessage = { type: "chunk"; payload: string[] };
type HeaderRecMessage = { type: "header"; payload: string[] };
type NamesRecMessage = { type: "names"; payload: string[] };
type SumColRecMessage = { type: "sumCol"; payload: string };
export type WorkerRecMessage =
  | ChunkRecMessage
  | HeaderRecMessage
  | ParsingRecMessage
  | NamesRecMessage
  | SumColRecMessage;
