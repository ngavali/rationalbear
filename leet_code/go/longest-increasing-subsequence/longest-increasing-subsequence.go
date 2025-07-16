package main

func Max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

var DPStore []int

func findSubSequence(nums []int, index int) int {

	if index > len(nums)-1 {
		return 0
	}

	maxSubSequence := 0

	if DPStore[index] != 0 {
		return DPStore[index]
	}

	for j := index + 1; j < len(nums); j++ {

		if nums[index] < nums[j] {
			if DPStore[j] != 0 {
				if maxSubSequence < DPStore[j] {
					maxSubSequence = DPStore[j]
				}
			} else {
				A := findSubSequence(nums, j)
				if maxSubSequence < A {
					maxSubSequence = A
				}
			}
		}

	}

	DPStore[index] = maxSubSequence + 1
	return DPStore[index]
}

func _lengthOfLIS(nums []int) int {

	DPStore = make([]int, len(nums))
	maxSubSequence := 0

	for i := range nums {
		maxSubSequence = Max(maxSubSequence, findSubSequence(nums, i))
	}

	return maxSubSequence
}

func LocateNumInArray(num int, arr []int) int {
	position := 0
	if len(arr) == 0 || num < arr[0] {
		return 0
	}
	if num > arr[len(arr)-1] {
		return len(arr)
	}

	if len(arr) != 0 {

		lw := 0
		hh := len(arr)

		for lw < hh {
			mid := lw + (hh-lw)/2

			if num < arr[mid] {
				hh = mid
				position = mid
			} else if num > arr[mid] {
				lw = mid + 1
				position = lw
			} else {
				position = mid
				break
			}

		}

	}

	return position
}

func lengthOfLIS(nums []int) int {

	A := make([]int, 0, len(nums))
	position := 0

	for _, v := range nums {
		position = LocateNumInArray(v, A)
		if position+1 > len(A) {
			A = append(A, v)
		} else {
			A[position] = v
		}
	}

	return len(A)

}
