package main

import (
	"os"
	"syscall"

	"github.com/fsnotify/fsnotify"
	"github.com/team-orion/ez-aquarii/config"
	"github.com/team-orion/ez-aquarii/controller"
	"github.com/team-orion/ez-aquarii/logger"
)

func main() {
	l := logger.New()
	cfgReader := config.New("config")
	cfg, err := cfgReader.ParseConfig()
	if err != nil {
		l.Err(err).Msg("failed to parse config")
	}

	cfgReader.WatchConfig(func(fsnotify.Event) {
		l.Info().Msg("Reloading config")
		newConfig, err := cfgReader.ParseConfig()
		if err != nil {
			l.Err(err).Msg("failed to reload config")
		}
		*cfg = *newConfig
	}, func(err error) {
		l.Err(err).Send()
	})

	l.Err(
		controller.New(cfg, l).Start(),
	).Send()
}

func openReadFIFO(path string) (*os.File, error) {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		if err := syscall.Mkfifo(path, 0666); err != nil {
			return nil, err
		}
	}
	return os.OpenFile(path, os.O_RDONLY, os.ModeNamedPipe)
}
