package module

import (
	"github.com/team-orion/ez-aquarii/config"
	"github.com/team-orion/ez-aquarii/logger"
)

type Dependencies struct {
	Config *config.Config
	Logger logger.Logger
}

type Module interface {
	Name() string
	Start() error
	Stop() error
}

type NewModule = func(Dependencies) Module

func ModuleMap(modules ...Module) map[string]Module {
	out := map[string]Module{}
	for _, m := range modules {
		out[m.Name()] = m
	}
	return out
}
