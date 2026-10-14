func ReadUvarint(r io.ByteReader) (uint64, error) {
	var x uint64
	var s uint
	for i := 0; i < MaxVarintLen64; i++ {
		b, err := r.ReadByte()
		if err != nil {
			if i > 0 && err == io.EOF {
				err = io.ErrUnexpectedEOF
			}
			return x, err
		}
		if b < 0x80 {
			if i == MaxVarintLen64-1 && b > 1 {
				return x, errOverflow
			}
			return x | uint64(b)<<s, nil
		}
		x |= uint64(b&0x7f) << s
		s += 7
	}
	return x, errOverflow
}
