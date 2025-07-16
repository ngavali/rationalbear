package main

import (
	"testing"
)

type Grid [][]byte

func TestNumIslands(t *testing.T) {

    grids := []Grid{
        {
            {'1', '1', '1', '1', '0'},
            {'1', '1', '0', '1', '0'},
            {'1', '1', '0', '0', '0'},
            {'0', '0', '0', '0', '0'},
        },
        {
            {'1', '1', '0', '0', '0'},
            {'1', '1', '0', '0', '0'},
            {'0', '0', '1', '0', '0'},
            {'0', '0', '0', '1', '1'},
        },
    }

    numIsland := []int{
        1,
        3,
    }

    for i, grid := range grids {
        want := numIsland[i]
        got := numIslands(grid)
        if got != want {
            t.Errorf("For input[%d], want=%d but got=%d", i, want, got)
        }
    }
}
