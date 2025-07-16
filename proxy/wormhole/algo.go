package wormhole

import (
	"github.com/pkg/errors"
)

const (
	CLOSEST int = iota
	ROUNDROBIN
)

var algoName = map[int]string{CLOSEST: "Closest", ROUNDROBIN: "Round Robin"}

type SelectionAlgo interface {
	Select([]destination, chan destination)
	Type() int
}

type algoClosest struct {
}

func (a *algoClosest) Type() int {
	return CLOSEST
}

func (a *algoClosest) Select(beList []destination, be chan destination) {
	for {
		for _, b := range beList {
			if b.isAlive {
				be <- b
				break
			}
		}
	}
}

type algoRoundRobin struct {
}

func (a *algoRoundRobin) Type() int {
	return ROUNDROBIN
}

func (a *algoRoundRobin) Select(beList []destination, be chan destination) {
	for {
		for _, b := range beList {
			if b.isAlive {
				be <- b
			}
		}
	}
}

func NewSelectionAlgo(algoType int) SelectionAlgo {
	switch algoType {
	case CLOSEST:
		return new(algoClosest)
	case ROUNDROBIN:
		return new(algoRoundRobin)
	default:
		panic(errors.Errorf("Unsupported algorithm: %d", algoType))
	}
}
