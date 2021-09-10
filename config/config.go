package config

import (
	_ "embed"
	"errors"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/fsnotify/fsnotify"
	"gopkg.in/yaml.v3"
)

//go:embed default.yml
var defaultConfig []byte

type Enable struct {
	Camera bool
}

type Config struct {
	CameraStreamPath string `yaml:"cameraStreamPath"`
	Enable           Enable
}

type ConfigReader struct {
	ConfigName string
}

func New(name string) *ConfigReader {
	return &ConfigReader{
		ConfigName: name + ".yaml",
	}
}

func (c *ConfigReader) ParseConfig() (*Config, error) {
	conf := &Config{}

	dat, err := ioutil.ReadFile(c.ConfigName)
	if err != nil {
		if os.IsNotExist(err) {
			err := ioutil.WriteFile(c.ConfigName, defaultConfig, 0o660)
			if err != nil {
				return nil, fmt.Errorf("failed to write default config: %+w", err)
			}

			return nil, errors.New("default configuration has been created, please edit it")
		}

		return nil, fmt.Errorf("failed to read config file: %+w", err)
	}
	err = yaml.Unmarshal(dat, conf)
	if err != nil {
		return nil, fmt.Errorf("failed to parse config file: %+w", err)
	}

	return conf, nil
}

func (c *ConfigReader) WatchConfig(onChange func(fsnotify.Event), onError func(error)) error {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return err
	}

	go func() {
		for {
			select {
			case event := <-watcher.Events:
				onChange(event)
			case err := <-watcher.Errors:
				onError(err)
			}
		}
	}()

	if err := watcher.Add(c.ConfigName); err != nil {
		return err
	}

	return nil
}
