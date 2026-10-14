func newInt64Value(val int64, p *int64) *int64Value {
	*p = val
	return (*int64Value)(p)
}
