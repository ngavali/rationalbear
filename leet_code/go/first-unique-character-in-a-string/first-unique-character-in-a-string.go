package main

func firstUniqChar(s string) int {

    charMap := make([]int,26)

    for i := range s {
        charMap[s[i] - 97]++
    }

    for i := range s {
        if charMap[s[i] - 97] == 1 {
            return i
        }
    }

    return -1

}

func main() {

}
