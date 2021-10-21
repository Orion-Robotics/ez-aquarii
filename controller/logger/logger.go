package logger

import (
	"os"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

type Logger struct {
	zerolog.Logger
}

func New() Logger {
	return Logger{
		Logger: log.
			Output(zerolog.ConsoleWriter{Out: os.Stdout}),
	}
}
