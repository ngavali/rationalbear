package main

import "testing"

func TestCanConstruct(t *testing.T) {
    inputs := [][]string{
        {"a", "z"},
        {"aa", "ab"},
        {"aa" , "aab"},
    }
    outputs := []bool{
        false,
        false,
        true,
    }
    for i, input := range inputs {
        want := outputs[i]
        got := canConstruct(input[0], input[1])
        if got != want {
            t.Errorf("Ïnput %+v, want=%t got=%t", input, want, got )
        }
    }
}
