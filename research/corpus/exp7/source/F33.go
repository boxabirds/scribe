func typeEncoder(t reflect.Type) encoderFunc {
	if fi, ok := encoderCache.Load(t); ok {
		return fi.(encoderFunc)
	}

	// To deal with recursive types, populate the map with an
	// indirect func before we build it. If the type is recursive,
	// the second lookup for the type will return the indirect func.
	//
	// This indirect func is only used for recursive types,
	// and briefly during racing calls to typeEncoder.
	indirect := sync.OnceValue(func() encoderFunc {
		return newTypeEncoder(t, true)
	})
	fi, loaded := encoderCache.LoadOrStore(t, encoderFunc(func(e *encodeState, v reflect.Value, opts encOpts) {
		indirect()(e, v, opts)
	}))
	if loaded {
		return fi.(encoderFunc)
	}

	f := indirect()
	encoderCache.Store(t, f)
	return f
}
