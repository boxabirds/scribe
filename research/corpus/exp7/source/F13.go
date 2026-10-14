func prefixAndSuffix(pattern string) (prefix, suffix string, err error) {
	for i := 0; i < len(pattern); i++ {
		if IsPathSeparator(pattern[i]) {
			return "", "", errPatternHasSeparator
		}
	}
	if pos := bytealg.LastIndexByteString(pattern, '*'); pos != -1 {
		prefix, suffix = pattern[:pos], pattern[pos+1:]
	} else {
		prefix = pattern
	}
	return prefix, suffix, nil
}
