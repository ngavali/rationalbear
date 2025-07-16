package main

import "testing"

func TestIsAnagram(t *testing.T) {

    inputs := [][]string{
        {"anagram","nagaram"},
        {"rat","car"},
    }

    outputs:= []bool{
        true,
        false,
    }

    for i, input := range inputs{
        want := outputs[i]
        got := isAnagram(input[0], input[1])
        if got!=want{
            t.Errorf("For input=%+v Want=%t, but Got=%t", input, want, got)
        }
    }

}
