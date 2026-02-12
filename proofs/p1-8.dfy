method MaxSum(x: int, y: int) returns (s: int, m: int)
  ensures s == x + y
  ensures x >= y ==> m == x
  ensures x < y ==> m == y
{
  s := x + y;
  if x >= y {
    m := x;
  } else {
    m := y;
  }
}

method ReconstructMaxSum(s: int, m: int) returns (x: int, y: int)
  requires m >= s - m
  ensures s == x + y
  ensures (m == x || m == y) && x <= m && y <= m
{
  x := s - m;
  y := m;
}

method TestMaxSum(x: int, y: int) {
  var s, m := MaxSum(x, y);
  var xx, yy := ReconstructMaxSum(s, m);
  assert (xx == x && yy == y) || (xx == y && yy == x);
}
