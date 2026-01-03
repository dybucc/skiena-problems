#import "@local/typst-template:0.23.0": *

#show: template.with(
  title: [DSA],
  authorship: (
    (
      name: "Adam Martinez",
      affiliation: "University of Life",
      email: "adammartinezoussat@gmail.com",
    ),
  ),
)

= The Algorithm Design Manual

== Introduction to Algorithm Design

/ Problem 1--9: \
  The algorithm proves correct because the evolution of the resulting polynomial leads to a
  progressive multiplication of the unknown by all former factors, such that the largest exponent of
  the unknown in the polynomial remains the largest polynomial at the end of the multiplication.
  Because the additional sum in the multiplication is also multiplied by the same additional factor
  of $x$ so long as the control variable is less than the smallest factor, namely 0, we can state
  that the last factor in the resulting polynomial will always avoid multiplication by the
  polynomial (i.e. the last constant factor is not multiplied by the unknown, as the problem expects
  from the polynomial of the form provided in the statement.)

/ Problem 1--10: \
  The algorithm considers the whole sequence for as many iterations as there are elements of the
  sequence minus 1. Throuhgout such iterations, the algorithm considers comparisons between all
  elements of each of the subsequences that have not yet been sorted, such that all numbers that are
  in their right order--statistic will always remain in the tail end of the original sequence. This
  makes for an incrementally smaller amount of comparisons across iterations, as each iteration
  discards the last elements of the last subsequence formed through iterations, knowing the
  order--statistic of the elements is that of incremental ordering.

/ Problem 1--11: \
  The problem is solved through a recursive procedure whereby the base assumption is that the GCD of
  a number and 0 is always going to be that number. Based on this, it performs the modulo operation
  between the numbers knowing that the residue of integer division represents the imprecission with
  which a number cannot be divided by another number. This operation also denotes the fact that the
  left operand is capable of being divided by all prime factors of the number to the left, such that
  if that is not the case, we are provided with a residue. This being the difference between the
  largest factor of a given number right before reaching the target number, it itself can be used as
  the object of the next modulo operation with the number whose multiple did not quite hit the goal;
  this will produce an increasingle smaller sequence of numbers that try to "fill up" the entirety
  of the original number by performing the largest possible division and only stopping once such
  that division yields a modulo 0 (i.e. the number yield by integer division is indeed the largest
  possible number fitting the ever present difference between the initial numbers' largest prime
  factors; an alternative definition for the GCD.)

