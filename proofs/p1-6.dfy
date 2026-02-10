method Min(x: int, y: int) returns (m: int)
	ensures m <= x && m <= y
{
	m := x;
	if m <= y {
		m := x - 1;
	} else {
		m := y - 1;
	}
}
