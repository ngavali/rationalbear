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

    if x >= len(mat) || y >= len(mat[0]) {
        return len(mat) + len(mat[0]) + 2
    }

    if mat[x][y] == 0 {
        return 0
    }

    if !visitedCell[x][y]  {

        mat[x][y] = minOf(findDistance(mat,x+1,y), findDistance(mat, x,y+1))+1

        visitedCell[x][y]=true
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
            findDistance(mat, i, j)
            if i-1 >= 0 {
                mat[i][j] = minOf(mat[i][j], mat[i-1][j]+1)
            }
            if j-1 >= 0 {
                mat[i][j] = minOf(mat[i][j], mat[i][j-1]+1)
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
