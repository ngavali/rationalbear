package main

import (
	"fmt"
	"log"
	"net"
	"strconv"
)

func main() {
	udpPorts := []string{"53", "54"}
	listenCamUDPError := true
	var selfUDPlistener *net.UDPConn
	testUDPPort := udpPorts[0]
	testUDPPort2 := udpPorts[1]
	selfIP := "0.0.0.0"

	for listenCamUDPError == true {
		fmt.Println("Trying to resolve this socket: ", selfIP, ":", testUDPPort)
		udpAddr, err := net.ResolveUDPAddr("udp4", selfIP+":"+testUDPPort)
		if err != nil {
			log.Fatal(err)
		}

		fmt.Println("Resolved: ", udpAddr)

		selfUDPlistener, err = net.ListenUDP("udp4", udpAddr)
		if err != nil {
			fmt.Println("Got an error: ", err, ", trying another port")
			intPort, _ := strconv.Atoi(testUDPPort)
			testUDPPort = strconv.Itoa(intPort + 2)
			testUDPPort2 = strconv.Itoa(intPort + 3)
			fmt.Println("Will try this pair of ports: ", testUDPPort, " ", testUDPPort2)
		} else {
			listenCamUDPError = false
		}
	}

    fmt.Print(selfUDPlistener)

}
