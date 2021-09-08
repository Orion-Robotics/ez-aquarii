package main

import (
	"fmt"
	"time"

	"github.com/team-orion/ez-aquarii/gen"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/timestamppb"
)

func main() {
	for {
		packet := &gen.Packet{
			Time: timestamppb.New(
				time.Now(),
			),
		}
		bin, err := proto.Marshal(packet)
		if err != nil {
			panic(err)
		}
		fmt.Println(string(bin))
		time.Sleep(1 * time.Second)
	}
}
