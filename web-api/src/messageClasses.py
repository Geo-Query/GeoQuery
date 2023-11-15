import typing

LAT_LONG: int = 0
BYTE_ORDER: typing.Literal['little', 'big'] = "little"

# Place message types here
message_types = [LAT_LONG]


class MessageHeader:
    message_type: int
    message_length: int

    def __init__(self, message_type: int, message_length: int):
        assert message_type in message_types
        assert message_length >= 0
        assert message_length < 4294967295
        self.message_type = message_type
        self.message_length = message_length

    def to_bytes(self) -> bytearray:
        build = bytearray()
        build.extend(self.message_type.to_bytes(2, byteorder=BYTE_ORDER))
        build.extend(self.message_length.to_bytes(4, byteorder=BYTE_ORDER))
        assert len(build) == 6
        return build


class MessageContent:
    content: bytes

    def __init__(self, content: bytes):
        self.content = content

    def __len__(self) -> int:
        return len(self.content)


class Message:
    message_header: MessageHeader
    message_content: MessageContent

    def __int__(self, content: MessageContent, message_type: int):
        self.message_content = content
        self.message_header = MessageHeader(message_type, len(content))

    def build(self) -> bytearray:
        build = bytearray()
        build.extend(self.message_header.to_bytes())
        build.extend(self.message_content.content)
        return build
