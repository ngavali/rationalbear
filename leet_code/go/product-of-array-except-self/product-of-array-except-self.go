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
    i := 0
    N := len(nums)
    for i<N {
        if nums[i] != 0 {
            product *= nums[i]
        } else {
            countZeroes++
            if countZeroes > 1 {
                return make([]int, N)
            }
        }
        if countZeroes == 1 {
            nums[i] = 0
        }
        i++
    }
    i = 0
    if countZeroes > 0 {
        for i<N {
            if nums[i] == 0 {
                nums[i] = product
                break
            } else {
                nums[i] = 0
            }
            i++
        }
    } else {
        i = 0
        for i<N {
            nums[i] = product/nums[i]
            i++
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
