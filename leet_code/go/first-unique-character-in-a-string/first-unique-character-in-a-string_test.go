package main

import "testing"

func TestFirstUniqChar(t *testing.T) {
    inputs := []string{
        "leetcode",
        "loveleetcode",
        "aabb",
    }
    outputs := []int{
        0,
        2,
        -1,
    }
    for i, input := range inputs {
        want := outputs[i]
        got := firstUniqChar(input)
        if got != want {
            t.Errorf("For input=%+v, want=%d but got=%d\n", input, want, got)
        }
    }

}
