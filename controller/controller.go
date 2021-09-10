package controller

import (
	"io"

	"github.com/team-orion/ez-aquarii/config"
	"github.com/team-orion/ez-aquarii/gen/protocol"
	"github.com/team-orion/ez-aquarii/ipc"
	"github.com/team-orion/ez-aquarii/logger"
)

// Controller is a robot instance
type Controller struct {
	cfg *config.Config
	l   logger.Logger
	// done is a channel that
	// notifies when the controller is shutting down
	done         chan bool
	cameraStream io.Reader
}

func New(l logger.Logger, cameraStream io.Reader) *Controller {
	return &Controller{
		l:            l,
		done:         make(chan bool),
		cameraStream: cameraStream,
	}
}

func (c *Controller) Start() error {
	c.l.Debug().Msg("Started!")
	if c.cfg.Enable.Camera {
		packets := make(chan *protocol.Packet)
		go c.streamCamera(c.done, packets)
		for {
			p := <-packets
			c.l.Debug().Time("received time", p.Time.AsTime())
		}
	}
	return nil
}

func (c *Controller) Stop() {
	c.done <- true
}

func (c *Controller) streamCamera(done chan bool, packetChan chan *protocol.Packet) {
	parsed := &protocol.Packet{}
	for {
		select {
		case <-done:
			return
		default:
			if err := ipc.ReadProto(c.cameraStream, parsed); err != nil {
				c.l.Err(err).Send()
				continue
			}
			packetChan <- parsed
		}
	}
}
