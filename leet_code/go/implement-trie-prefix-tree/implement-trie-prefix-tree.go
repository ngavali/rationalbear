package main

import "fmt"

type Trie struct {
    charMap [26]*Trie
    isword bool
}

func Constructor() Trie {
    return Trie{[26]*Trie{}, false}
}

func (this *Trie) Insert(word string)  {
    for i := range word {
        if this.charMap[word[i] - 97] == nil {
            this.charMap[word[i] - 97] = &Trie{[26]*Trie{}, false}
            this = this.charMap[word[i] - 97]
        } else {
            this = this.charMap[word[i] - 97]
        }
    }
    this.isword = true
}

func (this *Trie) Search(word string) (hasWord bool) {
    for i := range word {
        if this.charMap[word[i] - 97] == nil {
            return false
        }
        this = this.charMap[word[i] - 97]
    }
    hasWord = this.isword
    return
}

func (this *Trie) StartsWith(prefix string) (bool) {
    for i := range prefix {
        if this.charMap[prefix[i] - 97] == nil {
            return false
        }
        this = this.charMap[prefix[i] - 97]
    }
    return true
}

func main() {
    /*
    ["Trie","insert","insert","insert","insert","insert","insert","search","search","search","search","search","search","search","search","search","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith"]
[[],["app"],["apple"],["beer"],["add"],["jam"],["rental"],["apps"],["app"],["ad"],["applepie"],["rest"],["jan"],["rent"],["beer"],["jam"],["apps"],["app"],["ad"],["applepie"],["rest"],["jan"],["rent"],["beer"],["jam"]]
    */
  obj := Constructor();
  obj.Insert("app");
  obj.Insert("apple");
  obj.Insert("beer");
  obj.Insert("add");
  obj.Insert("jam");
  obj.Insert("rental");
  fmt.Println(obj.Search("apps"))
  fmt.Println(obj.Search("app"))
  fmt.Println(obj.Search("ad"))
  fmt.Println(obj.Search("applepie"))
  fmt.Println(obj.Search("rest"))
  fmt.Println(obj.Search("jan"))
} 
