package main

func longestPalindrome(s string) int {

    length := 0
    alphaMapOnes := [52]int{}
    alphaMapTwos := [52]int{}

    for _, c := range s {
        switch {
            case 'a' <= c && c <= 'z' :
                if alphaMapOnes[c-'a'] == 0 {
                    alphaMapOnes[c-'a']++
                } else {
                    alphaMapOnes[c-'a']--
                    alphaMapTwos[c-'a']++
                }
            case 'A' <= c && c <= 'Z':
                if alphaMapOnes[c-'A'+26] == 0 {
                    alphaMapOnes[c-'A'+26]++
                } else {
                    alphaMapOnes[c-'A'+26]--
                    alphaMapTwos[c-'A'+26]++
                }
        }
    }

    for _, pairs := range alphaMapTwos {
       length += 2 * pairs
    }

    for _, single := range alphaMapOnes {
        if single == 1 {
            length ++
            break
        }
    }
    return length

}
