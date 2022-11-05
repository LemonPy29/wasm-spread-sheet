import { map, none, of, Option } from "fp-ts/lib/Option";

type Node<T> = { inner: T; next: Option<Node<T>> };

type List<T> = {
  head: Option<Node<T>>;
};

export default class ListImpl<T extends { id: number }> {
  private data: List<T>;

  constructor() {
    this.data = { head: none };
  }

  isEmpty(): boolean {
    return this.data.head === none;
  }

  push(el: T) {
    const head = {
      inner: el,
      next: this.data.head,
    };

    this.data = { head: of(head) };
  }

  pop(): Option<T> {
    return map((node: Node<T>) => {
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

  find(id: number): Option<T> {
    for (const el of this.iter()) {
      if (el.id === id) {
        return of(el);
      }
    }
    return none;
  }

  insertAt(id: number, el: T) {
    let ptr = this.data.head;

    while (ptr._tag === "Some") {
      const node = ptr.value;

      if (node.inner.id === id) {
        const next = {
          inner: el,
          next: node.next,
        };

        ptr.value.next = of(next);
        break;
      }

      ptr = node.next;
    }
  }

  replaceAt(id: number, el: T) {
    let ptr = this.data.head;

    while (ptr._tag !== "None") {
      const node = ptr.value;

      if (node.inner.id === id) {
        ptr.value.inner = el;
        break;
      }

      ptr = node.next;
    }
  }
}
