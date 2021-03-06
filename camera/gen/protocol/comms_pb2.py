# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protocol/comms.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='protocol/comms.proto',
  package='',
  syntax='proto3',
  serialized_options=b'B\nCommsProtoP\001Z-github.com/team-orion/ez-aquarii/gen/protocol',
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x14protocol/comms.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"%\n\x07Point2D\x12\x0c\n\x01x\x18\x01 \x01(\x05R\x01x\x12\x0c\n\x01y\x18\x02 \x01(\x05R\x01y\"A\n\x05Robot\x12$\n\x08position\x18\x01 \x01(\x0b\x32\x08.Point2DR\x08position\x12\x12\n\x04team\x18\x02 \x01(\x08R\x04team\"<\n\x04Goal\x12\x1e\n\x05point\x18\x01 \x01(\x0b\x32\x08.Point2DR\x05point\x12\x14\n\x05\x63olor\x18\x02 \x01(\x08R\x05\x63olor\"\x95\x01\n\x0bUpdateField\x12!\n\x08\x62lueGoal\x18\x01 \x01(\x0b\x32\x05.GoalR\x08\x62lueGoal\x12%\n\nyellowGoal\x18\x02 \x01(\x0b\x32\x05.GoalR\nyellowGoal\x12\x1c\n\x04\x62\x61ll\x18\x03 \x01(\x0b\x32\x08.Point2DR\x04\x62\x61ll\x12\x1e\n\x06robots\x18\x04 \x03(\x0b\x32\x06.RobotR\x06robots\"\\\n\x06Packet\x12.\n\x04time\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x04time\x12\"\n\x05\x66ield\x18\x02 \x01(\x0b\x32\x0c.UpdateFieldR\x05\x66ieldB=B\nCommsProtoP\x01Z-github.com/team-orion/ez-aquarii/gen/protocolb\x06proto3'
  ,
  dependencies=[google_dot_protobuf_dot_timestamp__pb2.DESCRIPTOR,])




_POINT2D = _descriptor.Descriptor(
  name='Point2D',
  full_name='Point2D',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='x', full_name='Point2D.x', index=0,
      number=1, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='x', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='y', full_name='Point2D.y', index=1,
      number=2, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='y', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=57,
  serialized_end=94,
)


_ROBOT = _descriptor.Descriptor(
  name='Robot',
  full_name='Robot',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='position', full_name='Robot.position', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='position', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='team', full_name='Robot.team', index=1,
      number=2, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='team', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=96,
  serialized_end=161,
)


_GOAL = _descriptor.Descriptor(
  name='Goal',
  full_name='Goal',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='point', full_name='Goal.point', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='point', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='color', full_name='Goal.color', index=1,
      number=2, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='color', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=163,
  serialized_end=223,
)


_UPDATEFIELD = _descriptor.Descriptor(
  name='UpdateField',
  full_name='UpdateField',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='blueGoal', full_name='UpdateField.blueGoal', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='blueGoal', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='yellowGoal', full_name='UpdateField.yellowGoal', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='yellowGoal', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ball', full_name='UpdateField.ball', index=2,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='ball', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='robots', full_name='UpdateField.robots', index=3,
      number=4, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='robots', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=226,
  serialized_end=375,
)


_PACKET = _descriptor.Descriptor(
  name='Packet',
  full_name='Packet',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='time', full_name='Packet.time', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='time', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='field', full_name='Packet.field', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, json_name='field', file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=377,
  serialized_end=469,
)

_ROBOT.fields_by_name['position'].message_type = _POINT2D
_GOAL.fields_by_name['point'].message_type = _POINT2D
_UPDATEFIELD.fields_by_name['blueGoal'].message_type = _GOAL
_UPDATEFIELD.fields_by_name['yellowGoal'].message_type = _GOAL
_UPDATEFIELD.fields_by_name['ball'].message_type = _POINT2D
_UPDATEFIELD.fields_by_name['robots'].message_type = _ROBOT
_PACKET.fields_by_name['time'].message_type = google_dot_protobuf_dot_timestamp__pb2._TIMESTAMP
_PACKET.fields_by_name['field'].message_type = _UPDATEFIELD
DESCRIPTOR.message_types_by_name['Point2D'] = _POINT2D
DESCRIPTOR.message_types_by_name['Robot'] = _ROBOT
DESCRIPTOR.message_types_by_name['Goal'] = _GOAL
DESCRIPTOR.message_types_by_name['UpdateField'] = _UPDATEFIELD
DESCRIPTOR.message_types_by_name['Packet'] = _PACKET
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Point2D = _reflection.GeneratedProtocolMessageType('Point2D', (_message.Message,), {
  'DESCRIPTOR' : _POINT2D,
  '__module__' : 'protocol.comms_pb2'
  # @@protoc_insertion_point(class_scope:Point2D)
  })
_sym_db.RegisterMessage(Point2D)

Robot = _reflection.GeneratedProtocolMessageType('Robot', (_message.Message,), {
  'DESCRIPTOR' : _ROBOT,
  '__module__' : 'protocol.comms_pb2'
  # @@protoc_insertion_point(class_scope:Robot)
  })
_sym_db.RegisterMessage(Robot)

Goal = _reflection.GeneratedProtocolMessageType('Goal', (_message.Message,), {
  'DESCRIPTOR' : _GOAL,
  '__module__' : 'protocol.comms_pb2'
  # @@protoc_insertion_point(class_scope:Goal)
  })
_sym_db.RegisterMessage(Goal)

UpdateField = _reflection.GeneratedProtocolMessageType('UpdateField', (_message.Message,), {
  'DESCRIPTOR' : _UPDATEFIELD,
  '__module__' : 'protocol.comms_pb2'
  # @@protoc_insertion_point(class_scope:UpdateField)
  })
_sym_db.RegisterMessage(UpdateField)

Packet = _reflection.GeneratedProtocolMessageType('Packet', (_message.Message,), {
  'DESCRIPTOR' : _PACKET,
  '__module__' : 'protocol.comms_pb2'
  # @@protoc_insertion_point(class_scope:Packet)
  })
_sym_db.RegisterMessage(Packet)


DESCRIPTOR._options = None
# @@protoc_insertion_point(module_scope)
