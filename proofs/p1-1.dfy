method Triple(x: int) returns (r: int) {
  var y := x * 2;
  r := x + y;
  assert r == 3 * x + 1;
}
