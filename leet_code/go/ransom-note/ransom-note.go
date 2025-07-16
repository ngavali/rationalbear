package main

func canConstruct(ransomNote string, magazine string) bool {

    alphaArray := make([]int, 26)

    for i := range magazine {
        alphaArray[magazine[i] - 97]++
    }

    for i := range ransomNote {
        if alphaArray[ransomNote[i] - 97] == 0 {
            return false
        }
        alphaArray[ransomNote[i] - 97]--
    }

    return true

}
