package main

import "github.com/team-orion/ez-aquarii/gen"

func main() {
	packet := gen.Packet{
		Time: time.Now(),
	}
	bin, err := proto.Marshal(packet)
	if err != nil {
		panic(err)
	}
	fmt.Println(bin)
}
