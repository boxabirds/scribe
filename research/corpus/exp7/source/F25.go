func parseSignedOffset(value string) int {
	sign := value[0]
	if sign != '-' && sign != '+' {
		return 0
	}
	x, rem, err := leadingInt(value[1:])

	// fail if nothing consumed by leadingInt
	if err != nil || value[1:] == rem {
		return 0
	}
	if x > 23 {
		return 0
	}
	return len(value) - len(rem)
}
