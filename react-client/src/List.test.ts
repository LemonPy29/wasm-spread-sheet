import { getOrElseW } from "fp-ts/lib/Option";
import { ListImpl } from "./globalDataHandler";

test("push element to list", () => {
  const list = new ListImpl();
  list.push({ id: 0 });

  const el = getOrElseW(() => "nothing")(list.find(0));
  expect(el).toStrictEqual({ id: 0 });

  const nothing = getOrElseW(() => "nothing")(list.find(1));
  expect(nothing).toBe("nothing");
});

test("insert at the middle of the list", () => {
  const list = new ListImpl();
  list.push({ id: 0 });
  list.push({ id: 2 });
  list.insertAt(2, { id: 1 });

  const iter = list.iter();
  const two = iter.next();
  expect(two.value).toStrictEqual({ id: 2 });
  const one = iter.next();
  expect(one.value).toStrictEqual({ id: 1 });
  const zero = iter.next();
  expect(zero.value).toStrictEqual({ id: 0 });
});

test("replace at the middle of the list", () => {
  const list = new ListImpl();
  list.push({ id: 0 });
  list.push({ id: 1 });
  list.push({ id: 2 });
  list.replaceAt(1, { id: 47 })

  const iter = list.iter();
  const two = iter.next();
  expect(two.value).toStrictEqual({ id: 2 });
  const fortySeven = iter.next();
  expect(fortySeven.value).toStrictEqual({ id: 47 });
  const zero = iter.next();
  expect(zero.value).toStrictEqual({ id: 0 });
});
