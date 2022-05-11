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
	"unsafe"
)

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

		P4Client := C.NewP4Client()
		C.Initialize(P4Client)
		C.Connect(P4Client)
		C.Run(P4Client, C.int(argc), &msg, &argv[0])
		C.Message(P4Client)
		C.Clear(P4Client)

		fmt.Printf("Dropped: %d\n", C.Dropped(P4Client))
		msg = C.CString("")
		C.Run(P4Client, C.int(argc), &msg, &argv[0])
		C.Message(P4Client)
		C.Clear(P4Client)

		fmt.Printf("Dropped: %d\n", C.Dropped(P4Client))
		msg = C.CString("")
		C.Run(P4Client, C.int(argc), &msg, &argv[0])
		C.Message(P4Client)
		C.Clear(P4Client)

		C.Final(P4Client)
		C.Disconnect(P4Client)
	}
}
