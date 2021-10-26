export class Trie {
  constructor() {
    this.root = new Node(null);
  }

  insert(word) {
    let current = this.root;
    word.split("").forEach((char) => {
      current.isLeaf = false;
      if (current.children[char] === undefined) {
        current.children[char] = new Node(char);
      }
      current = current.children[char];
    });
    current.isLeaf = true;
  }

  search(word) {
    let current = this.root;
    for (let i = 0; i < word.length; i++) {
      const char = word[i];
      if (current.children[char] === undefined) {
        return false;
      }
      current = current.children[char];
    }
    return true;
  }
}

export class Node {
  constructor(value) {
    this.value = value;
    this.children = {};
    this.isLeaf = false;
  }
}
