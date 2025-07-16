package main

import "testing"

func TestMirrorReflection(t *testing.T) {

	inputs := [][]int{{2, 1}, {3, 1}, {3, 2}, {4, 3}, {4, 4}, {6, 2}, {6, 4}}
	output := []int{2, 1, 0, 2, 1, 1, 0}

	for i, input := range inputs {
		want := output[i]
		got := mirrorReflection(input[0], input[1])
		if got != want {
			t.Errorf("For input: %v, want=%d but got=%d", input, want, got)
		}
	}

}

func TestGcd(t *testing.T) {

	inputs := [][]int{{4, 3}}
	output := []int{12}

	for i, input := range inputs {
		want := output[i]
		got := lcm(input[0], input[1])
		if got != want {
			t.Errorf("For input: %v, want=%d but got=%d", input, want, got)
		}

	}
}
