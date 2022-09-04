package main

import "fmt"

func doPartition(nums []int, low, high int ) int {

    for i:=low; i<high; i++ {
        if nums[i] < nums[high] {
            if i != low {
                fmt.Println("Swapped:", nums[i], nums[low])
                nums[i], nums[low] = nums[low], nums[i]
            }
            low++
        }
    }

    nums[low], nums[high] = nums[high], nums[low]

return low

}

func quicksort(nums []int, low, high, pos int) []int {

    if low < high {
        partition := doPartition(nums, low, high) 

            quicksort(nums, low, partition-1, pos)
        if partition < pos-1 {
            quicksort(nums, partition+1, high, pos)
        }
    }
    return nil
}

func main() {
    //nums := []int{-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0}
    nums := []int{11,21,32,31,55,10,20,30,40,50,1,8,9,2,9,16,3,11,10,4,15,5,12}
    fmt.Println(nums)
    quicksort(nums, 0, len(nums)-1, 5)
    fmt.Println(nums)
}
