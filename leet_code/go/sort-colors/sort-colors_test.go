package main

import "testing"

func TestSortColors(t *testing.T) {

    inputs := [][]int{
        {2,0,2,1,1,0},
        {2,0,1},
        {1,2,0},
        {2,1,2},
    }
    outputs := [][]int{
        {0,0,1,1,2,2},
        {0,1,2},
        {0,1,2},
        {1,2,2},
    }
    for i, input := range inputs {
        want := outputs[i]
        sortColors(input)
        for j := range want {
            if input[j] != want[j] {
                t.Errorf("For input[%d] want=%+v but got=%+v\n", i, want, input)
            }
        }
    }
}
