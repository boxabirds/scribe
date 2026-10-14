    def iterkeys(self):
        try:
            return iter(self._index)
        except TypeError:
            raise error('DBM object has already been closed') from None
