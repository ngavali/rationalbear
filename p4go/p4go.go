package main

/*
#cgo CFLAGS: -I./cpp_files
#cgo LDFLAGS: -L./libs -lp4go
#include "p4api.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"os"
	"time"
	"unsafe"
)

func RunPerforceCommand(P4Client C.P4Client, argc C.int, msg *C.char, argv []*C.char) C.int {
	C.Initialize(P4Client)
	C.Connect(P4Client)
	C.Run(P4Client, C.int(argc), &msg, &argv[0])
	C.Message(P4Client)
	C.Clear(P4Client)
	fmt.Printf("DROPPED := %d\n", C.Dropped(P4Client))
	return C.Dropped(P4Client)
}

func main() {
	args := os.Args
	argc := len(args) - 1
	if argc > 0 {
		argv := make([]*C.char, argc)

		for i := 0; i < argc; i++ {
			cs := C.CString(args[i+1])
			defer C.free(unsafe.Pointer(cs))
			argv[i] = cs
		}

		var msg *C.char
		defer C.free(unsafe.Pointer(msg))

		var P4Client C.P4Client
		P4Client = C.NewP4Client()
		dropped := RunPerforceCommand(P4Client, C.int(argc), msg, argv)

		time.Sleep(5 * time.Second)
		if dropped == 1 {
			fmt.Println("==New connection")
			C.Final(P4Client)
			C.Disconnect(P4Client)
			P4Client = C.NewP4Client()
		}
		dropped = RunPerforceCommand(P4Client, C.int(argc), msg, argv)

		time.Sleep(5 * time.Second)
		if dropped == 1 {
			fmt.Println("==New connection")
			C.Final(P4Client)
			C.Disconnect(P4Client)
			P4Client = C.NewP4Client()
		}

		RunPerforceCommand(P4Client, C.int(argc), msg, argv)

	}
}