/ Problem 1--12: \
  The problem statement provides information on one of the basic formulas for sums. This formula is
  a consequence of summing some number starting at 1 to some other number $>= 0$. The base
  assumption in the induction is that the target number $n$ in the sum

  $
    sum^n_(i = 0) i
  $

  is expected to recurse for numbers larger than $n = 0$. Thus, the recurrence relation on the
  evaluation of the function (i.e. not on its time complexity) is

  $
    f(n, i) = cases(
      n & "if" i = n\,,
      0 & "if" n = 0\,,
      i + f(n, i + 1) & "otherwise".
    )
  $ <p112-rec-rel>

  Assumming that $i = 1$ at the start of the call, the resulting stack would look something like the
  following.

  $
    i + f(n, i + f(n, dots.c space n)) = (n dot (n + 1)) / 2.
  $ <p112-gen-stack>

  Surely the factor taking on the value of $n$ must be equal to the same factor as the one in the
  resulting formula. This is due to the fact that we always perform $n$ sums. Now, the reason why
  the formula is capable of generalizing the behavior of all terms in the sum to $(n + 1) / 2$ is
  yet unknown to me. Say we took $n = 4$. In this case, we would see ourselves with the following
  call stack as per the recurrence relation described in @p112-rec-rel.

  $
    1 + f(4, 1 + f(4, 1 + f(4, 1 + f(4, 1)))).
  $

  Or maybe my assumption on the function's recurrence is wrong for the case where $i != n, 0$. This
  is likely the case, because otherwise the recursion would not be discrete for discrete values of
  $n$. Thus, reformulated, the function's evaluation recurrence could be as follows.

  $
    f(n, i) = cases(
      n & "if" i = n\,,
      0 & "if" i = 0\,,
      i + f(n, i + 1) & "otherwise".
    )
  $

  This relation turned out to be the same as the one initially formulated in @p112-rec-rel. That
  likely means what's wrong was my base assumption on the generalized behavior of the recursed stack
  in @p112-gen-stack.

  If that is, indeed, the case, then the real generalized behavior would look something like the
  following.

  $
    i + f(n, i + 1) => (i + 1) + f(n, (i + 1) + 1) => dots.c & => (i + 1 + dots.c + 1) + f(n, n) \
                                                             & => (n - 1) + f(n, n).
  $

  Based on the observation of the behavior of the function, it seems like by the end iteration, once
  $i = n$, the additional factor $(i + 1 + dots.c + 1)$ is likely to evaluate to $n - 1$. Thus, for
  some $n = 4$, the development of the above formula would be as follows.

  $
    1 + f(4, 1 + 1) & => (1 + 1)       & + & f(4, (1 + 1) + 1) \
                    & => ((1 + 1) + 1) & + & f(4, ((1 + 1) + 1) + 1) \
                    & =>               &   & (((1 + 1) + 1) + 1).
  $

  Upon unwinding the stack, the above formulation would end up looking like the sum of all integer
  values going from 1 to $n = 4$, indeed.

  $
    1 + f(4, 1 + 1) & => (1 + 1)       & + & ((1 + 1) + 1) + (((1 + 1) + 1) + 1) \
                    & => ((1 + 1) + 1) & + & (((1 + 1) + 1) + 1) \
                    & =>               &   & (((1 + 1) + 1) + 1). \
  $ <p112-rewind>
  #v(-.5em)
  #un-math[$
    1 + (1 + 1) + ((1 + 1) + 1) + (((1 + 1) + 1) + 1).
  $]

  Clearly, we're performing an $n$ number of sums, and thus that explains the $n$ factor in the
  resulting formula, but I can't quite figure out the reason behind the $(n + 1) / 2$ factor. It
  seems as if the generalization of the sum of any number $n$ from 1 to such number $n$ is dominated
  by half the integer following such number. For $n = 4$, $(n + 1) / 2 = 5 / 2$, and thus
  $4 dot 5 / 2 = 8 / 2 dot 5 / 2 = 40 / 4 = 10$.

  Maybe the relation is drawn from the participating sums in the resulting expression; Where the
  last factor is always known to be $n$ and the first factor is always known to be 1, the middle
  factors ought have some relationship to the last factor such that $(n + 1) / 2$ produces the right
  result. Surely there is a pattern in the evolution of those middle factors between the target $n$
  and the initial 1.

  Say we took $n = 3$. One would easily realize that the middle factors in $1 + dots.c + n$ ought
  sum to $n - 1$, for $1 + 1$, the factor following the initial 1, is the only one between it and
  this $n$. Unlike with $n = 3$, though, $n = 4$ yields a different expression in terms of $n$ for
  the middle factors: $n + 1$. I can't quite discern a pattern in the signs, beyond a possibly
  even/odd--relationship, which I am lead to believe will not help much for larger values of $n$.

  For $n = 5$, the middle terms add up to 9, which indeed, yields $n + 4$. Maybe this describes a
  series related to powers of 2? Further, for $n = 6$, the middle terms sum to $n + 8$. Indeed, it
  seems as if the added factor could be described in terms of a power of 2 or possibly a fraction
  with denominator 2. What about $n = 7$? The mid--terms sum to $n + 13$. Welp. No power of 2 nor
  fraction with denominator 2 can possibly describe 13.

  But maybe there's still hope for a pattern; It seems as if the sum of the considered $n$ with the
  additional factor of the mid--terms of the prior $n$, namely $n - 1$, seem to add up to a number
  that, subtracting 2, yields the right mid--terms for the current value of $n$. Take $n = 5$.
  Considering the sum for $n - 1$ yields mid--terms $(n - 1) + 1$, one can state that
  $n + (n + 1 - 2) = n + 4$. Indeed, because the mid--terms of some $n$ integrate those of $n - 1$,
  we can express them in terms of $n - 1$'s sum of mid--terms.

  Take now $n = 6$. The mid--terms for $n - 1 = 5$, yield $(n - 1) + 4$, and thus
  $n + (n + 4 - 2) = n + 8$. As it turns out, that is, indeed, the sum of the mid--terms of $n = 6$.
  But let us test in full the results of our prior findings; Take now $n = 7$. For $n - 1 = 6$, we
  have just described how the mid--terms are given by $(n - 1) + 8$, so the mid--terms of $n = 7$
  should be described by $n + (n + 8 - 2) = n + 13$. Well, that seems like a fine result, indeed.

  Let us test now the mid--terms for $n = 8$, where we know the mid--terms of $n - 1 = 7$ to be
  $(n - 1) + 13$, we should also know the mid--terms of $n = 8$ to be $n + (n + 13 - 2) = n + 19$.
  This, indeed, provides the correct result, as it is proven by the fact that

  $
    sum_(i = 0)^(n = 8) i = (n dot (n + 1)) / 2 = (8 dot (8 + 1)) / 2 & = 36. \
                                                         36 - (8 + 1) & = 27.
  $

  The issue in our formulation lies now in the fact that the sum of some number $n$ depends on
  knowing not only the sum of the mid--terms for the sum of $n - 1$, but on being capable of
  expressing those terms in the form $(n - 1) + k$, where $k$ is the key number in computing the
  mid--terms of $n$ through the expression $n + (n + k - 2)$, such that the total sum may be
  expressed as

  $
    underbrace(n + (n + k - 2), #[mid--terms]) + overbrace((n + 1), #[initial and final terms]).
  $

  For this, though, there may still be a pattern yet to be exploited. Consider the value of $k$ as
  we moved through $n = 5, 6, 7, 8$.

  $
    n = 5 & => k = 4, \
    n = 6 & => k = 8, \
    n = 7 & => k = 13, \
    n = 8 & => k = 19.
  $

  Further examination leads to $n = 9 => k = 26, n = 10, k = 34$. I struggle now to see the pattern
  I believed there to exist. Maybe there's some relationship in the way these numbers add up
  together?

  $
    & "For" & n && = & 5,  & k && = & 4  & => & 4 - 5   & = & -1, \
    & "For" & n && = & 6,  & k && = & 8  & => & 8 - 6   & = & 2, \
    & "For" & n && = & 7,  & k && = & 13 & => & 13 - 7  & = & 6, \
    & "For" & n && = & 8,  & k && = & 19 & => & 19 - 8  & = & 11, \
    & "For" & n && = & 9,  & k && = & 26 & => & 26 - 9  & = & 17, \
    & "For" & n && = & 10, & k && = & 34 & => & 34 - 10 & = & 24, \
    & "For" & n && = & 11, & k && = & 54 & => & 54 - 11 & = & 43. \
  $

  The value of $k$ is wrong for all of the above; $k$ is supposed to be factor in $(n - 1) + k$ used
  to compute the mid--terms of $n - 1$. With this correction, maybe we can find another pattern in
  these results?

  $
    & "For" & n && = & 6,  & k && = & 4 , \
    & "For" & n && = & 7,  & k && = & 8 , \
    & "For" & n && = & 8,  & k && = & 7 , \
    & "For" & n && = & 9,  & k && = & 19, \
    & "For" & n && = & 10, & k && = & 26, \
    & "For" & n && = & 11, & k && = & 34. \
  $

  There doesn't seem to be a pattern in this series. Let us rewind back to the point where we
  concluded that the total number of sums is equal to the multiplication of $n$, namely, to
  @p112-rewind.

  At that point, we had concluded that we performed $n$ sums, but we weren't capable of deriving the
  meaning behind the $(n + 1) / 2$ factor. Now, were we to compute $n dot n$, we would be computing
  $n$ sums of the value of $n$, of which to compute the sum as we know, we would be required to
  subtract from each of those $n$ terms an increasingly larger term $k$.

  This relationship, for some discrete $n = 4$, translates as follows.

  $
    4 + 4 + 4 + 4 & =  && (1 + 1 + 1 + 1)     & + & (1 + 1 + 1 + 1) + \
                  &    && (1 + 1 + 1 + 1)     & + & (1 + 1 + 1 + 1). \
                  & => && (1 + 1 + 1 + 1 - 3) & + & (1 + 1 + 1 + 1 - 2) + \
                  &    && (1 + 1 + 1 + 1 - 1) & + & (1 + 1 + 1 + 1). \
  $

  This leads to the following conclusion, as per the recurrence relation defined in @p112-rec-rel.

  $
    sum_(i = 1)^(n) i = n^2 - sum_(i = 1)^(n - 1) i = n^2 - ((n - 1)^2 - (dots.c - 0)), "for" n >= 0.
  $

  To avoid computing the sum in terms of another sum, we may choose to compute $(n + 1) dot n$,
  which for a discrete value $n = 4$ yields

  $
    5 + 5 + 5 + 5 & =  && (1 + 1 + 1 + 1 + 1)     & + & (1 + 1 + 1 + 1 + 1) + \
                  &    && (1 + 1 + 1 + 1 + 1)     & + & (1 + 1 + 1 + 1 + 1). \
                  & => && (1 + 1 + 1 + 1 + 1 - 2) & + & (1 + 1 + 1 + 1 + 1 - 1) + \
                  &    && (1 + 1 + 1 + 1 + 1 - 0) & + & (1 + 1 + 1 + 1 + 1). \
  $

  I see it now. Computing $n$ sums of $n + 1$ yields a set of terms where, compared with the sum
  proper, we can compute it one of two ways, from the right or from the left. Mathematically
  speaking, this means each unit subsum of $n + 1$ yields a value that is equal to the same subsum
  minus an increasingly smaller $k$, where $1 <= k < n + 1$.

  This implies that for some term $n$, its sum in terms of $n + 1$ is given by

  $
    sum_(i = 1)^n i = & (1 + 1 + dots.c + 1_(n + 1) - n)_1 + (1 + 1 + dots.c + 1_(n + 1) - (n - 1))_2 + dots.c \
    & dots.c + (1 + 1 + dots.c + 1_(n + 1) - 1)_n.
  $

  Because the series described by the ever--present negative term $n - k$, where $0 <= k < n$, can
  further expand in the development of the sum to

  $
    sum_(i = 1)^n i & = && (n + 1)_1 + (n + 1)_2 + dots.c + (n + 1)_n - (n + (n - 1) + dots.c + 1) \
    & = && (n + 1)_1 + (n + 1)_2 + dots.c + (n + 1)_n - ((n + 1) + n + dots.c + 1) \
    & = && (n + 1)_1 + (n + 1)_2 + dots.c + (n + 1)_n - ((n + 1) + (n + 1) + dots.c + 1) \
    & = && (n + 1)_1 + (n + 1)_2 + dots.c + (n + 1)_n - (n / 2 dot (n + 1)) \
    & = && n dot (n + 1) - n / 2 dot (n + 1) \
    & = && (n dot (n + 1)) / 2.
  $ <p112-basic-sum-c1>

  And this proves the sum formula correct. The key was in finding the equivalence between a sum of
  integer terms $[n, 1]$ to be $(n dot (n + 1)) / 2$. If we look closely at the sum
  $n + (n - 1) + dots.c + 1$, we realize that by grouping the first and last term, we consistently
  form $n + 1$ terms, that on an even $n$ produce a division of the initial $n$--sized group into
  $n / 2$ subsets of $n + 1$. On odd values of $n$, the sequence always yields the expression
  $(n + 1) + (n + 1) + dots.c + (n + 1) / 2$, where there are $floor(n / 2) dot (n + 1)$ terms akin
  to the prior terms, and one $(n + 1) / 2$ term, totaling

  $
    floor(n / 2) dot (n + 1) + (n + 1) / 2 & = ((n - 1) dot (n + 1)) / 2 + (n + 1) / 2 \
                                           & = ((n - 1 + 1) dot (n + 1)) / 2 \
                                           & = (n dot (n + 1)) / 2.
  $ <p112-basic-sum-c2>

/ Problem 1--13: \
  The problem asks for another famous formula on sums, that being a polynomial where the $i$ control
  variable is squared. Proving this should be easier than the prior formula because I can account
  for that formula as a true statement not requiring proof. The next statement to prove should also
  be fairly simple to prove considering it raises the constant exponential factor to 3.

  The formula in question is

  $
    sum_(i = 1)^n i^2, "for" n >= 0.
  $

  This formula is really the same as the prior formula, whereby instead of considering a single
  factor of $i$, I am to consider two factors of $i$.

  $
    sum_(i = 1)^n i dot i = & ((1 + 1 + dots.c + 1_(n + 1) - n) &dot& (1 + 1 + dots.c + 1_(n + 1) - n)) &&+ \
    & ((1 + 1 + dots.c + 1_(n + 1) - (n - 1)) &dot& (1 + 1 + dots.c + 1_(n + 1) - (n - 1))) &&+ dots.c + \
    & ((1 + 1 + dots.c + 1_(n + 1) - 1) &dot& (1 + 1 + dots.c + 1_(n + 1) - 1)).&&
  $

  Factorization can now be performed in terms of the additional $n$--term in each of the sum terms,
  as the same $n$--term is mutliplying both expansions of each value of $i$ throughout the series.
  This can be done by considering, for generalization purposes, the first term $((n + 1) - n)^2$,
  which we can develop through the binomial theorem.

  $
    ((n + 1) - n)^2 & = && sum_(k = 0)^(j = 2) binom(j, k) dot (n + 1)^(j - k) dot (-n)^k \
                    & = && 2! / (0! (2 - 0)!) dot (n + 1)^(2 - 0) dot (-n)^0 + \
                    &   && 2! / (1! (2 - 1)!) dot (n + 1)^(2 - 1) dot (-n)^1 + \
                    &   && 2! / (2! (2 - 2)!) dot (n + 1)^(2 - 2) dot (-n)^2 \
                    & = && (n + 1)^2 - 2n(n + 1) + n^2.
  $ <p113-first-binom>

  The first term in the expression can further yield another expansion of the binomial theorem.

  $
    (n + 1)^2 = sum_(k = 0)^(j = 2) binom(j, k) dot n^(j - k) dot 1^k
    = & 2! / (0!(2 - 0)!) dot n^(2 - 0) dot 1 + \
      & 2! / (1!(2 - 1)!) dot n^(2 - 1) dot 1 + \
      & 2! / (2!(2 - 2)!) dot n^(2 - 2) dot 1 \
    = & n^2 + 2n + 1.
  $ <p113-second-binom>

  Thus the complete expression for the first term of the sum proves correct as per the following
  resolution, where @p113-second-binom is plugged into @p113-first-binom.

  $
    n^2 + 2n + 1 - 2n(n + 1) + n^2 = 1.
  $

  The result from @p113-first-binom should be generalizable to any term of the sum, such that

  $
    sum_(i = 1)^(n) i^2 = & ((n + 1)^2 && - 2n(n + 1)       && + n^2)       && + \
                          & ((n + 1)^2 && - 2(n - 1)(n + 1) && + (n - 1)^2) && + dots.c + \
                          & ((n + 1)^2 && - 2(n + 1)        && + 1).        &&
  $

  Beyond this, I'm at a loss. The $(n + 1)^2$ term can be extracted from all expressions into the
  constant $n dot (n^2 + 2n + 1)$ as per @p113-second-binom.

  $
    n dot (n^2 + 2n + 1) = n^3 + 2n^2 + n.
  $

  Which would make the sum evaluate to

  $
    sum_(i = 1)^n i^2 = (n^3 + 2n^2 + n) + & (-2n       & dot & (n + 1) && + n^2)       && + \
                                           & (-2(n - 1) & dot & (n + 1) && + (n - 1)^2) && + dots.c + \
                                           & (-2        & dot & (n + 1) && + 1).        &&
  $

  Upon further observation, we can separate the $-2(n + 1)$ factor as it seems ever present in all
  terms of the sum.

  $
    sum_(i = 1)^n i^2 = (n^3 + 2n^2 + n) - 2(n + 1)(n + (n - 1) + dots.c + 1) + (n^2 + (n - 1)^2 + dots.c + 1).
  $

  This is looking really good. From @p112-basic-sum-c1 and @p112-basic-sum-c2, we know that the
  factor multiplying $-2(n + 1)$ is $(n dot (n + 1)) / 2$, which lets us rewrite the present sum as

  $
    sum_(i = 1)^n i^2 = (n^3 + 2n^2 + n) - (n + 1)(n dot (n + 1)) + (n^2 + (n - 1)^2 + dots.c + 1).
  $ <p113-sum-binom-last>

  The last thing remaining to solve for is the last term of the sum. This seems very much akin to
  the results obtained in @p112-basic-sum-c1 and @p112-basic-sum-c2, but I believe there to be a
  more intricate algebraic manipulation involved, considering each of the subsequent terms to the
  initial $n^2$ are expansions of the binomial theorem.

  Still, something similar should apply to this. Let us evaluate the expansion of the second term,
  namely, $(n - 1)^2$, to see if there is some repeating pattern.

  $
    (n - 1)^2 = sum_(k = 0)^(j = 2) binom(j, k) dot n^(j - k) dot (-1)^k &=&&
    2! / (0!(2 - 0)!) dot n^(2 - 0) dot (-1)^0 + \
    &&& 2! / (1!(2 - 1)!) dot n^(2 - 1) dot (-1)^1 + \
    &&& 2! / (2!(2 - 2)!) dot n^(2 - 2) dot (-1)^2 \
    &=&& n^2 -2n + 1.
  $

  From this result, we know for sure that the first and last terms are the ones yielding $n^2$ and a
  positive $b$, respectively.

  Before going into the term where $k = 1$, I believe we can model the term where $k = 2$ as a
  series ${1, 2, dots.c, n - 1}$. In the context of our sum, this is described by
  $sum_(i = 1)^(n - 1) i$, which itself is equivalent to the sum $(sum_(i = 1)^n i) - 1$. Because we
  already proved in @p112-basic-sum-c1 and @p112-basic-sum-c2 such formula, we can state that the
  sum of all terms $k = 2$ for each and every term of the last term in @p113-sum-binom-last is
  $(n dot (n + 1)) / 2 - 1$.

  We've already figured out the pattern of all binomial expansions of @p113-sum-binom-last for terms
  $k = 0$ and $k = 2$, but let's formalize them before jumping to $k = 1$. In our latest iteration
  on $sum_(i = 1)^n i^2$, the last term contains a single $n^2$ term, followed by the sum of the
  binomial expansions of $(n - l)^2$ where $1 <= l <= n - 1$. We have found that the term $k = 0$ of
  each of those binomial expansions always yields the term $n^2$, so the last term in
  @p113-sum-binom-last contains the initial $n^2$ and $(n - 1) dot n^2$, totaling $n dot n^2 = n^3$.

  Additionally, we have found that the term $k = 2$ for each of those binomial expansions can itself
  be modeled after the formula we proved in @p112-basic-sum-c1 and @p112-basic-sum-c2. This term
  always evaluates to one of $1, dots.c, n - 1$, so the sum for each expansion in
  @p113-sum-binom-last is

  $
    sum_(i = 1)^(n - 1) i = sum_(i = 1)^n (i) - 1 = (n dot (n + 1)) / 2 - 1.
  $

  Thus, so far we have found that the last term in @p113-sum-binom-last evaluates, pending of the
  value for each expansion on $k = 1$, denoted $k_s$, to

  $
    (n^2 + (n - 1)^2 + dots.c + 1) = n^3 + k_s + (n dot (n + 1)) / 2 - 1.
  $ <p113-sum-binom-last-expansion>

  Now, onto finding a pattern for each expansion's $k = 1$ term. As per the binomial theorem,
  whenever $k$ takes on the value of 1, the following expression evaluates, for some $b$ in the
  range $-(n - 1) <= b <= -1$.

  $
    dots.c + underbrace(2! / (1!(2 - 1)!) dot n^(2 - 1), 2n) dot b^1 + dots.c
  $

  Because $b$ is always known to be a negative factor, we can further extract constant components,
  such that the expression evaluates solely to $-2n dot abs(b)$. Because we also know $b$ takes on a
  well--defined range of values, we can model this after a sum.

  $
    -2n dot abs(b) = -2n dot sum_(i = 1)^(n - 1) i = -2n dot (sum_(i = 1)^n (i) - 1) &= -2n dot ((n dot (n + 1)) / 2 - 1) \
    &= -(n^2 dot (n + 1)) + 2n \
    &= -n^3 - n^2 + 2n.
  $ <p113-ks>

  Having found the total sum of each expansion's term for $k = 1$, we can reformulate
  @p113-sum-binom-last-expansion by substituting the $k_s$ term with the result of @p113-ks.

  $
    (n^2 + (n - 1)^2 + dots.c + 1) & = n^3 + (-n^3 - n^2 + 2n) + (n dot (n + 1)) / 2 - 1 \
                                   & = -n^2 + 2n + (n^2 + n) / 2 - 1 \
                                   & = -n^2 / 2 + (5n) / 2 - 1.
  $ <p113-sum-binom-last-final>

  Back to @p113-sum-binom-last, we can now consider substituting in the expression obtained in
  @p113-sum-binom-last-final.

  $
    sum_(i = 1)^n i^2 & = (n^3 + 2n^2 + n)     && - (n + 1)(n dot (n + 1)) && + (-n^2 + 5n - 2) / 2 \
                      & = n^3 + 2n^2 + n       && - (n^3 + 2n^2 + n)       && + (-n^2 + 5n - 2) / 2 \
                      & = (-n^2 + 5n - 2) / 2. &&                          &&
  $

  That doesn't look like the result on Skiena's book. No matter, moving on.

/ Problem 1--14: \
  Now we're proving a very much similar formula to the above, which could benefit from the same
  findings and that will hopefully help me sort out the actual procedure I followed with the prior
  proof.

  The formula in question is

  $
    sum_(i = 1)^n i^3 = (n^2 dot (n + 1)^2) / 4, "for" n >= 0.
  $

  As per the prior proof, we can express this in terms of @p112-basic-sum-c1, such that

  $
    sum_(i = 1)^n i dot i dot i = &(((n + 1) - n&)& &dot& ((n + 1) - n&)& &dot& ((n + 1) - n&)&&)& &&+ \
    &(((n + 1) - (n - 1)&)& &dot& ((n + 1) - (n - 1)&)& &dot& ((n + 1) - (n - 1)&)&&)& &&+ dots.c + \
    &(((n + 1) - 1&)& &dot& ((n + 1) - 1&)& &dot& ((n + 1) - 1&)&&).
  $ <p114-start>

  The pattern follows that each term is equivalent to an expansion of the binomial theorem. A
  consequence of that is that the original sum may be rewritten in terms of another sum.

  $
    sum_(i = 1)^n i^3 = sum_(j = 0)^(n - 1) ((n + 1) - (n - j))^3.
  $

  The expansion of the binomial theorem thus follows, such that we are left with 4 different terms
  participating in the new sum.

  $
    sum_(j = 0)^(n - 1) ((n + 1) - (n - j))^3 &= sum_(j = 0)^(n - 1) (&& sum_(k = 0)^(l = 3) binom(l, k) dot (n + 1)^(l - k) dot (-(n - j))^k) \
    &= sum_(j = 0)^(n - 1) (&& 3! / (0!(3 - 0)!) dot (n + 1)^(3 - 0) dot (-(n - j))^0 + \
      &&& 3! / (1!(3 - 1)!) dot (n + 1)^(3 - 1) dot (-(n - j))^1 + \
      &&& 3! / (2!(3 - 2)!) dot (n + 1)^(3 - 2) dot (-(n - j))^2 + \
      &&& 3! / (3!(3 - 3)!) dot (n + 1)^(3 - 3) dot (-(n - j))^3).
  $

  For the sake of clarity and space, I will treat each individual term of the sum separately.

  $
    sum_(j = 0)^(n - 1) 3! / (0!(3 - 0)!) dot (n + 1)^(3 - 0) dot (j - n)^0 &=
    sum_(j = 0)^(n - 1) (n + 1)^3 \
    &= (n + 1)^3 dot n.
  $

  $
    sum_(j = 0)^(n - 1) 3! / (1!(3 - 1)!) dot (n + 1)^(3 - 1) dot (j - n)^1 &=
    sum_(j = 0)^(n - 1) 3 dot (n + 1)^2 dot (j - n) \
    &= 3 dot (n + 1)^2 dot sum_(j = 0)^(n - 1) j - n \
    &= 3 dot (n + 1)^2 dot ((n dot (n + 1)) / 2 - n - n^2).
  $

  The following terms require another binomial expansion of which, much like the present one, I will
  treat separately on a term--by--term basis.

  $
    sum_(j = 0)^(n - 1) 3! / (2!(3 - 2)!) dot (n + 1)^(3 - 2) dot (j - n)^2 &=
    sum_(j = 0)^(n - 1) 3 dot (n + 1) dot &&(j - n)^2 \
    &= 3 dot (n + 1) dot sum_(j = 0)^(n - 1) &&(j - n)^2 \
    &= 3 dot (n + 1) dot sum_(j = 0)^(n - 1) (&&sum_(k = 0)^(l = 2) binom(l, k) dot j^(l - k) dot (-n)^k) \
    &= 3 dot (n + 1) dot sum_(j = 0)^(n - 1) (&&2! / (0!(2 - 0)!) dot j^(2 - 0) dot (-n)^0 + \
      &&&2! / (1!(2 - 1)!) dot j^(2 - 1) dot (-n)^1 + \
      &&&2! / (2!(2 - 2)!) dot j^(2 - 2) dot (-n)^2).
  $ <p114-binomfirst>

  $
    sum_(j = 0)^(n - 1) 3! / (3!(3 - 3)!) dot (n + 1)^(3 - 3) dot (j - n)^3 &=
    sum_(j = 0)^(n - 1) &&(j - n)^3 \
    &= sum_(j = 0)^(n - 1) (&&sum_(k = 0)^(l = 3) binom(l, k) dot j^(l - k) dot (-n)^k) \
    &= sum_(j = 0)^(n - 1) (&&3! / (0!(3 - 0)!) dot j^(3 - 0) dot (-n)^0 + \
      &&&3! / (1!(3 - 1)!) dot j^(3 - 1) dot (-n)^1 + \
      &&&3! / (2!(3 - 2)!) dot j^(3 - 2) dot (-n)^2 + \
      &&&3! / (3!(3 - 3)!) dot j^(3 - 3) dot (-n)^3).
  $ <p114-binomsecond>

  Following, we resolve each term of the final binomial on @p114-binomfirst, assumming true the
  statement $sum_(i = 1)^n i^2 = (n dot (n + 1) dot (2n + 1)) / 6$.

  $
    sum_(j = 0)^(n - 1) 2! / (0!(2 - 0)!) dot j^(2 - 0) dot (-n)^0 &= sum_(j = 0)^(n - 1) j^2 \
    &= sum_(j = 1)^n (j^2) - n^2 \
    &= (n dot (n + 1) dot (2n + 1)) / 6 - n^2 \
    &= ((n^2 + n) dot (2n + 1) - 6n^2) / 6 \
    &= (2n^3 + n^2 + 2n^2 + n - 6n^2) / 6 \
    &= (2n^3 - 3n^2 + n) / 6.
  $

  $
    sum_(j = 0)^(n - 1) 2! / (1!(2 - 1)!) dot j^(2 - 1) dot (-n)^1 &= sum_(j = 0)^(n - 1) -2n j \
    &= -2n dot ((n dot (n + 1)) / 2 - n) \
    &= -2n dot (n^2 - n) / 2 \
    &= -n^3 + n^2.
  $

  $
    sum_(j = 0)^(n - 1) 2! / (2!(2 - 2)!) dot j^(2 - 2) dot (-n)^2 & = sum_(j = 0)^(n - 1) n^2 \
                                                                   & = n^2 dot sum_(j = 0)^(n - 1) 1 \
                                                                   & = n^3.
  $

  Next, we resolve each term of the final binomial of @p114-binomsecond.

  $
    sum_(j = 0)^(n - 1) 3! / (0!(3 - 0)!) dot j^(3 - 0) dot (-n)^0 = sum_(j = 0)^(n - 1) j^3.
  $ <p114-falsestart>

  Or not. @p114-falsestart can be expressed in terms of the formula we are performing this whole
  proof for, which makes everything we've done until now for this problem useless.

  $
    sum_(j = 0)^(n - 1) j^3 = sum_(j = 1)^(n - 1) j^3 = sum_(j = 1)^n (j^3) - n^3.
  $

  Time to try a different approach. Let's rewind back to the point where we hadn't yet formulated
  the conclusion on this being an instance of the binomial theorem; Namely, let's go back to
  @p114-start.

  No luck solving this. Moving on.

/ Problem 1--15: \
  Another proof, this time of the formula

  $
    sum_(i = 1)^n i(i + 1)(i + 2) = (n(n + 1)(n + 2)(n + 3)) / 4.
  $

  This time we are not told to solve by induction so there may be some alternative way out of this.
  I think can get this somewhere. Let's consider the more simplified form of the initial sum.

  $
    sum_(i = 1)^n i(i + 1)(i + 2) = sum_(i = 1)^n (i^2 + i)(i + 2) = sum_(i = 1)^n i^3 + 3i^2 + 2i.
  $

  From the formulas that were shown to be true on the previous proofs (though not by my hand,) we
  can solve this by separating each term of the sum such that

  $
    sum_(i = 1)^n i^3 + 3i^2 + 2i = sum_(i = 1)^n i^3 + 3 dot sum_(i = 1)^n i^2 + 2 dot sum_(i = 1)^n i.
  $ <p115-initial>

  Now, proceeding as follows with each term separately.

  $
    sum_(i = 1)^n i^3 = (n^2(n + 1)^2) / 4 = (n^2(n^2 + 2n + 1)) / 4 = (n^4 + 2n^3 + n^2) / 4.
  $
  $
    3 dot sum_(i = 1)^n i^2 = 3 dot (n(n + 1)(2n + 1)) / 6 = 3 dot ((n^2 + n)(2n + 1)) / 6 = (2n^3 + 3n^2 + n) / 2.
  $
  $
    2 dot sum_(i = 1)^n i = 2 dot (n(n + 1)) / 2 = n^2 + n.
  $

  Now, putting these all back together into @p115-initial, we get

  $
    sum_(i = 1)^n i^3 + 3i^2 + 2i & = (n^4 + 2n^3 + n^2) / 4 + (2n^3 + 3n^2 + n) / 2 + n^2 + n \
                                  & = (n^4 + 2n^3 + n^2 + 4n^3 + 6n^2 + 2n + 4n^2 + 4n) / 4 \
                                  & = (n^4 + 6n^3 + 11n^2 + 6n) / 4.
  $

  The form we have arrived to is likely the non--factorized version of the result initially provided
  and expected to prove. This implies we can perform some form of factorization to get this sorted
  out, and maybe get to the same result.

  To factorize the polynomial on the numerator, I believe there was some procedure I don't quite
  remember. Time to think. I believe the roots of the polynomial to be
  $(n - 0)(n + 1)(n + 2)(n + 3)$, which aligns with the expected result's numerator.

  Now, technically speaking, this is not completely correct because I used elements of proofs that
  were only stated true for values of $n >= 0$, while this particular proof did not bound the range
  of $n$. Still, moving on.

/ Problem 1--16: \
  We're going back to proofs by induction. This time constrained by some $a != 1$ and $n >= 1$.

  $
    sum_(i = 0)^n a^i = (a^(n + 1) - 1) / (a - 1).
  $

  This is fairly obvious when thinking in binary because one can simply state that
  $mono(1111) = mono(10000) - mono(0001)$. And because $a = 2$ in the above formula for this
  example, the denominator resolves pretty easily.

  The thing here is that, for say 3, there's not much bandwith to work with. All bases above 2 are
  not something I can personally easily compute something in, so there is not much I can say there
  is going into it. Based on what the formula says, there must be a relationship between the
  grouping of numbers making up single digits in some base $a$, and the number of digits that
  correpond in that base to a full power of the base.

  In base 2, such computation resolves to 1. This makes a fair amount of sense becuase there's one
  digit per power and thus one goes from $2^0$ to $2^1$ by toggling the bits at position 0 or those
  at position 1.

  But what about base 3 and larger bases? We know that each power represents a position for bits in
  binary, so technically we should be expecting a similar translation for other bases. The only
  pattern I see here is that for any given base, there exist as many in--between numbers between
  whole powers of such base that is equal to the number denoting the base times the targeted power
  (thus accounting for the numbers that came before it until the immediate prior power) minus one.

  This makes sense for both base 2 and base 3 because we can state that any power in base 2 will
  compute to

  $
    2^0 & =      && 1, \
    2^1 & =      && 2, "and thus there are" 2 dot 1 - 1 = 1 "numbers between this and the prior power", \
    2^2 & =      && 4, "and thus there are" 2 dot 2 - 1 = 3 "numbers between this and the prior power", \
        & dots.c && \
    2^n & =      && k, "and thus there are" 2n - 1 "numbers between this and the prior power".
  $

  If we further generalize this for any base $a$ and try to compute the amount of numbers between
  any power $n$ and the base power $n_0 = 0$, we can technically state that there will be $a^n$
  numbers. If we account also for the number making up the power, there will be $a^n + 1$ numbers.
  Then we also know that out of those $a^n$ numbers coming before it, if we divide them into groups
  of... nothing.

  Moving on.

/ Problem 1--17: \
  Another proof by induction. This time I think I can get through it just fine, though I'm not sure
  if the resulting proof would count as being inductive in nature. Anyway, here goes the formula.

  $
    sum_(i = 1)^n 1 / (i(i + 1)) = n / (n + 1), "for" n >= 1.
  $

  So if we manipulate the statement, we can technically see that there is a binomial expansion with
  negative exponential factor. Considering the binomial theorem applies to any such factor in $NN$,
  we should be capable of reformulating the expression such that

  $
    sum_(i = 1)^n (i^2 + i)^(-1) = sum_(i = 1)^n sum_(k = 0)^(j = -1) binom(j, k) dot (i^2)^(j - k) dot i^k.
  $

  And that is completely wrong, because apparently $NN$ is made out of $ZZ^+$, which doesn't cover
  the $-1$ factor.

  If I think in terms of separating the factors in the denominator, technically we can state that

  $
    sum_(i = 1)^n 1 / i dot 1 / (i + 1).
  $

  Just as $sum_(i = 1)^n i$ is an increasingly larger number that sums the elements from the series
  ${1, 2, dots.c, n}$, $1 / i$ defines the sum of increasingly smaller numbers as each larger value
  of $i$ denotes a smaller subdivision of the unit value. Without offering much proof, we could say
  that the sum of the first $1 / ({1, 2, dots.c, n})$ values would produce an approximation in $RR$
  to $n$.

  That much I can figure from the initial statement, as it shows how the $n$ factor is the
  numerator. Now, if the initial sum were only $sum_(i = 1)^n 1 / i$, the result couldn't have that
  $n / (n + 1)$, so it's quite likely the denominator is given by the second factor; Namely,
  $1 / (i + 1)$.

  Now, the second factor denotes an even smaller number, computing an approximation in $RR$ to
  $n + 1$. This may or may not mean that if we compute the multiplication of each term of the sum,
  as the statement expects, we will obtain a number that is always smaller than $1 / 2$, considering
  the starting value is $i dot (i + 1), "for" i >= 1 "given" n >= 1$.

  What I believe to be the natural conclusion of adding together $1 / i dot 1 / (i + 1)$ and
  $1 / (i + 1) dot 1 / (i + 2)$ is that the number resulting after $n$ sums should be smaller than
  $1 / n$ but still approximate to $n / (n + 1)$, considering the denominator is given by a factor
  larger than any number smaller than $n$.

  Thus, if it is an approximation to $n$ by some factor larger than $n$, then surely the resulting
  sum should be equal to, at the very least, $n + 1$. But this is not an estimation problem; I'm
  wrong.

  No matter, moving on.

/ Problem 1--18: \
  Another proof by induction. This time we ought show that

  $
    n^3 + 2n "is divisible by" 3 "for all" n >= 0.
  $

  Alright, so maybe this can be solved by trying to unwrap the polynomial expression. $n^3$ is
  technically the multiplication $n dot n dot n$, which lets us factor this as

  $
    n dot n dot n + 2n = n dot (n^2 + 2).
  $ <p118-initial>

  When I think of a number being divisible by 3, two things come to mind; solving for a number
  modulo 3 with result 0, and having all digits of the number add up to a smaller, known multiple
  of 3.

  I'm not so sure about the mathematical certainty of the latter statement, so maybe the way to
  solve this is by considering a reduction of $n(n^2 + 2)$ that yields another number we can compute
  modulo 3.

  The base hypothesis is that there ought be some expression $n_0$ whereby $n_0 (mod 3) = 0$.

  Maybe the key here is in distinguishing that some number $n$ may or may not be factorized into
  prime numbers including 3. Then, if $n$ includes the prime factor 3, we may reformulate
  @p118-initial into the following statement.

  $
    n dot n dot (n / 3 dot 3) + 2n, "for any" n "with prime factor 3".
  $

  This could be one case of the recurrence relation, but it's technically not correct, because 3 is
  supposed to be dividing the whole statement, so the above would actually be

  $
    (n dot n dot (n / 3 dot 3)) / 3 + (2n) / 3.
  $

  Or maybe it's correct, because the $n$ factor also participates in $2n$, so the next recursive
  call would be

  $
    (n dot n dot (n / 3)) / 3 + (2 dot n / 3) / 3 = (n dot n dot n) / 9 + (2n) / 9.
  $

  So maybe the recurrence is defined by an ever smaller factor of $1 / {3^1, 3^2, dots.c, 3^k}$
  dividing $n^3 + 2n$ on each recursive call of the $k$ total calls. This would sort of make sense,
  considering one can compute those factors without having $n^3 + 2n$ be divisible by them, in terms
  of $3$.

  $
    n^3 / 9 + (2n) / 9 = n^3 / 3 dot 1 / 3 + (2n) / 3 dot 1 / 3.
  $

  This would end up resolving to, in general terms, the following expression.

  $
    (n^3 / 3 dot 1 / 3 dot dots.c dot 1 / 3) + ((2n) / 3 dot 1 / 3 dot dots.c dot 1 / 3).
  $ <p118-infrecursion>

  Of course, the base case mentioned before for some expression $n_0 (mod 3) = 0$ is always being
  hit, so @p118-infrecursion doesn't indicate the existence of a converging factor $1 / 3$.

  Though maybe the expression to find isn't that for which the initial $n$ is already a multiple of
  3, but rather that where $n$ is not a multiple of 3. In that case, we should state that

  $
    n (mod 3) != 0 "so" n^3 / 3 + (2n) / 3 (mod 3) = 0.
  $

  This doesn't hold at all, because I'm assumming the $n$ term is the central term for which the
  search for an Euclidean division with 3 should produce residue 0. That could or could not be the
  case, considering the problem statement only points towards the whole expression being divisible
  by 3.

  Maybe the fact that the whole expression is divisible by 3 is truly the base case of the
  induction. That could make sense if what we were initially trying to check for was if
  $n (mod 3) = 0$. But then again, this is not necessarily the thing we're trying to prove.

  Or maybe the whole thing is wrong, and I'm actually supposed to compute for

  $
    ((1 dot 2 dot dots.c dot n_p) dot (1 dot 2 dot dots.c dot n_p) dot (1 dot 2 dot dots.c dot n_p)) / 3 + (2 dot (1 dot 2 dot dots.c dot n_p)) / 3.
  $

  This considers the equivalent expression to the initial statement, only decomposing $n$ into each
  of its prime factors. But this doesn't get me anywhere, because I was expecting I could then
  factor out one of those prime factors and hope for the denominator to cancel out. That's not
  possible, because it would require that #l-enum[$n$ have more than a single prime factor, and][the
    combination of some prime factors in $n$ produce a 3].

  For the case where $n = 0$, the above conditions always hold true, because we can simply assume
  that all prime numbers can divide 0, and the result such Euclidean division will always have
  residue 0. Then for $n = 0$, can consider those inner multiplications as containing 3, and thus,
  they can factor out of both main terms of the expression, namely $n^3$ and $2n$, to continue
  producing $n^3 + 2n, "for" n = 0$.

  No matter, moving on.

