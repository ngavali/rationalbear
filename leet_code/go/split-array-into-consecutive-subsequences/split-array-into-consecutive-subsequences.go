package main

func isPossible(nums []int) bool {

	var prevNum, nextNum, nextToNextNum int

	numsMap := make(map[int]int)

	sequenceMap := make(map[int]int)

	for _, val := range nums {
		numsMap[val]++
	}

	for _, curr_num := range nums {
		if numsMap[curr_num] == 0 {
			continue
		}

		numsMap[curr_num]--
		prevNum = curr_num - 1
		if sequenceMap[prevNum] > 0 {
			sequenceMap[prevNum]--
			sequenceMap[curr_num]++
		} else {
			nextNum = curr_num + 1
			nextToNextNum = nextNum + 1
			if numsMap[nextNum] > 0 && numsMap[nextToNextNum] > 0 {
				numsMap[nextNum]--
				numsMap[nextToNextNum]--
				sequenceMap[nextToNextNum]++
			} else {
				return false
			}
		}
	}
	return true
}
