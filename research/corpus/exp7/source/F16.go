func (s *Scanner) setErr(err error) {
	if s.err == nil || s.err == io.EOF {
		s.err = err
	}
}
