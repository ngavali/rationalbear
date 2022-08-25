package main

import "testing"

func TestSearch(t *testing.T) {
	inputs := [][]int{
		{9, -1, 0, 3, 5, 9, 12},
		{2, -1, 0, 3, 5, 9, 12},
        {13,-1,0,3,5,9,12},
	}
	outputs := []int{
		4,
		-1,
        -1,
	}

    for i, input := range inputs {
        want := outputs[i]
        got := search(input[1:], input[0])
        if got!= want{
            t.Errorf("For input[%d] want=%d but got=%d", input, want, got)
        }
    }

}