/ Problem 1--19: \
  Another proof by induction, whereby I'm expected to show that a tree with $n$ vertices and $m$
  edges, has $m = n - 1$.

  Considering the definition of a tree is that of a graph $G = (V, E)$ where $|E| = |V| - 1$, and
  where the graph is both connected and acyclic, these two latter statements force an implication
  over the number of edges in the total degree of each node. Given a vertex $i in V$, this node may
  only have upwards of $n - 1$ edges, such that assumming no cycle is allowed (and trees are not
  multigraphs,) such vertex may only connect to every other vertex, the total of which is denoted as
  $|V - {i}| = |V| - 1$.

  For some other vertex $j in V$, if the edge $(i, j) in E$ already exists, as per the prior
  statements, one can only consider the existence of that single edge, namely $(i, j) in E$, because
  otherwise the graph would have a cycle from edge
  $(j, k) in E "for some" k in V "where" (i, k) in E$. Assumming as well that no self--loops are
  allowed, the only edge is that which was already considered from node $i$.

  And I think this is a good enough proof.

/ Problem 1--20: \
  The last proof by induction of this chapter. I must show that the following statement holds true.

  $
    sum_(i = 1)^n i^3 = (sum_(i = 1)^n i)^2.
  $ <p120-initial>

  Which goes to say that the sum of the cubes of the first $n in NN$ numbers is equal to squaring
  the total sum of those first $n$ numbers.

  Assumming true the formulas that each one of #l-enum[$sum_(i = 1)^n i^3$, and][$sum_(i = 1)^n i$]
  expand to, I think this should be fairly simple to prove. But the resulting conclusion is likely
  not going to be inductive in nature.

  $
    (sum_(i = 1)^n i)^2 = ((n(n + 1)) / 2)^2 = (n(n + 1))^2 / 2^2 = (n^2(n + 1)^2) / 4 = sum_(i = 1)^n i^3.
  $

  This is by no means a proof, so I must think further.

  If we think about what the left--hand side of @p120-initial expands to, maybe we can find a
  pattern.

  $
    sum_(i = 1)^n i^3 = (1 dot 1 dot 1) + (2 dot 2 dot 2) + dots.c + (n dot n dot n).
  $

  Upon factoring one term in each of those cubes, one can see that the resulting expression can be
  further manipulated into the form $(1 + 2 + dots.c + n)(1^2 + 2^2 + dots.c + n^2)$.

  $
    (1 dot 1 dot 1) + (2 dot 2 dot 2) + dots.c + (n dot n dot n) = 1(1 dot 1) + 2(2 dot 2) + dots.c + n(n dot n).
  $

  Or not. But maybe it can expand to

  $
    1(1 dot 1) + 2(2 dot 2) + dots.c + n(n dot n) & = && 1(1 dot 1) + (1 + 1)(2 dot 2) + dots.c + \
    & && (1 + 1 + dots.c + 1_n)(n dot n) \
    & = && (sum_(j = 1)^1 1)(1 dot 1) + (sum_(j = 1)^2 1)(2 dot 2) + dots.c + \
    & && (sum_(j = 1)^n 1)(n dot n).
  $

  This implies the left--hand side of @p120-initial may be rewritten as

  $
    sum_(i = 1)^n i^3 = sum_(i = 1)^n i^2 dot sum_(j = 1)^i 1.
  $

  Still, this doesn't provide much value, because it's obvious from the following statement.

  $
    sum_(i = 1)^n i^3 = sum_(i = 1)^n i dot i dot i = sum_(i = 1)^n sum_(j = 1)^i 1 dot sum_(j = 1)^i 1 dot sum_(j = 1)^i 1.
  $

  Still, this is meant to be proven by induction. So there must be a way of considering, on a
  term--by--term basis, that the resulting relationship, namely the right--hand side of
  @p120-initial, holds true.

  So let's think smaller. Let's consider only the term for which $i = 1$, which is ever present,
  considering the sum starts there, unless $n = 0$ and we assume that $0 in NN$.

  $
    1 dot 1 dot 1 = 1 dot 1.
  $

  What about $i = 1, 2, 3, 4$?

  $
    & (1 dot 1 dot 1) + (2 dot 2 dot 2) && = (1 + 2) & dot & (1 + 2). \
    & (1 dot 1 dot 1) + (2 dot 2 dot 2) + (3 dot 3 dot 3) && = (1 + 2 + 3) & dot & (1 + 2 + 3) \
    & (1 dot 1 dot 1) + (2 dot 2 dot 2) + (3 dot 3 dot 3) + (4 dot 4 dot 4) && = (1 + 2 + 3 + 4) & dot & (1 + 2 + 3 + 4).
  $

  Maybe we can start thinking from the right--hand side of @p120-initial instead. Let's take
  $n = 2$.

  $
    (1 + 2) dot (1 + 2) = (1 dot 1) + (1 dot 2) + (2 dot 1) + (2 dot 2).
  $

  No matter, moving on.

