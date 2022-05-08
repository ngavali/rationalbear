package main

/*
#cgo CFLAGS: -I./cpp_files
#cgo LDFLAGS: -L./libs -lp4go
#include "c_for_go.h"
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

		res := C.RunCmd(C.int(argc), &argv[0])
		defer C.free(unsafe.Pointer(res))

		fmt.Printf("%s", C.GoString(res))
	}
}
