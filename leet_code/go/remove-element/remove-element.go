package main

func removeElement(nums []int, val int) (count int) {
	idx := 0
	for i := range nums {
		if nums[i] != val {
			nums[idx] = nums[i]
			idx++
			count++
		}
	}
	return
}
