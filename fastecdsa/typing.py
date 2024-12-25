from typing import Any, Callable, Tuple, Union

EcdsaSignature = Tuple[int, int]
SignableMessage = Union[str, bytes, bytearray]
HashFunction = Callable[[Any], Any]
