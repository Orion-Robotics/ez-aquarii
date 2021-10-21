package controller

import (
	"io"

	"github.com/team-orion/ez-aquarii/config"
	"github.com/team-orion/ez-aquarii/controller/module"
	"github.com/team-orion/ez-aquarii/controller/module/camera"
	"github.com/team-orion/ez-aquarii/logger"
)

// Controller is a robot instance
type Controller struct {
	cfg          *config.Config
	l            logger.Logger
	cameraStream io.Reader
	modules      map[string]module.Module
	// done is a channel that
	// notifies when the controller is shutting down
	done chan bool
}

func New(cfg *config.Config, l logger.Logger) *Controller {
	deps := module.Dependencies{
		Config: cfg,
		Logger: l,
	}

	return &Controller{
		l:    l,
		cfg:  cfg,
		done: make(chan bool),
		modules: module.ModuleMap(
			camera.NewModule(deps),
		),
	}
}

func (c *Controller) Start() error {
	for _, moduleName := range c.cfg.Enable {
		go func(name string) {
			if err := c.modules[name].Start(); err != nil {
				c.l.Err(err).Msgf("%s module exited", name)
			}
		}(moduleName)
	}
	c.l.Debug().Msg("Started!")
	for {
	}
}

func (c *Controller) Stop() {
	c.done <- true
}
