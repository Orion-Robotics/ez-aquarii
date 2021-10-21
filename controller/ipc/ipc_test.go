package ipc_test

import (
	"bytes"
	"io"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/team-orion/ez-aquarii/ipc"
)

type MockInput struct {
	io.Reader
	io.Writer
}

func TestSerialize(t *testing.T) {
	var sentBuffer bytes.Buffer
	sentData := []byte("your mom")
	err := ipc.Write(&sentBuffer, sentData)
	assert.NoError(t, err)

	receivedData, err := ipc.Read(
		bytes.NewReader(sentBuffer.Bytes()),
	)
	assert.NoError(t, err)
	assert.Equal(t, sentData, receivedData)
}
