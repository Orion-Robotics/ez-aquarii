package main

import (
	"bufio"
	"fmt"
	"os/exec"

	"github.com/golang/protobuf/proto"
	"github.com/team-orion/ez-aquarii/gen/protocol"
)

func newCommandScanner(command string) (*bufio.Scanner, error) {
	cmd := exec.Command(command, "")
	output, err := cmd.StdoutPipe()
	if err != nil {
		return nil, err
	}
	if err := cmd.Start(); err != nil {
		return nil, err
	}
	scanner := bufio.NewScanner(output)
	return scanner, nil
}

func main() {
	scanner, err := newCommandScanner("./camera.sh")
	if err != nil {
		panic(err)
	}
	for scanner.Scan() {
		data := scanner.Bytes()
		fmt.Printf("%#v\n", string(data))
		parsed := &protocol.Packet{}
		if err := proto.Unmarshal(data, parsed); err != nil {
			fmt.Printf("failed to parse: %v", err)
		}
		fmt.Println(parsed.GetTime())
	}
}
