package wormhole

import (
	"fmt"
	"net"
	"time"
)

var destinations map[string]destination

type destination struct {
	name        string
	host        string
	port        uint
	sendProxy   bool
	connectTime []float64
	isAlive     bool
}

type svrList []destination

func (b svrList) Less(i, j int) bool {
	var sumI, sumJ float64
	var samples = len(b[i].connectTime)
	//Weighted average, considering recent ones are more valued
	for s := 0; s < samples; s++ {
		sumI += b[i].connectTime[s] * float64((s+1)/samples)
		sumJ += b[j].connectTime[s] * float64((s+1)/samples)
	}
	return sumI < sumJ
}

func (b svrList) Len() int {
	return len(b)
}

func (b svrList) Swap(i, j int) {
	b[i], b[j] = b[j], b[i]
}

func (b *destination) Stat(timeout int) {
	//	for sample > 0 {
	start_time := time.Now()
	conn, _err := b.Connect(timeout)
	//time.Sleep(time.Millisecond * time.Duration(rand.Intn(100)))
	if _err != nil {
		fmt.Println("Couldnt connect destination. Error =>", _err)
		b.connectTime = append(b.connectTime[1:], 999999999)
		b.isAlive = false
	} else {
		end_time := time.Now()
		ct := float64(end_time.UnixNano()-start_time.Local().UnixNano()) / 1000000000.0
		//fmt.Printf("Connect time %3.3f\n", ct)
		conn.Close()
		//			sample--
		b.connectTime = append(b.connectTime[1:], ct)
		b.isAlive = true
	}
	//	}
	//fmt.Printf("Connect time: %+v\n", b.connectTime)
}

func (b *destination) Connect(timeout int) (net.Conn, error) {
	conn, _err := net.DialTimeout("tcp", fmt.Sprintf("%s:%d", b.host, b.port), time.Duration(timeout)*time.Second)
	if _err != nil {
		fmt.Printf("Couldn't connect to destination %s:%d due to an error=>%s\n", b.host, b.port, _err)
		//panic(errors.Errorf("Couldn't connect to destination @%s due to an error=>%s", address, _err))
	}
	return conn, _err
}

func loadServer(destinationConfig map[string]destinationConf) map[string]destination {
	beList := map[string]destination{}
	for k, v := range destinationConfig {
		beList[k] = destination{name: k, host: v.Host, port: v.Port, sendProxy: v.Proxy}
	}
	return beList
}
