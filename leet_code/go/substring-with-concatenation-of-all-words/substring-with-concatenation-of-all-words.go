package main

import "fmt"

func findSubstring(s string, words []string) []int {
	lw := len(words[0])
	wl := len(words)
	wordMap := map[string]int{}
	word := ""
	i := 0
	count := []int{}
	for i < len(s)-lw {
		w := s[i : i+lw]
		word += w
		if _, ok := wordMap[w]; !ok {
			wordMap[w]++
			i += lw
		} else {
			i += lw
			delete(wordMap, word[:lw])
			word = word[lw:]
		}
		if len(word) == wl*lw {
			fmt.Println(i, w, word)
			wordMap = map[string]int{}
			word = ""
			count = append(count, i-wl*lw)
		}
	}
	return count
}

func main() {
	fmt.Println(findSubstring("wordgoodgoodgoodbestword", []string{"word", "good", "best", "word"}))
	fmt.Println(findSubstring("barfoothefoobarman", []string{"foo", "bar"}))
}
