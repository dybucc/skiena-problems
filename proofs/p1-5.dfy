method Triple(x: int) returns (r: int)
	requires x % 4 == 0
	ensures r == 3 * x
{
	var y := x / 2;
	r := 6 * y;
}
