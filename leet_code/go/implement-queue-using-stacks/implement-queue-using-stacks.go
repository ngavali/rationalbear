package main

type MyQueue struct {
    data []int
}

func Constructor() MyQueue {
    data := []int{}
    return MyQueue{data}
}

func (this *MyQueue) Push(x int) {
    this.data = append(this.data, x)
}

func (this *MyQueue) Pop() int {
    popped := this.data[0]
    this.data = this.data[1:]
    return popped
}

func (this *MyQueue) Peek() int {
    return this.data[0]
}

func (this *MyQueue) Empty() bool {
    if len(this.data) == 0 {
        return true
    }
    return false
}
