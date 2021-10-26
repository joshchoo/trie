import { Trie } from ".";

describe("Trie", () => {
  it("returns true when inserting and searching for the inserted word", () => {
    const trie = new Trie();
    trie.insert("hello");
    const result = trie.search("hello");
    expect(result).toBeTruthy();
  });

  it("returns false when searching for a non-existent word", () => {
    const trie = new Trie();
    const result = trie.search("hello");
    expect(result).toBeFalsy();
  });
});
