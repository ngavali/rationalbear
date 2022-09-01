package main

import "testing"

func TestContainsDuplicate(t *testing.T) {

    inputs := [][]int{
        {1,2,3,1},
        {1,2,3,4},
        {1,1,1,3,3,4,3,2,4,2},
    }
    outputs := []bool{
        true,
        false,
        true,
    }

    for i, input := range inputs {
        want := outputs[i]
        got := containsDuplicate(input)
        if got!=want {
            t.Errorf("For input=%+v\n\tWant=%t but got=%t\n",input, want,got)
        }
    }

}
