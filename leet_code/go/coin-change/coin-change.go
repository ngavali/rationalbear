package main

import "sort"

/*
Notes:- this ain't best solution, work on it
*/

func minOf(x, y int) int {
    if x < y {
        return x
    }
    return y
}

var coinCountMap map[int]map[int]int

func coinsCounter(coins []int, n, sum int) (count int) {

    if sum == 0 {
        return 0
    }
    if n <= 0 || sum < 0 {
        return 10001
    }
    c := coins[n-1]
    if v, ok := coinCountMap[c][sum]; ok {
        return v
    }
    coinCountMap[c][sum] =
        minOf(
            coinsCounter(coins, n, sum-c)+1,
            coinsCounter(coins, n-1, sum),
        )

    return coinCountMap[c][sum]

}

func coinChange(coins []int, amount int) (numCoins int) {

    coinCountMap = make(map[int]map[int]int, len(coins))
    for i := range coins {
        coinCountMap[coins[i]] = make(map[int]int, amount)
    }
    sort.Slice(coins, func(i,j int) bool{
        return coins[i] < coins[j]
    })
    numCoins = coinsCounter(coins, len(coins), amount)
    if numCoins == 10001 {
        return -1
    }
    return

}
