    def update(self, msg):
        """Feed data from msg into this hashing object."""
        inst = self._hmac or self._inner
        inst.update(msg)
