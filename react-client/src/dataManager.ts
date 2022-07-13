import { map, none, of, Option } from "fp-ts/lib/Option";

export interface Queryable {
  id: string;
  sliceAsJsStrings: (offset: number, len: number) => string[];
}

type List<T> = {
  head: Option<Node<T>>;
};

type Node<T> = { inner: T; next: Option<Node<T>> };

export class DataManager {
  private data: List<Queryable>;

  constructor() {
    this.data = { head: none };
  }

  push(queryable: Queryable) {
    const head = {
      inner: queryable,
      next: this.data.head,
    };

    this.data = { head: of(head) };
  }

  pop(): Option<Queryable> {
    return map((node: Node<Queryable>) => {
      this.data.head = node.next;
      return node.inner;
    })(this.data.head);
  }

  *iter() {
    let ptr = this.data.head;

    while (ptr._tag === "Some") {
      const node = ptr.value;
      ptr = node.next;
      yield node.inner;
    }
  }

  find(id: string): Option<Queryable> {
    for (const el of this.iter()) {
      if (el.id === id) {
        return of(el);
      }
    }
    return none;
  }

  insert_at(id: string, queryable: Queryable) {
    let ptr = this.data.head;

    while (ptr._tag !== "None") {
      const node = ptr.value;

      if (node.inner.id === id) {
        const next = {
          inner: queryable,
          next: node.next,
        };

        ptr.value.next = of(next);
      }
    }
  }
}
