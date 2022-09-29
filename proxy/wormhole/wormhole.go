
package wormhole

import (
	"fmt"
	"io"
	"net"
	"sort"
	"sync"
	"time"

	"github.com/pkg/errors"
)

var wormholes map[string]wormhole

type wormhole struct {
	name            string
	port            uint
	beChan          chan destination
	algo            SelectionAlgo
	destinationList []destination
	checkCycle      int
}

func (t *wormhole) Info() {
	fmt.Printf("Listening on port: %d\nAlgo: %s\nNum of backends: %2d\nCheck interval: %3d seconds\n", t.port, algoName[t.algo.Type()], len(t.destinationList), t.checkCycle)
	for k, s := range t.destinationList {
		fmt.Printf("Name: %s, Index: %2d, Host: %s, Port: %5d\n", s.name, k, s.host, s.port)
	}
}

//Runs periodic stats against destinations
func (bg *wormhole) stats() {
	for {
		for k := range bg.destinationList {
			bg.destinationList[k].Stat(bg.checkCycle / 2)
		}
		if bg.algo.Type() == CLOSEST {
			sort.Sort(svrList(bg.destinationList))
		}
		/*
			for _, be := range bg.destinationList {
				fmt.Printf("Stat: %s => %v\n", be.name, be.connectTime)
			}*/
		time.Sleep(time.Second * time.Duration(bg.checkCycle))
	}
}

func NetworkDataMover(wg *sync.WaitGroup, src, dst net.Conn, bufferSize uint) {
	dataBuffer := make([]byte, bufferSize)
	for {
		nr, _err := src.Read(dataBuffer)
		if _err != nil {
			if _err == io.EOF {
				//fmt.Printf("Connection closed by %s\n", src.RemoteAddr().String())
			}
			break
		}
		nw, _err := dst.Write(dataBuffer[:nr])
		if _err != nil || nr != nw {
			fmt.Printf("Error while writing to %s. Error=%s\n", dst.RemoteAddr().String(), _err)
			break
		}
	}
	wg.Done()
}

func sendProxyHeader(dst, src net.Conn) {
	shost, sport, _err := net.SplitHostPort(src.RemoteAddr().String())
	lhost, lport, _err := net.SplitHostPort(dst.LocalAddr().String())
	//_, lport, _err := net.SplitHostPort(dst.LocalAddr().String())
	if _err != nil {
		fmt.Printf("Cannot parse remote address => %s", _err)
	}
	proxy_header := fmt.Sprintf("PROXY TCP4 %s %s %s %s\r\n", shost, lhost, sport, lport)
	fmt.Println(proxy_header)
	_, _err = dst.Write([]byte(proxy_header))
	if _err != nil {
		fmt.Printf("Proxy header send error => %s", _err)
	}
}

func (bg *wormhole) handleConnection(client net.Conn) {

	wg := &sync.WaitGroup{}
	//fmt.Printf("Connection received from => %s\n", client.RemoteAddr().String())

	var b destination

	b = <-bg.beChan

	//fmt.Printf("Server selected -> %+v\n", b)

	destination, _err := b.Connect(bg.checkCycle)
	if _err != nil {
		fmt.Printf("Closing client connection => %s\n", client.RemoteAddr().String())
		client.Close()
		return
	}

	if b.sendProxy {
		sendProxyHeader(destination, client)
	}

	wg.Add(1)
	go NetworkDataMover(wg, client, destination, 1024*16)
	wg.Add(1)
	go NetworkDataMover(wg, destination, client, 1024*16)

	wg.Wait()
	_err = destination.Close()
	if _err != nil {
		fmt.Println("Error closing the destination connection=>", _err)
	}
	_err = client.Close()
	if _err != nil {
		fmt.Println("Error closing the client connection=>", _err)
	}
	//fmt.Println("Connection handle complete!!!")

}

func NewWormhole(tc wormholeConf, be ...destination) wormhole {
	return wormhole{name: tc.Name, port: tc.Port, beChan: make(chan destination), destinationList: be, algo: NewSelectionAlgo(tc.Algo), checkCycle: tc.Check}
}

func (bg *wormhole) Start(wg *sync.WaitGroup) {

	address := fmt.Sprintf("0.0.0.0:%d", bg.port)
	listener, _err := net.Listen("tcp4", address)
	if _err != nil {
		panic(errors.Errorf("Couldn't start the listener @%s due to an error=>%s\n", address, _err))
	}
	//start stat collection
	go bg.stats()

	//start destination generator algorithm
	go bg.algo.Select(bg.destinationList, bg.beChan)

	for {
		conn, _err := listener.Accept()
		if _err != nil {
			fmt.Printf("Error while accepting connection=>%s. Error=>%s\n", address, _err)
			continue
		}
		go bg.handleConnection(conn)
	}

}

func loadWormhole(wormholeConfig map[string]wormholeConf, destinations map[string]destination) map[string]wormhole {
	bgList := map[string]wormhole{}

	for k, v := range wormholeConfig {
		beList := []destination{}
		//bgConfig := destinationConfig{name: k, port: v.Port, algo: v.Algo}
		for _, b := range v.BeList {
			be := destinations[b]
			be.connectTime = make([]float64, v.Check)
			beList = append(beList, be)
		}
		bg := NewWormhole(v, beList...)
		bgList[k] = bg
	}
	return bgList
}

func Init(serverConfigFile, wormholeConfigFile string) {

	destinationConfig = loadServerFromConfig(serverConfigFile)
	wormholeConfig = loadWormholeFromConfig(wormholeConfigFile)

	destinations = loadServer(destinationConfig)
	wormholes = loadWormhole(wormholeConfig, destinations)

}

func Start() {

	fmt.Println("Creating wormholes...")
	wgWormhole := &sync.WaitGroup{}

	for _, t := range wormholes {

		wgWormhole.Add(1)

		fmt.Printf("Wormhole: %v\n", t.name)

		t.Info()

		go t.Start(wgWormhole)

	}

	wgWormhole.Wait()

}
