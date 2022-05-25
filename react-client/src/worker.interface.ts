export type ParsingSendMessage = {
  type: "parsing";
  payload: { chunk: Uint8Array; header: boolean; remainder: Uint8Array };
};
export type ChunkSendMessage = { type: "getChunk"; payload: { offset: number; len: number } };
export type ProcessRemainderSendMessage = { type: "processRemainder"; payload: Uint8Array };
export type HeaderSendMessage = { type: "getHeader" };
export type WorkerSendMessage =
  | ParsingSendMessage
  | ChunkSendMessage
  | ProcessRemainderSendMessage
  | HeaderSendMessage;

type ParsingRecMessage = {
  type: "parsing";
  payload: {
    progress: number;
    remainder: Uint8Array;
  };
};
type ChunkRecMessage = { type: "chunk"; payload: string[] };
type HeaderRecMessage = { type: "header"; payload: string[] };
export type WorkerRecMessage = ChunkRecMessage | HeaderRecMessage | ParsingRecMessage;
