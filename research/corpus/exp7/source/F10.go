func ParseInt(s string, base int, bitSize int) (i int64, err error) {
	const fnParseInt = "ParseInt"

	if s == "" {
		return 0, ErrSyntax
	}

	// Pick off leading sign.
	neg := false
	switch s[0] {
	case '+':
		s = s[1:]
	case '-':
		s = s[1:]
		neg = true
	}

	// Convert unsigned and check range.
	var un uint64
	un, err = ParseUint(s, base, bitSize)
	if err != nil && err != ErrRange {
		return 0, err
	}

	if bitSize == 0 {
		bitSize = IntSize
	}

	cutoff := uint64(1 << uint(bitSize-1))
	if !neg && un >= cutoff {
		return int64(cutoff - 1), ErrRange
	}
	if neg && un > cutoff {
		return -int64(cutoff), ErrRange
	}
	n := int64(un)
	if neg {
		n = -n
	}
	return n, nil
}