/ Problem 1--21: \
  I'm asked about the total number of pages in the books I own and whether that number is around 1
  million pages. Then I'm also asked to estimate whether the total number of pages in my school
  library is also around that number.

  I am completely confident that I don't own enough books to total 1 million pages, based on the
  fact I own less than 100 books and assumming most of them are below 1000 pages, they don't even
  get to $10^2 dot 10^3 = 10^5$ pages, which would be the bare minimum for this estimate.

  In my school library, this would be hard to estimate, considering I've never been inside of it.
  Still, I know there's three floors to it, each of about $150 space "m"^2$. I'll try to estimate
  the number of bookshelves in each floor, prior to the number of books in each bookshelf and
  finally the number of pages per book on average (aiming for an upper bound on all three
  heuristics.)

  The floors are about $150 space "m"^2$, based on the fact they're not exactly twice as big as my
  house but definitely near that (my house is $91 space "m"^2$.) In each floor, let's assume there
  are bookshelves all around its perimeter, and some throughout the inner area. Let's proceed first
  with the bookshelves in the perimeter.

  Based on observation, I'd wager the floors are pretty near the shape of a rectangle, which would
  imply its sides are approximately $150 / 10 = 15 space "m" times 10 space "m"$. I would say a
  bookshelf takes up about $5 space "m"$ in length, and I believe its width to be of about
  $40 "cm"$. Based on these estimates, the shorter sides of each floor should have $10 / 5 = 2$
  bookshelves, while the longer sides should have $15 / 5 = 3$ bookshelves. This would total
  $3 + 2 = 5 dot 3 = 15$ bookshelves from the perimeter of all floors.

  Let's compute now an approximate over the total number of bookshelves in the inner area of the
  rectangle. In what I consider to be a standard middle bookshelf layout for a library, each shelf
  is set up horizontally, such that each side of it (considering the longest stride as the _side_;
  Its length) should face the shorter sides of the overarching rectangle. This means that if the
  width of each bookshelf is about $40 "cm"$ and the corridor between bookshelves is of about
  $3 space "m"$, there must be $(15 space "m") / (3 + 0.4 space "m") approx 4$ bookshelves in the
  inner area of each floor. This totals $4 dot 3 = 12$ bookshelves in the inner area of the whole
  library.

  The total number of bookshelves in the library is now at $15 + 12 = 27$ bookshelves. Assumming
  each bookshelf is about $4 space "m"$ tall considering each floor is about $7 space "m"$ tall, and
  each shelf's divided into about $40 "cm"$ tall levels, there's space for
  $(4 space "m") / (0.4 space "m") = 10$ book levels per shelf.

  If each book is, on average, $5 "cm"$ wide and we considered the length of the shelves to be
  standing at $5 space "m"$, then each level can hold an estimate of
  $(5 space "m") / (0.05 space "m") = 100$ books.

  Thus each shelf holds $10 "levels" dot 100 space "books"/"level" = 1000$ books. Accounting for
  each of the 27 shelves we computed before, this makes up
  $27 "shelves" dot 1000 space "books"/"shelf" = 27000$ books in the entire library.

  Assumming each book is, on average, between 500 and 1000 pages, so about $(1000 + 500) / 2 = 750$
  pages long, then the total number of pages ought be $27000 dot 750 approx 20$ million pages.
  Accounting for books that are less than the lower end of the average (i.e. less than 500 pages,)
  this would still make for a number well above 1 million pages.

