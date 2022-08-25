package main

import "fmt"

var myColor,m,n,new_color int
var myImage [][]int

func spread(sr, sc int) {

    if myImage[sr][sc] == myColor {
        //myColor := image[sr][sc]
        myImage[sr][sc] = new_color
        //Top
        if sr-1 >= 0 {
            spread(sr-1, sc)
        }
        //Bottom
        if sr+1 < m {
            spread(sr+1, sc)
        }
        //Left
        if sc-1 >= 0 {
            spread(sr, sc-1)
        }
        //Right
        if sc+1 < n {
            spread(sr, sc+1)
        }
    }

}

func floodFill(image [][]int, sr int, sc int, color int) [][]int {

    myColor = image[sr][sc]
    if myColor == color {
        return image
    }
    m = len(image)
    n = len(image[0])

    myImage = image
    new_color = color

    spread(sr, sc)

    return myImage
}

func Xspread(image [][]int, m, n, sr, sc, color int) [][]int {

    if image[sr][sc] != color {
        myColor := image[sr][sc]
        image[sr][sc] = color
        //Top
        if sr-1 >= 0 && myColor == image[sr-1][sc] {
            Xspread(image, m, n, sr-1, sc, color)
        }
        //Bottom
        if sr+1 < m && myColor == image[sr+1][sc] {
            Xspread(image, m, n, sr+1, sc, color)
        }
        //Left
        if sc-1 >= 0 && myColor == image[sr][sc-1] {
            Xspread(image, m, n, sr, sc-1, color)
        }
        //Right
        if sc+1 < n && myColor == image[sr][sc+1] {
            Xspread(image, m, n, sr, sc+1, color)
        }
    }

    return image
}

func XfloodFill(image [][]int, sr int, sc int, color int) [][]int {

    m := len(image)
    n := len(image[0])

    return Xspread(image, m, n, sr, sc, color)
}

func main() {
    image := [][]int{{1, 1, 1}, {1, 1, 0}, {1, 0, 1}}
    fmt.Println(floodFill(image, 1, 1, 2))
    fmt.Println(floodFill([][]int{{1}}, 0, 0, 2))
    fmt.Println(floodFill([][]int{{0, 0, 0}, {0, 0, 0}}, 0, 0, 0))
}
