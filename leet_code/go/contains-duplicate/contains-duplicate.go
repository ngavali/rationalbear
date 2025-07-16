package main

var empty struct{}
func containsDuplicate(nums []int) bool {

    numMap := make(map[int]struct{}, len(nums))

    for _, n := range nums {
        if _, ok := numMap[n]; ok {
            return true
        }
        numMap[n] = empty
    }

    return false

}
