    def __init__(self, timefunc=_time, delayfunc=time.sleep):
        """Initialize a new instance, passing the time and delay
        functions"""
        self._queue = []
        self._lock = threading.RLock()
        self.timefunc = timefunc
        self.delayfunc = delayfunc
        self._sequence_generator = count()