/ Problem 1--22: \
  This one asks about the amount of words on Skiena's book.

  From looking at one regular (non--problem--full page,) I'd wager the font is a Computer Modern at
  $12 "pt"$ and the page size is a US Letter. Based on personal experience, I'd estimate there are
  about 350 to 400 words per page. Considering the first part of the book stands at about 430 pages,
  as that is the part of the book for which I'm considering this first approximation, there ought be
  about $430 dot 375 approx 160 000$ words in the first part of the book.

  The second part is actually more text heavy, even if we consider the use of illustrations on the
  front page of each algorithm or data structure presented, but it should still put the estimate to
  slightly above 375, likely totaling 400 full words per page. Not accounting for the bibliography
  nor index, the second part spans $718 - 435 = #(718 - 435)$ pages long, which computes
  $#(718 - 435) dot 400 approx 110 000$ words.

  Thus my total estimate is of $160 000 + 110 000 = 270 000$ words in Skiena's book.

/ Problem 1--23: \
  I am to estimate the number of hours in 1 million seconds, as well as possibly the number of days.
  Because the problem explicitly states that the arithmetic should be performed on one's own head,
  I'll just list the result I considered.

  In terms of hours, about 250 hours. In terms of days, about 10 days.

/ Problem 1--24: \
  I'm expected to compute the number of cities and towns in the whole of the USoA.

  I barely know the population distribution in the US, so I'm going to throw some wild guesses based
  on the trends I've observed from other first--world, western countries.

  The population centers are centered about the coasts, and I recall there being a bunch of names
  especially in the sides of the country that went across a large area of those regions. Dare I say,
  it quite possibly totals 100 cities and towns per coast, assumming the "coast" extends to the
  border with Mexico. This gets us to 200 cities and towns.

  Then, I'd say there's likely a progressive decrease (i.e. not immediate) of the number of towns as
  you move towards the middle area of the country. I'm going to model this after the theoretical
  area of a circle, where I consider 5 circumcentric rings, the first of which I already covered
  from the coast. The distance between each conceptual ring should map to the decrease in population
  from the area covered by one ring, itself measured as the area between this ring and the next one.

  I'm going to assume the decrease is of 10 cities and towns per ring, which should make for the
  following series.

  $
    {100, 90, 80, 70, 60}.
  $

  Considering then that we are to double each to account for each side of the country, I'd wager
  there are about

  $
    100 dot 2 + 90 dot 2 + 80 dot 2 + 70 dot 2 + 60 dot 2 = 800 "cities and towns in the US".
  $

