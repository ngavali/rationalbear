package main

import "fmt"

type Node struct {
    Val       int
    Neighbors []*Node
}

var graphMap =  make(map[*Node]*Node, 100)

func cloneGraph(node *Node) *Node {

    queue:= []*Node{node}
    curr_node := node

    for len(queue) > 0 && curr_node!=nil {
        curr_node = queue[0]
        if _, ok := graphMap[curr_node]; !ok {
            graphMap[curr_node] = &Node{curr_node.Val, make([]*Node, len(curr_node.Neighbors))}
            queue = append(queue, curr_node.Neighbors...)
        }
        queue = queue[1:]
    }

    for old_node, new_node := range graphMap {
        for i, old_neighbor := range old_node.Neighbors {
            new_neighbor := graphMap[old_neighbor]
            new_node.Neighbors[i] = new_neighbor
        }
    }

    return graphMap[node]

}

func main() {
    node2:= &Node{2, nil}
    node4:= &Node{4, nil}
    node1:= &Node{1, []*Node{node2, node4}}
    node3:= &Node{3, []*Node{node2, node4}}
    node2.Neighbors = []*Node{node1, node3}
    node4.Neighbors = []*Node{node1, node3}

    cloneGraph(node1)
    for old_node, new_node := range graphMap {
        fmt.Println(old_node, new_node)
    }
}
