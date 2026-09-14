from typing import Any, Callable, Union

EcdsaSignature = tuple[int, int]
SignableMessage = Union[str, bytes, bytearray]
HashFunction = Callable[[Any], Any]
