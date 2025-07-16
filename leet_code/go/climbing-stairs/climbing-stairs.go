package main

import "fmt"

var steps int

func climbStairs(n int) int {

	if n == 0 {
		steps++
	}
	if n > 0 {

		climbStairs(n - 1)
		climbStairs(n - 2)

	}

	return 0
}

var dp = make([]int,45)

func fibonacci(n int) int {
    num := n
	if dp[n] == 0 {
		i, j := 0, 1
		for n >= 0 {
			j, i = j+i, j
			n--
		}
		dp[num] = i
	}
	return dp[num]
}

func main() {

	//climbStairs(4)
	//fmt.Println(steps)
	fmt.Println(fibonacci(4))
}
