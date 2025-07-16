package main

import (
	"testing"
)

type testcase struct{
    coins []int
    amount int
    expected int
}
func TestCoinsCounter(t *testing.T)  {
    testcases := []testcase{
        {[]int{1}, 1000,1000},
        {[]int{25, 10, 5}, 30, 2},
        {[]int{186,419,83,408}, 6249, 20},
        {[]int{2, 5, 10, 1}, 27, 4},
        {[]int{1, 2, 5}, 11, 3},
        {[]int{2}, 0, 0},
        {[]int{2}, 3, -1},
        {[]int{2, 5, 3, 6}, 10, 2},
        {[]int{411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422}, 9864, 24},
    }

    for i, testcase := range testcases {
        got := coinChange(testcase.coins, testcase.amount)
        if got != testcase.expected  {
            t.Errorf("Input=%d, want=%d but got=%d", i, testcase.expected, got)
        }
    }
}
