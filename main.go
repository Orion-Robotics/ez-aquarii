package main

import (
	"fmt"
	"os"
	"time"

	"github.com/golang/protobuf/proto"
	"github.com/team-orion/ez-aquarii/gen/protocol"
	"github.com/team-orion/ez-aquarii/ipc"
)

func main() {
	// if err := exec.Command("./camera.sh", "").Start(); err != nil {
	// 	panic(err)
	// }

	cameraStream, err := os.OpenFile("./camerastream", os.O_RDONLY, os.ModeNamedPipe)
	if err != nil {
		panic(err)
	}

	start := time.Now()
	for i := 0; i < 3000; i++ {
		fmt.Println(i)
		data, err := ipc.Read(cameraStream)
		if err != nil {
			panic(err)
		}
		parsed := &protocol.Packet{}
		if err := proto.Unmarshal(data, parsed); err != nil {
			fmt.Printf("failed to parse: %v", err)
		}
	}
	fmt.Printf("%v\n", time.Since(start))
}
