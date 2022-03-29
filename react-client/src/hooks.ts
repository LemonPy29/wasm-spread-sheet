import { tryCatch } from "fp-ts/TaskEither";
import { foldW, none, Option, some } from "fp-ts/lib/Option";
import React from "react";
import { map } from "fp-ts/Either";

export function useTaskEither<T>(unsafePromise: Promise<T>) {
  const [data, setData] = React.useState<Option<T>>(none);

  const te = tryCatch(
    () => unsafePromise,
    (reason: unknown) => new Error(`${reason}`)
  );

  foldW(
    async () => {
      const safePromise = await te();
      const mapResponse = map((response: T) => setData(some(response)));
      mapResponse(safePromise);
    },
    () => {}
  )(data);

  return data;
}
