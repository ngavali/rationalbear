package main

const (
	DFS = iota
	BFS
)

var method = DFS

func numIslands(grid [][]byte) int {
	M = len(grid)
	N = len(grid[0])

	if method == DFS {
		return numIslandsDFS(grid)
	} else {
		return numIslandsBFS(grid)
	}
}

var M, N int

func numIslandsDFS(grid [][]byte) int {
	numIsland := 0

	for i := 0; i < M; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '1' {
				visitIsland(grid, i, j)
				numIsland++
			}
		}
	}

	return numIsland
}

func visitIsland(grid [][]byte, x, y int) {
	if grid[x][y] == '1' {
		grid[x][y]++
		if x+1 < M {
			visitIsland(grid, x+1, y)
		}
		if x-1 >= 0 {
			visitIsland(grid, x-1, y)
		}
		if y+1 < N {
			visitIsland(grid, x, y+1)
		}
		if y-1 >= 0 {
			visitIsland(grid, x, y-1)
		}
	}
}

/* BFS */
type Location struct {
	x, y int
}

func numIslandsBFS(grid [][]byte) int {

	numIsland := 0
	queue := make([]Location, 0, M*N)

	for i := 0; i < M; i++ {
		for j := 0; j < N; j++ {

			if grid[i][j] == '1' {
				queue = append(queue, Location{i, j})

				for len(queue) > 0 {

					location := queue[0]

					if grid[location.x][location.y] == '1' {
						grid[location.x][location.y]++
						if location.x+1 < M {
							queue = append(queue, Location{location.x + 1, location.y})
						}
						if location.x-1 >= 0 {
							queue = append(queue, Location{location.x - 1, location.y})
						}
						if location.y+1 < N {
							queue = append(queue, Location{location.x, location.y + 1})
						}
						if location.y-1 >= 0 {
							queue = append(queue, Location{location.x, location.y - 1})
						}
					}

					queue = queue[1:]
				}

				numIsland++

			}

		}
	}

	return numIsland
}
