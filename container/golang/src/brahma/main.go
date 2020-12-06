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

package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

func main() {
	switch os.Args[1] {
	case "run":
		run()
	case "loka":
		loka()
	default:
		fmt.Println("Usage: ./brahma run command")
	}
}

func run() {
	fmt.Printf("Creating Loka for %v\n", os.Args[2:])
	cmd := exec.Command("/proc/self/exe", append([]string{"loka"}, os.Args[2:]...)...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Cloneflags: syscall.CLONE_NEWNS |
			syscall.CLONE_NEWUTS |
			syscall.CLONE_NEWIPC |
			syscall.CLONE_NEWPID |
			syscall.CLONE_NEWNET |
			syscall.CLONE_NEWUSER,
		//Map UID
		UidMappings: []syscall.SysProcIDMap{
			{
				HostID:      os.Getuid(), //UID of current program to be mapped with
				ContainerID: 0,           //UID in container
				Size:        1,
			},
		},
		//Map GID
		GidMappings: []syscall.SysProcIDMap{
			{
				HostID:      os.Getgid(), //GID of current program to be mapped with
				ContainerID: 0,           //GID in container
				Size:        1,
			},
		},
	}
	_err := cmd.Start()
	if _err != nil {
		fmt.Println("Error while creating Loka ->", _err)
	} else {
		fmt.Println("Loka created successfully!!!")
	}
	if _err := cmd.Wait(); _err != nil {
		fmt.Println("Error while in Loka ->", _err)
	}
	fmt.Println("Loka distroyed!!!")
}

func loka() {
	fmt.Printf("Preparing %v in new Loka\n", os.Args[2:])
	syscall.Sethostname([]byte("my-loka"))
	syscall.Chroot("/containerfs")               //New root for the Loka container
	syscall.Chdir("/")                           //Change directory
	syscall.Mount("proc", "proc", "proc", 0, "") //Mount new proc
	//Replace the current process. Leaving no trace of the parent /proc/self/exe
	_err := syscall.Exec(os.Args[2], os.Args[3:], nil)
	if _err != nil {
		fmt.Println("Loka error ->", _err)
	}
	fmt.Println("I am dying!!!")
}
