package main

import "testing"

func TestIsPalindrome(t *testing.T) {

    inputs := []string{
        "A man, a plan, a canal: Panama",
        "abc",
        "aabbaa",
        "race a car",
        " ",
    }

    outputs := []bool{
        true,
        false,
        true,
        false,
        true,
    }

    for i, input := range inputs {
        want := outputs[i]
        got := isPalindrome(input)
        if got!= want {
            t.Errorf("For input=%s, Want=%t but Got=%t", input, want, got)
        }
    }

}
