package main


func maxProfit(prices []int) int {

	lowest := 10001
	maxProfit := 0

	for i := 0; i < len(prices); i++ {
		if prices[i] < lowest {
			lowest = prices[i]
		}
		if maxProfit < prices[i]-lowest {
			maxProfit = prices[i] - lowest
		}
	}

	return maxProfit
}

func YmaxProfit(prices []int) int {

	lowest := 10001
	highest := 0
	maxProfit := 0

	for i := 0; i < len(prices); i++ {
		if prices[i] < lowest {
			lowest = prices[i]
			highest = prices[i]
		}
		if highest < prices[i] {
			highest = prices[i]
			if maxProfit < highest-lowest {
				maxProfit = highest - lowest
			}
		}
	}

	return maxProfit
}

func XmaxProfit(prices []int) int {

	maxProfit := 0
	profit := 0

	for i := 0; i < len(prices)-1; i++ {
		for j := i; j < len(prices); j++ {
			profit = prices[j] - prices[i]
			if maxProfit < profit {
				maxProfit = profit
			}
		}

	}

	return maxProfit
}
