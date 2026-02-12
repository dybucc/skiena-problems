method MaxSum(x: int, y: int) returns (s: int, m: int)
  ensures x >= y ==> m == x
  ensures x < y ==> m == y
  ensures s == x + y
{
  s := x + y;
  if x >= y {
    m := x;
  } else {
    m := y;
  }
}

method Test()
{
  var some, max := MaxSum(1928, 1);
  assert some == 1929;
  assert max == 1928;
}
