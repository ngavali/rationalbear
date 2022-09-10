package main

import (
	"testing"
)

func TestOrangesRotting(t *testing.T) {

    grids := []Grid{
        {{2},{2},{1},{0},{1},{1}},
        {
            {1},
            {1},
            {1},
            {1},
        },
        {
            {1, 2},
        },
        {
            {2, 1, 1},
            {1, 1, 0},
            {0, 1, 1},
        },
        {
            {2, 1, 1},
            {0, 1, 1},
            {1, 0, 1},
        },
        {
            {0, 2},
        },
    }
    minutes := []int{
        -1,
        -1,
        1,
        4,
        -1,
        0,
    }
    for i := range grids {
        want := minutes[i]
        got := orangesRotting(grids[i])
        if got != want {
            t.Errorf("For input[%d] want=%d but got=%d", i, want, got)
        }
    }
}
