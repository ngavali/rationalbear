package main

import (
	"testing"
)

func TestAddBinary(t *testing.T) {

    inputs := [][]string{
        {"1001", "101"},
        {"01", "111"},
        {"110", "1100"},
        {"0", "0"},
    }
    outputs := []string{
        "1110",
        "1000",
        "10010",
        "0",
    }

    for i, input := range inputs {
        want := outputs[i]
        got := addBinary(input[0], input[1])
        if got != want {
            t.Errorf("For input=%+v\n\tWant=%s but got=%s", input, want, got)
        }
    }
}
