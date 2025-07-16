package main

/*
func sortColors(nums []int) {
    zero, one := 0, 0
    for i, n := range nums {
        fmt.Println("B",zero, one, nums)
        nums[i] = 2
        if n <= 1 {
            nums[one] = 1
            one++
        }
        if n == 0 {
            nums[zero] = 0
            zero++
        }
        fmt.Println("A",zero, one, nums)
    }
}
*/

//Single Pass
func sortColors(nums []int) {

    i, indx0, indx2 := 0, 0, len(nums)-1

    for i <= indx2 {
        switch nums[i] {
            case 2:    
            nums[indx2], nums[i] = nums[i], nums[indx2]
            indx2--
        default:
            if nums[i] == 0 {
                nums[indx0], nums[i] = nums[i], nums[indx0]
                indx0++
            }
            i++
        }
    }
}

/*
//Using sort
func sortColors(nums []int) {

    if len(nums) > 1 {
        low, high := 0, len(nums)-1
        for j := low; j < high; j++ {
            if nums[j] < nums[high] {
                if j != low {
                    nums[j], nums[low] = nums[low], nums[j]
                }
                low++
            }
        }

        nums[low], nums[high] = nums[high], nums[low]

        sortColors(nums[:low])
        sortColors(nums[low+1:])
    }

}
*/
