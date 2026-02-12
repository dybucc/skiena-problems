method Triple(x: int) returns (r: int)
  ensures r == 3 * x
{
  r := Average(x, 5 * x);
}

method Triple'(x: int) returns (r: int)
  ensures Average(r, 3 * x) == 3 * x
{
  // TODO: get the thing  done.
  r := 3 * x;
}

function Average(a: int, b: int): int {
  (a + b) / 2
}
