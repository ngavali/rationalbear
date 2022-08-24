package main

import "fmt"

func myPow(x float64, n int) float64 {
	var res float64 = 1
	signed := false
	if n < 0 {
		n = -n
		signed = true
	}
	for i := 0; i < n; i++ {
		res *= x
	}
	if signed {
		return 1 / res
	}
	return res
}

func powM(x float64, n int) float64 {
	res := 1.0
	for i := 0; i < n; i++ {
		res *= x
	}
	return res
}

func PowX(x float64, n int) float64 {
	if x*x == 1.0 || x == 0.0 {
		if n%2 == 0 && x < 0 {
			return -x
		}
		return x
	}
	res := 1.0
	i := 0
	if n < 0 {
		for i = n / 10; i < 0; i++ {
			res /= (x * x * x * x * x * x * x * x * x * x)
			if res < 0.00001 {
				break
			}
		}
		if n%10 != 0 && res > 0.00001 {
			fmt.Println(-n%10, res)
			res /= powM(x, -n%10)
		}
	} else {
		for i = 0; i < n/10; i++ {
			res *= (x * x * x * x * x * x * x * x * x * x)
		}
		if n%10 != 0 {
			res *= powM(x, n%10)
		}
	}
	return res
}

func myPowX(x float64, n int) float64 {
	if x*x == 1.0 || x == 0.0 {
		if n%2 == 0 && x < 0 {
			return -x
		}
		return x
	}
	res := 1.0
	i := 0
	if n < 0 {
		for i = n / 2; i < 0; i++ {
			res /= (x * x)
			if res < 0.0001 {
				break
			}
		}
		if n%2 != 0 {
			res /= x
		}
	} else {
		for i = 0; i < n/2; i++ {
			res *= (x * x)
		}
		if n%2 != 0 {
			res *= x
		}
	}
	return res
}

func main() {
	//fmt.Println(myPowX(2.00000, -2147483648))
	//fmt.Println(myPowX(34.00515, -3))
	fmt.Println(PowX(2, -2))
}
