from enum import Enum
from typing import Optional, Any, TypeVar, Type, cast


T = TypeVar("T")
EnumT = TypeVar("EnumT", bound=Enum)


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def from_none(x: Any) -> Any:
    assert x is None
    return x


def from_union(fs, x):
    for f in fs:
        try:
            return f(x)
        except:
            pass
    assert False


def to_enum(c: Type[EnumT], x: Any) -> EnumT:
    assert isinstance(x, c)
    return x.value


def to_class(c: Type[T], x: Any) -> dict:
    assert isinstance(x, c)
    return cast(Any, x).to_dict()


class Command(Enum):
    GET_SERVER_INFO = "get_server_info"
    SOME_OTHER_COMMAND = "some_other_command"


class Generated:
    command: Command
    with_stuff: Optional[str]

    def __init__(self, command: Command, with_stuff: Optional[str]) -> None:
        self.command = command
        self.with_stuff = with_stuff

    @staticmethod
    def from_dict(obj: Any) -> 'Generated':
        assert isinstance(obj, dict)
        command = Command(obj.get("command"))
        with_stuff = from_union([from_str, from_none], obj.get("with_stuff"))
        return Generated(command, with_stuff)

    def to_dict(self) -> dict:
        result: dict = {}
        result["command"] = to_enum(Command, self.command)
        if self.with_stuff is not None:
            result["with_stuff"] = from_union([from_str, from_none], self.with_stuff)
        return result


def generated_from_dict(s: Any) -> Generated:
    return Generated.from_dict(s)


def generated_to_dict(x: Generated) -> Any:
    return to_class(Generated, x)
