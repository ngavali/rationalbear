package main

import "fmt"

/*
//Best time

func permute(nums []int) [][]int {
    if len(nums) == 0 {
        return [][]int{}
    }

    used,p,res := make([]bool,len(nums)),[]int{},[][]int{}

    generatePermutation(nums,0,p,&res,&used)
    return res
}

func generatePermutation(nums []int, index int, p []int, res *[][]int, used *[]bool) {
    if index == len(nums) {
        temp := make([]int,len(p))
        copy(temp,p)
        *res = append(*res,temp)
        return
    }

    for i:=0; i < len(nums); i++ {
        if !(*used)[i] {
            (*used)[i] = true
            p = append(p,nums[i])
            generatePermutation(nums,index+1,p,res,used)
            p = p[:len(p)-1]
            (*used)[i] = false
        }
    }

    return
}

//Best memory
func permute(nums []int) [][]int {

    // return ALL permutations
    // nothing else than  brute force
    // integers are distinct - so have to cleanly go through them
    //  without getting duplicate combinations

    // can I establish some usable recursion
    //  choose from N, then choose from N-1, then from N-2

    // permutaions of first two : two orders
    // permutaions with one more : insert the one more at any locatoin from all
    //                             previous permutations

    if len(nums) == 1 {
        return [][]int{{nums[0]}}
    }

    previousPerms:= permute(nums[1:])

    newPerms := make([][]int,0,len(previousPerms)*len(nums))
    newElem := nums[0]

    for _,basePerm := range previousPerms {

        //fmt.Println("extend perm",basePerm,"with elem", newElem)
        for i:=0 ; i< len(nums); i++ {

            var k int
            newPerm := make([]int,len(nums))

            if i > 0 {
                for k = 0; k< i ; k++ {
                    newPerm[k] = basePerm[k]
                }
            }
            newPerm[k] = newElem
            //fmt.Println("inserted new element as index",i,"value",newElem)
            if k+1 < len(newPerm) {
                for k=k+1; k<len(newPerm); k++ {
                    newPerm[k] = basePerm[k-1]
                }
            }

            //fmt.Println("  built new",newPerm)
            newPerms = append(newPerms,newPerm)
            // build more permutions, insert the new element at all positions

        }
    }

    //fmt.Println("BUILT ALL THESE",newPerms)
    return newPerms
}
*/

//100% time
func permute(nums []int) [][]int {

    if len(nums) == 1 {
        return [][]int{nums}
    }

    res := [][]int{}
    if len(nums) == 2 {
        
        res = append(append(res, nums), []int{nums[1], nums[0]})

    } else {

        for i := 0; i < len(nums); i++ {

            subset := make([]int, len(nums))
            copy(subset, nums)
            key := subset[i]
            for _, val := range permute(append(subset[:i], subset[i+1:]...)) {
                res = append(res, append(val, key))
            }

        }

    }

    return res
}

func main() {

    nums := []int{1, 2, 3}
    //nums := []int{1, 2, 3, 4}
    //nums := []int{0}
    //nums := []int{0,1}
    fmt.Println(permute(nums))
}