/ Problem 1--25: \
  I am to estimate the number of cubic miles of water that flow out of the Mississipi River on a
  daily basis.

  I don't have the first idea of how wide that river is, but I've heard of it and there must be a
  reason why the author is using a sort--of well--known river in his book, so I'm going to assume
  it's fairly thick; Let's go for $2 "km"$ wide.

  Based on the same fact, I'd guess that river is likely fairly long, which in US terms possibly
  means it crosses 3 to 4 different, and big, states. If I define "big state" as a state of about
  $1000 "km"$ cross--section, then if the river were to cross 3 of these, it should be about
  $3 dot 1000 = 3000 "km"$ long.

  With a width of $2 "km"$ and a length of $3000 "km"$, and considering $1 "mi" approx 1.25 "km"$,
  then there ought be at least $2 "km" dot 0.5 "km" dot 3000 "km" = 3000 space "km"^3$ of water
  flowing out of it at any given time (the $0.5 "km"$ accounts for an estimate on depth.) In miles,
  these would be about $3000 space "km"^3 dot (1 "mi")^3 / (1.25 "km")^3 approx 1500 space "mi"^3$
  of water flowing out of the river at any given time.

  In a single day, I'd say it's fair to say this happens almost on a second basis, so there should
  be about
  $(1500 space "mi"^3) / (1.5 space "s") times 24 space "h" dot 3.6 times 10^2 space "s" approx 8 times 10^6 space "mi"^3$
  flowing out of the Mississipi River each day.

