package main

import (
	"fmt"
	"strconv"
)

var stack []int

func evalRPN(tokens []string) (res int) {

    stack = make([]int, 0, len(tokens))
    for _, token := range tokens {
        if token == "+" {
            res = stack[len(stack)-2] + stack[len(stack)-1]
            stack = stack[:len(stack)-2]
        } else if token == "-" {
            res = stack[len(stack)-2] - stack[len(stack)-1]
            stack = stack[:len(stack)-2]
        } else if token == "*" {
            res = stack[len(stack)-2] * stack[len(stack)-1]
            stack = stack[:len(stack)-2]
        } else if token == "/" {
            res = stack[len(stack)-2] / stack[len(stack)-1]
            stack = stack[:len(stack)-2]
        } else {
            res, _ = strconv.Atoi(token)
        }
        stack = append(stack, res)
    }
    return res
}

func main() {
    fmt.Println(evalRPN([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}))
}
