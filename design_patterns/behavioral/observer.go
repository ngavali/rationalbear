
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
