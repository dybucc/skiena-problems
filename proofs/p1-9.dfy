method Triple(x: int) returns (r: int)
  requires x % 2 == 0
  ensures r == 3 * x
{
  var a := Average(x - 1, x + 2) - x;
  // if x > 0 { a := a - x; }
  r := x * a * 6;
}

method Triple'(x: int) returns (r: int)
  ensures Average(r, 3 * x) == 3 * x
{
  if x == 0 { r := 0; } else {
    r := 0;
    // TODO
  }
}

function Average(a: int, b: int): int {
  (a + b) / 2
}
