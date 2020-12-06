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
//      Observer pattern
//

package main

import "fmt"

type Subject interface {
	register(Observer)
	unregister(Observer)
	notify()
}

type subject struct {
	observers []Observer
	name      string
	value     int
}

func NewSubject(name string) *subject {
	return &subject{name: name}
}

func (s *subject) register(o Observer) {
	s.observers = append(s.observers, o)
}

func (s *subject) unregister(o Observer) {
	for k, v := range s.observers {
		if v == o {
			s.observers = append(s.observers[:k], s.observers[k+1:]...)
		}
	}
}

func (s *subject) update() {
	s.value++
	s.notify()
}

func (s *subject) notify() {
	for _, v := range s.observers {
		v.update(s.value)
	}
}

type Observer interface {
	update(int)
}

type observer struct {
	name  string
	score subject
}

func NewObserver(name string) *observer {
	return &observer{name: name}
}

func (o *observer) update(score int) {
	o.score.value = score
	o.display()
}

func (o *observer) display() {
	fmt.Printf("%s : current score is : %d\n", o.name, o.score.value)
}

func main() {

	var CricketScore = NewSubject("CricketScore")

	var s1, s2, s3 = NewObserver("s1"), NewObserver("s2"), NewObserver("s3")

	CricketScore.register(s1)
	CricketScore.register(s2)
	CricketScore.register(s3)

	CricketScore.update()
	CricketScore.update()

	CricketScore.unregister(s2)

	CricketScore.update()

}
