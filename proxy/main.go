
package main

import (
	"proxy/wormhole"
)

func main() {

	wormhole.Init("server.json", "wormhole.json")

	wormhole.Start()

}
