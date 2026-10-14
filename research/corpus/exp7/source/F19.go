func nextRune(b []byte) rune {
	r, _ := utf8.DecodeRune(b)
	return r
}
