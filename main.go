package main

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/golang/protobuf/proto"
	"github.com/team-orion/ez-aquarii/gen/protocol"
	"github.com/team-orion/ez-aquarii/ipc"
)

func main() {
	if err := exec.Command("./camera.sh", "").Start(); err != nil {
		panic(err)
	}

	cameraStream, err := os.OpenFile("./camerastream", os.O_RDONLY, os.ModeNamedPipe)
	if err != nil {
		panic(err)
	}

	for {
		data, err := ipc.Read(cameraStream)
		if err != nil {
			panic(err)
		}
		parsed := &protocol.Packet{}
		if err := proto.Unmarshal(data, parsed); err != nil {
			fmt.Printf("failed to parse: %v", err)
		}
		fmt.Println(parsed.GetTime())
	}
}
