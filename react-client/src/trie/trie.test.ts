import Trie from "./trie";

test("Append elements into the tree", () => {
  const tree = new Trie();
  tree.append("bar");
  tree.append("baz");
  const suggestions = tree.suggest("ba");

  expect(suggestions).toHaveLength(2);
})
