/* The MIT License

   Copyright (c) 2008, 2009 by gavali.nilesh80186 <gavali.nilesh80186@gmail.com>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

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
