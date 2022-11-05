import SuggestionTree from "./suggestion-tree";

test("Append elements into the tree", () => {
  const tree = new SuggestionTree();
  tree.append("bar");
  tree.append("baz");
  const suggestions = tree.suggest("ba");

  expect(suggestions).toHaveLength(2);
})
