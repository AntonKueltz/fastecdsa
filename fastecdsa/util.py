import hmac


class RFC6979:
    ''' deterministic nonce generation for ECDSA '''
    def __init__(self, msg, x, q, hashfunc):
        self.x = x
        self.q = q
        self.msg = msg
        self.qlen = len(bin(q)) - 2  # -2 for the leading '0b'
        self.rlen = ((self.qlen + 7) / 8) * 8
        self.hashfunc = hashfunc

    def _bits2int(self, b):
        ''' http://tools.ietf.org/html/rfc6979#section-2.3.2 '''
        i = int(b.encode('hex'), 16)
        blen = len(b) * 8

        if blen > self.qlen:
            i >>= (blen - self.qlen)

        return i

    def _int2octets(self, x):
        ''' http://tools.ietf.org/html/rfc6979#section-2.3.3 '''
        octets = ''

        while x > 0:
            octets = chr(0xff & x) + octets
            x >>= 8

        padding = chr(0x00) * ((self.rlen / 8) - len(octets))
        return padding + octets

    def _bits2octets(self, b):
        ''' http://tools.ietf.org/html/rfc6979#section-2.3.4 '''
        z1 = self._bits2int(b)  # -2 for the leading '0b'
        z2 = z1 % self.q
        return self._int2octets(z2)

    def gen_nonce(self):
        ''' http://tools.ietf.org/html/rfc6979#section-3.2 '''
        h1 = self.hashfunc(self.msg)
        hash_size = h1.digest_size
        h1 = h1.digest()
        key_and_msg = self._int2octets(self.x) + self._bits2octets(h1)

        v = ''.join([chr(0x01) for _ in range(hash_size)])
        k = ''.join([chr(0x00) for _ in range(hash_size)])

        k = hmac.new(k, v + chr(0x00) + key_and_msg, self.hashfunc).digest()
        v = hmac.new(k, v, self.hashfunc).digest()
        k = hmac.new(k, v + chr(0x01) + key_and_msg, self.hashfunc).digest()
        v = hmac.new(k, v, self.hashfunc).digest()

        while True:
            t = ''

            while len(t) * 8 < self.qlen:
                v = hmac.new(k, v, self.hashfunc).digest()
                t = t + v

            nonce = self._bits2int(t)
            if nonce >= 1 and nonce < self.q:
                return nonce

            k = hmac.new(k, v + chr(0x00), self.hashfunc).digest()
            v = hmac.new(k, v, self.hashfunc).digest()
