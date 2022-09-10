package main

/* Some other best solutions

func orangesRotting(grid [][]int) int {
    m, n := len(grid), len(grid[0])
    updated := true
    t := 0
    for updated {
        updated = false
        old := grid
        grid = make([][]int, m)
        for i := 0; i < m; i++ {
            grid[i] = make([]int, n)
            for j := 0; j < n; j++ {
                grid[i][j] = old[i][j]
                if old[i][j] != 1 { continue }
                if i > 0 && old[i-1][j] == 2 { updated = true; grid[i][j] = 2; continue }
                if j > 0 && old[i][j-1] == 2 { updated = true; grid[i][j] = 2; continue }
                if i < m-1 && old[i+1][j] == 2 { updated = true; grid[i][j] = 2; continue }
                if j < n-1 && old[i][j+1] == 2 { updated = true; grid[i][j] = 2; continue }
            }
        }
        if updated { t++ }
    }
    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            if grid[i][j] == 1 { return -1 }
        }
    }
    return t
}

*/
type Grid [][]int
type Pos [2]int
var M, N int

func orangesRotting(grid [][]int) (minute int) {

    M = len(grid)
    N = len(grid[0])
    queue := make([]Pos, 0, M*N)

    visited := make([][]int, M)
    for i := range visited {
        visited[i] = make([]int, N)
    }

    allZeroes, noRotten := true, true

    for i := range grid {
        for j := range grid[i] {
            if grid[i][j] == 2 {
                noRotten = false
                queue = append(queue, Pos{i, j})
            }
            if grid[i][j] == 1 {
                allZeroes = false
            }
        }
    }

    if len(queue) == 0 {
        if allZeroes {
            return 0
        }
        if noRotten {
            return -1
        }
    }

    for len(queue) > 0 {
        location := queue[0]

        x := location[0]
        y := location[1]

        if visited[x][y] == 0 {
            visited[x][y] = 1
            minute = grid[x][y]

            if x+1 < M && grid[x+1][y] == 1 {
                grid[x+1][y] += minute
                queue = append(queue, Pos{x + 1, y})
            }
            if x-1 >= 0 && grid[x-1][y] == 1 {
                grid[x-1][y] += minute
                queue = append(queue, Pos{x - 1, y})
            }
            if y+1 < N && grid[x][y+1] == 1 {
                grid[x][y+1] += minute
                queue = append(queue, Pos{x, y + 1})
            }
            if y-1 >= 0 && grid[x][y-1] == 1 {
                grid[x][y-1] += minute
                queue = append(queue, Pos{x, y - 1})
            }

        }

        queue = queue[1:]
    }

    for i := range grid {
        for j := range grid[i] {
            if grid[i][j] == 1 {
                return -1
            }
        }
    }
    return minute - 2
}
