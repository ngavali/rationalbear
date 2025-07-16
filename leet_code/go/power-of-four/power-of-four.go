package main

import "fmt"

func isPowerOfFour(n int) bool {

	if n > 0 {
		fmt.Println("Num ->", n)
		for n%4 == 0 {
			n >>= 2
		}

		if n == 1 {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(isPowerOfFour(16))
	fmt.Println(isPowerOfFour(6))
	fmt.Println(isPowerOfFour(32))
}
