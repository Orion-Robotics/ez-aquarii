package camera

import (
	"io"
	"os"
	"syscall"
	"time"

	"github.com/team-orion/ez-aquarii/controller/module"
	"github.com/team-orion/ez-aquarii/gen/protocol"
	"github.com/team-orion/ez-aquarii/ipc"
)

type cameraModule struct {
	module.Dependencies
	stream io.ReadCloser
}

func NewModule(deps module.Dependencies) module.Module {
	return &cameraModule{
		Dependencies: deps,
	}
}

func (m *cameraModule) Name() string {
	return "camera"
}

func (m *cameraModule) Start() error {
	m.Logger.Info().Msg("opening camera pipe...")
	stream, err := m.openCameraStream()
	if err != nil {
		return err
	}
	m.Logger.Info().Msg("camera pipe opened")
	m.stream = stream
	parsed := &protocol.Packet{}
	for {
		if err := ipc.ReadProto(stream, parsed); err != nil {
			return err
		}
	}
}

func (m *cameraModule) openCameraStream() (*os.File, error) {
	timeoutNotifier := time.NewTicker(1 * time.Second)
	defer timeoutNotifier.Stop()

	openStart := time.Now()

	streamc := make(chan *os.File, 1)
	errc := make(chan error, 1)

	go func() {
		file, err := openReadFIFO(m.Config.CameraStreamPath)
		if err != nil {
			errc <- err
		}
		streamc <- file
	}()

	for {
		select {
		case <-timeoutNotifier.C:
			m.Logger.Warn().Msgf("no writer on camera pipe after %ds", int(time.Since(openStart).Seconds()))
		case stream := <-streamc:
			return stream, nil
		case err := <-errc:
			return nil, err
		}
	}
}

func (m *cameraModule) Stop() error {
	return m.stream.Close()
}

func openReadFIFO(path string) (*os.File, error) {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		if err := syscall.Mkfifo(path, 0666); err != nil {
			return nil, err
		}
	}
	return os.OpenFile(path, os.O_RDONLY, os.ModeNamedPipe)
}
