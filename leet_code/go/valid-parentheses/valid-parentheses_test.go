package main

import "testing"

func TestIsValid(t *testing.T) {

    inputs := []string{
        "]",
        "[",
        "()",
        "()[]{}",
        "(]",
    }
    outputs:= []bool{
        false,
        false,
        true,
        true,
        false,
    }

    for i, input := range inputs {
        want:=outputs[i]
        got := isValid(input)
        if got!=want {
            t.Errorf("For input=%s, want=%t, but got=%t", inputs[i], want, got)
        }
    }

}
