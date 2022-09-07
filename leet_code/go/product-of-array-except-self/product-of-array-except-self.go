package main

import "fmt"

/*
func productExceptSelf(nums []int) []int {

    products := make([]int,len(nums))

    product := 1
    for i, num := range nums {
        products[i] = product
        product = product * num
    }

    product = 1
    for i:= len(nums)-1; i>=0 ; i-- {
        products[i] = products[i]*product
        product = product*nums[i]
    }
    return products
}
*/

func productExceptSelf(nums []int) []int {
    product := 1
    countZeroes := 0
    for _, n := range nums {
        if n != 0 {
            product *= n
        } else {
            countZeroes++
        }
        if countZeroes > 1 {
            return make([]int, len(nums))
        }
    }
    if countZeroes > 0 {
        for i, n := range nums {
            if n == 0 {
                nums[i] = product
            } else {
                nums[i] = 0
            }
        }
        return nums
    } else {
        for i, n := range nums {
            if n != 0 {
                nums[i] = product/n
            }
        }
    }
    return nums
}

func main() {
    input := []int{1,2,3,4}
    output := productExceptSelf(input)
    fmt.Println(output)
    input = []int{-1,1,0,-3,3}
    output = productExceptSelf(input)
    fmt.Println(output)
}
