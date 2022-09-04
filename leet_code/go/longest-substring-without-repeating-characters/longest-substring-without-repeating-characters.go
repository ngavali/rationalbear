package main

var charMap = make([]int, 256)
func _lengthOfLongestSubstring(s string) int {

    if len(s) < 2 {
        return len(s)
    }

    for i := range charMap {
        charMap[i] = -1
    }
        start, end := 0, 0
        max_len := 0

    for end = range s {
                if charMap[s[end]] >= start {
                        start = charMap[s[end]] + 1
                }
                charMap[s[end]] = end
                if max_len < end-start+1 {
                             max_len = end - start+1
                        }
    }

    return max_len
}

func lengthOfLongestSubstring(s string) int {

    start, maxLen := 0, 0
    charMap := make(map[byte]int, len(s))

    for end := range s {
        if idx, ok := charMap[s[end]]; ok && idx >= start {
           start = idx+1 
        }
        if maxLen < end - start + 1 {
            maxLen = end - start + 1
        }
        charMap[s[end]] = end
    }

    return maxLen
}
