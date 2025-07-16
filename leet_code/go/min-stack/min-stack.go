package main

import "fmt"

type Data struct {
    val    int
    minIdx int
}

type MinStack struct {
    pointer int
    data []Data
}

func Constructor() MinStack {
    return MinStack{0, make([]Data,32768)}
}

func (this *MinStack) Push(val int) {
    data := Data{val, 0}
        if this.pointer != 0 {
            if val < this.data[this.data[this.pointer-1].minIdx].val {
                data.minIdx = this.pointer
            } else {
                data.minIdx = this.data[this.pointer-1].minIdx
            }
        }
    this.data[this.pointer] = data
    this.pointer++
}

func (this *MinStack) Pop() {
    this.pointer--
}

func (this *MinStack) Top() int {
    return this.data[this.pointer-1].val
}

func (this *MinStack) GetMin() int {
    return this.data[this.data[this.pointer-1].minIdx].val
}

/*
type Data struct {
    val    int
    minIdx int
}

type MinStack struct {
    data []Data
}

func Constructor() MinStack {
    return MinStack{make([]Data,0,32768)}
}

func (this *MinStack) Push(val int) {
        if len(this.data) == 0 {
            data := Data{val, 0}
            this.data = append(this.data, data)
        } else {
        lowIdx := this.data[len(this.data)-1].minIdx
            data := Data{val, lowIdx}
        if val < this.data[lowIdx].val {
            data.minIdx = len(this.data)
        }
        this.data = append(this.data, data)
    }
}

func (this *MinStack) Pop() {
    this.data = this.data[:len(this.data)-1]
}

func (this *MinStack) Top() int {
    return this.data[len(this.data)-1].val
}

func (this *MinStack) GetMin() int {
    return this.data[this.data[len(this.data)-1].minIdx].val
}
*/

func main() {

    obj := Constructor()
    obj.Push(10)
    obj.Push(1)
    obj.Push(5)
    obj.Push(-1)
    obj.Pop()
    fmt.Println(obj.Top())
    fmt.Println(obj.GetMin())
}

/**
 * Your MinStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(val);
 * obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.GetMin();
 */
