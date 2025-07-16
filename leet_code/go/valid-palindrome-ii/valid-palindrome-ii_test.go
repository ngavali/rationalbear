package main

import "testing"

func TestValidPalindrome(t *testing.T) {

    inputs := []string{
        "aba",
        "abca",
        "abc",
        "cbbcc",
    }
    outputs := []bool{
        true,
        true,
        false,
        true,
    }
    for i, input := range inputs {
        want := outputs[i]
        got := validPalindrome(input)
        if got!=want {
            t.Errorf("For input=%s, want=%t but got=%t", input, want, got)
        }
    }
}
