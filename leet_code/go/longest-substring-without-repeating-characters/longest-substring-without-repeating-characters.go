package main

import "fmt"

func lengthOfLongestSubstring(s string) int {

    if len(s) < 2 {
        return len(s)
    }
	//char_map := make(map[byte]int, len(s))
    charMap := make([]int, 128)
    for i := range charMap {
        charMap[i] = -1
    }
	start, end := 0, 0
	max_len, str_len := 0, len(s)

	for end < str_len {
		//if idx, ok := char_map[s[end]]; ok && idx >= start {
        if charMap[s[end]] >= start {   
        fmt.Println(start, end)
			start = charMap[s[end]] + 1
		}
		if max_len < end-start+1 {
			max_len = end - start + 1
		}
		charMap[s[end]] = end
		end++
	}
	return max_len
}

func main() {
	fmt.Println(lengthOfLongestSubstring("au"))
	fmt.Println(lengthOfLongestSubstring(" "))
	fmt.Println(lengthOfLongestSubstring("abcabcbb"))
	fmt.Println(lengthOfLongestSubstring("bbbbb"))
	fmt.Println(lengthOfLongestSubstring("pwwkew"))
	fmt.Println(lengthOfLongestSubstring("abba"))
	fmt.Println(lengthOfLongestSubstring(""))
	fmt.Println(lengthOfLongestSubstring("tmmzuxt"))
}
