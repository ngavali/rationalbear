package main

func isAnagram(s string, t string) bool {

    if len(s)!=len(t) {
        return false
    }

    alphaMap := make([]int,26,26)

    for i := range s {
        alphaMap[s[i] - 'a']++
        alphaMap[t[i] - 'a']--
    }

    for i := range alphaMap {
        if alphaMap[i] != 0 {
            return false
        }
    }

    return true
}

func XisAnagram(s string, t string) bool {

    if len(s)!=len(t) {
        return false
    }

    alphaMap := make([]int,26)

    for i := range s {
        alphaMap[s[i] - 97]++
    }

    for j := range t {
        if alphaMap[t[j] - 97] == 0 {
            return false
        }
        alphaMap[t[j] - 97]--
    }

    return true
}
