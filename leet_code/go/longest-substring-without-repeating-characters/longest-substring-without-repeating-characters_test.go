package main

import (
	"testing"
)

type testcase struct {
    i string
    o int
}

func TestLengthOfLongestSubstring(t *testing.T) {

    testcases := []testcase{
        {"au",2},
        {" ",1},
        {"abcabcbb",3},
        {"bbbbb",1},
        {"pwwkew",3},
        {"abba",2},
        {"",0},
        {"tmmzuxt",5},
    }

    for _, testcase := range testcases {
        want := testcase.o
        got := lengthOfLongestSubstring(testcase.i)
        if got!= want {
            t.Errorf("For input=%s, want=%d but got=%d", testcase.i, want, got)
        }
    }

}