/ Problem 1--26: \
  Now I'm expected to estimate the amount of Starbucks and McDonald's in my country.

  Based on the fact my country only happens to have those in the "main cities" of each locality,
  there ought be about 20 between both franchises on each of the about 15 main cities around my
  country, so this goes for $20 dot 15 = 300$ Starbucks and McDonald's in my country.

/ Problem 1--27: \
  Now I need to compute the amount of time it would take to empty a bathhub with a drinking straw.

  Assumming the bathhub is full, and it can hold about $30 space "l"^3$, and assumming as well that
  the straw can suck in in $(0.005 space "l"^3) / (1 "s")$, then it will take about

  $
    30 space "l"^3 times (1 "s") / (5 times 10^(-3) space "l"^3) = 6000 "s to empty the whole bathhub".
  $

  So a bit less than 2 hours.

/ Problem 1--30: \
  I ought implement the #smallcaps[TSP] heuristics mentioned in the chapter and determine which of
  them is more performant. I should also try to think of a better solution if I can find one off the
  top of my head.

  The problem treated in the section pointed to by the problem is a symmetric instance of the
  #smallcaps[TSP], where a robot arm ought go through a set of points, starting from some point $a$
  and ending at the same point $a$ while taking as little time as possible in taking the tour across
  the rest of the locations.

  This can be modeled after a complete, weighted graph $G = (V, E)$, where each edge $(i, j) in E$
  considers the distance between its connecting vertices.

  The first heuristic mentioned in the book is that of the _nearest--neighbor_. This should prove to
  be the simplest to implement, even though it is far from finding the optimal path as it considers
  the nearest vertex to the one currently considered in a loop. This implies the path is not taken
  into consideration, because only the distance of every other vertex to the "current" vertex is
  regarded when taking the decision to move to the next edge. Adding up each of these distances, the
  total path ends up being much larger than it needs to be.

  Using an adjacency matrix for the graph DS, the nearest--neighbor heuristic would require keeping
  a list of all the vertices that have not yet been visited, while selecting some vertex from the
  matrix. Because the heuristic doesn't specify that the selected vertex be random, we can simply
  pick the first vertex.

  Then for each vertex in the matrix that is not yet marked visited in the tracking list, we ought
  select the closest, unvisited vertex to the initially selected one. Following, we repeat the same
  process, only this time using the latest visited vertex as the one to consider in search of its
  closest vertex. We determine distance as a function of the weight of the graph edges.

  Once we hit the point where no vertex remains unvisited, the cycle has been completed. To denote
  this, we may add back to the tracking list the first edge that we visited, that is to say, the
  very first vertex in our adjacency matrix, as per the selection criteria mentioned before.

  In Rust, we can keep track of an adjacency matrix DS with a custom type holding a vector of
  vectors of another custom type for edges. The edge type must consider one of two possible states;
  Weighted edges and nonexistent edges. The nature of the problem follows that no self--loops are
  allowed, and the graph is complete because there's implicit edges between any nodes. By the
  problem description, it also follows that all edges are attributed a weight reflecting the
  distance between the connecting vertices.

  This may be modeled in the constructor of the matrix DS through linear--cost operations that check
  for the matrix to #l-enum[be square (i.e. it's a graph proper)][have a main diagonal made out of
    nonexistent edge variants (i.e. it's got no self--loops,) and][have the values below and above
    the main diagonal be equal (i.e. the graph is undirected.)]

  The pattern to check for when considering whether the elements of each row are nonexistent or
  weighted edges should follow that only those edges denoted by indices that happen to be the same
  as the current row's index in the overarching matrix are nonexistent, while all others are
  weighted.

  Let me get some form of pseudocode out for what I understand the nearest--neighbor heuristic to
  be.

  #pseudocode(title: smallcaps(all: false)[Nearest--Neighbor($G$)])[
    + $"visited" <- emptyset$
    + *for* $v$ *in* $V, "where" G = (V, E)$ *do*
      + $"visited" <- "visited" union {0}$
    + $v <- V_0, "where" G = (V, E) "and" V = {V_0, V_1, dots.c, V_(abs(V) - 1)}$
    + $"output" <- emptyset$
    + *while* $a in "visited" : exists a [a != 1]$ *do*
      + $"visited"[v] <- 1$
      + $"output" <- "output" union {v}$
      + $d_"min" <- oo$
      + $v_"next" <- v$
      + *for* $b$ *in* $V_v, "where" G = (V, E) "and" V_v = V inter {e in "visited" : e != 1}$ *do*
        + *if* $(v, b) in E, "where" G = (V, E) : (v, b) < d_"min"$ *do*
          + $d_"min" <- (v, b)$
          + $v_"next" <- b$
      + $v <- v_"next"$
    + $"output" <- "output" union {V_0}, "where" G = (V, E) "and" V = {V_0, V_1, dots.c, V_(abs(V) - 1)}$
    + *return* $"output"$
  ]
