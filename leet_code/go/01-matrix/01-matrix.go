package main

import "fmt"

type position struct {
	x, y int
}

type Queue []position

func minOf(x, y int) int {
    if x < y {
        return x
    }
    return y
}

var visitedCell = [][]bool{}

func findDistance(mat [][]int, x, y int) int {

    if x > len(mat)-1 || x < 0 || y < 0 || y > len(mat[0])-1 {
        return 10000
    }

    if mat[x][y] == 0 {
        return 0
    }

    if !visitedCell[x][y] {

        mat[x][y] = minOf(findDistance(mat,x+1,y), findDistance(mat, x,y+1))+1

        visitedCell[x][y] = true
        
        mat[x][y] = minOf(mat[x][y], minOf(findDistance(mat,x-1,y),findDistance(mat,x,y-1))+1)

    }

    return mat[x][y]
}

func updateMatrix(mat [][]int) [][]int {

    visitedCell = make([][]bool, len(mat))
    for i := range mat {
        visitedCell[i] = make([]bool, len(mat[i]))
    }
    for i := range mat {
        for j := range mat[i] {
            if mat[i][j] != 0 {
                findDistance(mat, i, j)
            }
        }
    }

    return mat
}

func main() {
	fmt.Println(updateMatrix([][]int{
		{0},
		{1},
	}))
	fmt.Println(updateMatrix([][]int{
		{0, 0, 0},
		{0, 1, 0},
		{0, 0, 0},
	}))
	fmt.Println(updateMatrix([][]int{
		{0, 0, 0},
		{0, 1, 0},
		{1, 1, 1},
	}))
	fmt.Println(updateMatrix([][]int{
		{1, 0, 1, 1, 0, 0, 1, 0, 0, 1},
		{0, 1, 1, 0, 1, 0, 1, 0, 1, 1},
		{0, 0, 1, 0, 1, 0, 0, 1, 0, 0},
		{1, 0, 1, 0, 1, 1, 1, 1, 1, 1},
		{0, 1, 0, 1, 1, 0, 0, 0, 0, 1},
		{0, 0, 1, 0, 1, 1, 1, 0, 1, 0},
		{0, 1, 0, 1, 0, 1, 0, 0, 1, 1},
		{1, 0, 0, 0, 1, 1, 1, 1, 0, 1},
		{1, 1, 1, 1, 1, 1, 1, 0, 1, 0},
		{1, 1, 1, 1, 0, 1, 0, 0, 1, 1},
	}))
}
