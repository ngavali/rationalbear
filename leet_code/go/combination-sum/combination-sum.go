package main

import "fmt"

/*
//Best memory

func combinationSum(candidates []int, target int) [][]int {
    var (
        res [][]int
        path []int
        dfs func(i, sum int)
    )
    dfs = func(i, sum int) {
        if sum == target {
            res = append(res, append([]int{}, path...))
            return
        }
        if sum > target || i >= len(candidates) {
            return
        }

        for j := 0; sum + j * candidates[i] <= target; j++ {
            for k := 0; k < j; k++ {
                path = append(path, candidates[i])
            }
            dfs(i + 1, sum + j * candidates[i])
            path = path[:len(path) - j]
        }
    }
    dfs(0, 0)
    return res
}

*/

var result [][]int

func findCombination(candidates, res []int, target, pos int) {

    if target == 0 {
        result = append(result, append([]int{}, res...))
        return
    }

    if target < 0 {
        return
    }

    for i := pos; i < len(candidates); i++ {
        if target >=  candidates[i] {
            //res_copy := make([]int, len(res))
            //copy(res_copy, res)
            findCombination(candidates, append(res, candidates[i]), target-candidates[i], i)
        }
    }

}

func findCombinationX(candidates, res []int, target, pos int) {

    if pos < len(candidates) {
        if target == 0 {
            result = append(result, res)
            return
        }

        if target < 0 {
            return
        }

        res_copy := make([]int, len(res))
        copy(res_copy, res)
        findCombinationX(candidates, append(res, candidates[pos]), target-candidates[pos], pos)
        findCombinationX(candidates, res_copy, target, pos+1)
    }

}

func combinationSum(candidates []int, target int) [][]int {

    result = [][]int{}
    //findCombinationX(candidates, []int{}, target, 0)
    findCombination(candidates, []int{}, target, 0)
    return result

}

func main() {
    candidatesCollection := [][]int{
        {2,7,6,3,5,1},
        {2, 3, 6, 7},
        {2, 3, 5},
    }

    target := []int{
        9,
        7,
        8,
    }
    for i, candidates := range candidatesCollection {
        result := combinationSum(candidates, target[i])
        fmt.Println(result)
    }
}
