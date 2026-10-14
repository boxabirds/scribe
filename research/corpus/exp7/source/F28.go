func (r *Reader) readContinuedLineSlice(lim int64, validateFirstLine func([]byte) error) ([]byte, error) {
	if validateFirstLine == nil {
		return nil, fmt.Errorf("missing validateFirstLine func")
	}

	// Read the first line.
	line, err := r.readLineSlice(lim)
	if err != nil {
		return nil, err
	}
	if len(line) == 0 { // blank line - no continuation
		return line, nil
	}

	if err := validateFirstLine(line); err != nil {
		return nil, err
	}

	// Optimistically assume that we have started to buffer the next line
	// and it starts with an ASCII letter (the next header key), or a blank
	// line, so we can avoid copying that buffered data around in memory
	// and skipping over non-existent whitespace.
	if r.R.Buffered() > 1 {
		peek, _ := r.R.Peek(2)
		if len(peek) > 0 && (isASCIILetter(peek[0]) || peek[0] == '\n') ||
			len(peek) == 2 && peek[0] == '\r' && peek[1] == '\n' {
			return trim(line), nil
		}
	}

	// ReadByte or the next readLineSlice will flush the read buffer;
	// copy the slice into buf.
	r.buf = append(r.buf[:0], trim(line)...)

	if lim < 0 {
		lim = math.MaxInt64
	}
	lim -= int64(len(r.buf))

	// Read continuation lines.
	for r.skipSpace() > 0 {
		r.buf = append(r.buf, ' ')
		if int64(len(r.buf)) >= lim {
			return nil, errMessageTooLarge
		}
		line, err := r.readLineSlice(lim - int64(len(r.buf)))
		if err != nil {
			return nil, err
		}
		r.buf = append(r.buf, trim(line)...)
	}
	return r.buf, nil
}
