package main

import "fmt"

func lcm(x, y int) int {

	lcm := x
	for lcm%y != 0 {
		fmt.Println(lcm)
		lcm += x
	}

	return lcm
}
func _mirrorReflection(p int, q int) int {
	for p%2 == 0 && q%2 == 0 { // if it's a bigger square, we can shrink it into a small square like in the example
		p /= 2
		q /= 2
	}

	fmt.Println(1-p%2+q%2, -p%2, q%2)
	return 1 - p%2 + q%2
}

func x_mirrorReflection(p int, q int) int {

	if p == q {
		return 1
	}

	lcm_pq := lcm(p, q)

	lcm_p := lcm_pq / p
	lcm_q := lcm_pq / q

	if lcm_q%2 == 0 {
		return 2
	}

	return lcm_p % 2
}

func mirrorReflection(p int, q int) int {

	/*
		lcm_pq := p
		for lcm_pq%q != 0 {
			lcm_pq += p
		}
	*/
	for p%2 == 0 && q%2 == 0 {
		p /= 2
		q /= 2
	}
	return 1 - p%2 + q%2

}
