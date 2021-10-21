package ipc

import (
	"encoding/binary"
	"io"

	"google.golang.org/protobuf/proto"
)

func Read(in io.Reader) ([]byte, error) {
	var length int32
	if err := binary.Read(in, binary.LittleEndian, &length); err != nil {
		return nil, err
	}
	buffer := make([]byte, length)
	if _, err := in.Read(buffer); err != nil {
		return nil, err
	}
	return buffer, nil
}

func Write(out io.Writer, buffer []byte) error {
	len := int32(len(buffer))
	if err := binary.Write(out, binary.LittleEndian, len); err != nil {
		return err
	}

	_, err := out.Write(buffer)
	return err
}

func ReadProto(in io.Reader, message proto.Message) error {
	data, err := Read(in)
	if err != nil {
		return err
	}
	return proto.Unmarshal(data, message)
}
