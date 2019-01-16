from abc import ABCMeta, abstractmethod


class KeyEncoder:
    __metaclass__ = ABCMeta

    @abstractmethod
    def encode_public_key(self, Q):
        pass

    @abstractmethod
    def encode_private_key(self, d, Q=None, curve=None):
        pass

    @abstractmethod
    def decode_public_key(self, binary_data):
        pass

    @abstractmethod
    def decode_private_key(self, binary_data):
        pass


class SigEncoder:
    __metaclass__ = ABCMeta

    @abstractmethod
    def encode_signature(self, r, s):
        pass

    @abstractmethod
    def decode_signature(self, binary_data):
        pass
