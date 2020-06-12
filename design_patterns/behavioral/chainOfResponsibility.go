//
// chainOfResponsibility.go
//

package main

import "fmt"

type Processor interface {
	Next() Processor
	SetNext(Processor)
	Do()
}

type processor struct {
	next Processor
}

func (p *processor) Next() Processor {
	return p.next
}

func (p *processor) SetNext(next Processor) {
	p.next = next
}

func (p *processor) Do() {
	if p.next != nil {
		p.next.Do()
	}
}

type extract struct {
	processor
}

func (e *extract) Do() {
	fmt.Println("Processing extract")
	e.processor.Do()
}

type transform struct {
	processor
}

func (t *transform) Do() {
	fmt.Println("Processing transform")
	t.processor.Do()
}

type load struct {
	processor
}

func (l *load) Do() {
	fmt.Println("Processing load")
	l.processor.Do()
}

type Map struct {
}

type Reduce struct {
}

func main() {

	var proc1, proc2, proc3 Processor
	proc1 = &extract{}
	proc2 = &transform{}
	proc3 = &load{}
	proc1.SetNext(proc2)
	proc2.SetNext(proc3)
	proc1.Do()

}
