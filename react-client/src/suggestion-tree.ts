type TreeNode = {
  _isLeaf: boolean;
  childrens: Record<string, TreeNode>;
};

export default class SuggestionTree {
  private suggestions: string[] = [];
  private root: TreeNode | null = null;

  static emptyNode(): TreeNode {
    return { _isLeaf: false, childrens: {} };
  }

  append(word: string) {
    if (!this.root) this.root = SuggestionTree.emptyNode();
    let ptr = this.root;

    for (const letter of word) {
      if (!(letter in ptr.childrens)) {
        ptr.childrens[letter] = SuggestionTree.emptyNode();
      }
      ptr = ptr.childrens[letter];
    }

    ptr._isLeaf = true;
  }

  private find(word: string): TreeNode | null {
    let ptr = this.root;

    for (const letter of word) {
      if (letter in ptr!.childrens) {
        ptr = ptr!.childrens[letter];
      } else {
        return null;
      }
    }

    return ptr;
  }

  private fill(node: TreeNode, word: string) {
    if (node._isLeaf) {
      this.suggestions.push(word);
      return;
    }

    for (const letter in node.childrens) {
      this.fill(node.childrens[letter], word + letter);
    }
  }

  suggest(word: string) {
    const node = this.find(word);
    node && this.fill(node, word);

    return this.suggestions;
  }

  flush() {
    this.suggestions = [];
  }
}
