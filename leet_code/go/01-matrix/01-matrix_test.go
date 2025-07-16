package main

import (
	"testing"
)

type testCase struct {
    input  [][]int
    output [][]int
}

func TestUpdateMatrix(t *testing.T) {

    testCases := []testCase{
        {
            input:  [][]int{{0}, {1}},
            output: [][]int{{0}, {1}},
        },
        {
            input: [][]int{
                {0, 0, 0},
                {0, 1, 0},
                {0, 0, 0},
            },
            output: [][]int{
                {0, 0, 0},
                {0, 1, 0},
                {0, 0, 0},
            },
        },
        {
            input: [][]int{
                {0, 0, 0},
                {0, 1, 0},
                {1, 1, 1},
            },
            output: [][]int{
                {0, 0, 0},
                {0, 1, 0},
                {1, 2, 1},
            },
        },
        {
            input: [][]int{
                {1, 0, 1, 1, 0, 0, 1, 0, 0, 1},
                {0, 1, 1, 0, 1, 0, 1, 0, 1, 1},
                {0, 0, 1, 0, 1, 0, 0, 1, 0, 0},
                {1, 0, 1, 0, 1, 1, 1, 1, 1, 1},
                {0, 1, 0, 1, 1, 0, 0, 0, 0, 1},
                {0, 0, 1, 0, 1, 1, 1, 0, 1, 0},
                {0, 1, 0, 1, 0, 1, 0, 0, 1, 1},
                {1, 0, 0, 0, 1, 1, 1, 1, 0, 1},
                {1, 1, 1, 1, 1, 1, 1, 0, 1, 0},
                {1, 1, 1, 1, 0, 1, 0, 0, 1, 1},
            },
            output: [][]int{
                {1, 0, 1, 1, 0, 0, 1, 0, 0, 1},
                {0, 1, 1, 0, 1, 0, 1, 0, 1, 1},
                {0, 0, 1, 0, 1, 0, 0, 1, 0, 0},
                {1, 0, 1, 0, 1, 1, 1, 1, 1, 1},
                {0, 1, 0, 1, 1, 0, 0, 0, 0, 1},
                {0, 0, 1, 0, 1, 1, 1, 0, 1, 0},
                {0, 1, 0, 1, 0, 1, 0, 0, 1, 1},
                {1, 0, 0, 0, 1, 2, 1, 1, 0, 1},
                {2, 1, 1, 1, 1, 2, 1, 0, 1, 0},
                {3, 2, 2, 1, 0, 1, 0, 0, 1, 1},
            },
        },
    }

    for _, testCase := range testCases {
        want := testCase.output
        got := updateMatrix(testCase.input)
        for i := range want {
            for j := range want[i] {
                if got[i][j]!= want[i][j] {
                    t.Errorf("For input=%+v\n\tWant=%+v\n\tBut got=%+v", testCase.input, want, got)
                }
            }
        }
    }
}
