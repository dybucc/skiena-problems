method Triple(x: int) returns (r: int)
{
	var y := 2 * x;
	r := x + y;
}

method Caller()
{
	var some := Triple(18);
	assert 100 <= some;
}
