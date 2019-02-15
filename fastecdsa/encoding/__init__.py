from abc import ABCMeta, abstractmethod


class KeyEncoder:
    """Base class that any encoding class for EC keys should derive from.

    All overriding methods should be static.
    """
    __metaclass__ = ABCMeta

    @abstractmethod
    def encode_public_key(Q):
        pass

    @abstractmethod
    def encode_private_key(d):
        pass

    @abstractmethod
    def decode_public_key(binary_data):
        pass

    @abstractmethod
    def decode_private_key(binary_data):
        pass


class SigEncoder:
    """Base class that any encoding class for EC signatures should derive from.

    All overriding methods should be static.
    """
    __metaclass__ = ABCMeta

    @abstractmethod
    def encode_signature(r, s):
        pass

    @abstractmethod
    def decode_signature(binary_data):
        pass
