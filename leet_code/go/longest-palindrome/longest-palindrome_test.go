package main

import "testing"

func TestLongestPalindrome(t *testing.T) {
    inputs := []string{
        "abccccdd",
        "abc",
        "a",
        "AAAAAA",
        "Aa",
    }
    outputs := []int{
        7,
        1,
        1,
        6,
        1,
    }

    for i, input := range inputs {
        want := outputs[i]
        got := longestPalindrome(input)
        if got != want {
            t.Errorf("For input=%s, Want=%d but got=%d", input, want, got)
        }
    }
}
