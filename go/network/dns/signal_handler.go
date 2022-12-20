package main

import (
	"os"
	"os/signal"
)

func signal_handler() chan os.Signal {
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Kill, os.Interrupt)
	return c
}
