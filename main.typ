#import "@local/typst-template:0.28.0": *

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
  considers the distance between its connecting vertices, namely $i, j$.

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
      + *for* $b$ *in*
        $V_v, "where" G = (V, E) "and" V_v = V inter {c_i in "visited", c in V : c_i != 1}$ *do*
        + *if* $(v, b) in E, "where" G = (V, E) : d_((v, b)) < d_"min"$ *do*
          + $d_"min" <- d_((v, b))$
          + $v_"next" <- b$
      + $v <- v_"next"$
    + $"output" <- "output" union {V_0},
      "where" G = (V, E) "and" V = {V_0, V_1, dots.c, V_(abs(V) - 1)}$
    + *return* $"output"$
  ]

  In Rust, modeling the inner `for` loop requires performing a check over the tracking list, which
  implies this tracking list contains as many elements as there are vertices in the graph. In Rust,
  I can think of only selecting from the iterator of elements of the current graph the one we
  currently are processing, followed by filtering from its list of edges in the adjacency matrix
  those elements whose indices coincide with the indices of the elements that are marked visited in
  the tracking list.

  Maybe the nearest neighbor heuristic can be optimized in the Rust code, such that instead of
  performing a full check of all elements of the adjacency matrix on every loop iteration, the
  iterator over the tracking list skips the first $n$ elements that are known to have already been
  checked. Or not, because it could very well be that the next node in the path is not the one
  immediately "following" (in terms of 0--indexed vertex identifiers) the one we just processed.

  The next heuristic is based on the same concept as the union--find DS, as it uses a closest pair
  approach whereby we initially consider a forest of single--vertex trees, each representing one of
  the vertices/locations in the graph. For each one of those initial trees minus 1, the algorithm
  goes through all separate trees and considers the edge between vertices of differing trees that
  has the smallest separating distance (i.e. the lightest edge.)

  Implementing this is likely going to require building an auxiliary DS for the union--find data
  structure that is customized to the needs of this problem. This context has the particularity that
  each of the trees needs to additionally support an operation for traversal of the nodes in each
  tree of the forest/disjoint set, but doesn't require keeping track of the tree height because path
  compression would ruin the whole algorithm. For this, I belive the best approach is going to be
  implementing an iterator with special properties.

  The iterator should consider each of the trees, and for each node in the tree, it should produce a
  2--tuple `Some((i, j))` where $mono(i) := T_0, mono(j) := T_1$. This means the iterator needs to
  keep track of the current tree being explored, the node of the current tree being considered and
  additionally, it will require knowing how many and which trees are left, as well as their
  component nodes.

  To support this, the design of the DS will need to keep track of #l-enum[an identifier assigned to
    each of the vertices][an array modeled after a backward--edge parent--tree, and][basic UFDS
    operations (`unite`, `find`, `same`.)]

  The first requirement is simple enough; We follow the same reasoning as with the nearest neighbor
  heuristic and use the indices of the vertices in the adjacency matrix as the numerical
  identifiers. The second requirement should be enough to leave the heavy--lifting traversal logic
  to the iterator, which itself is going to use the basic operations in abundance.

  To allow for less computations to be performed on each call to `next()`, the iterator should also
  keep internal state to be initialized with the call to `iter()`. This should contain a record of
  the total number of trees in the forest, a collection of indices for the representative vertices
  of each tree (thus the length of this should provide for the fomer,) and the index of the
  currently considered node.

  Because all elements to be iterated over are known the moment the `iter()` call is made, the
  iterator can precompute, on a per--node basis, the Cartesian product of

  $
    {a} times {b, c, dots.c, n},
    "where" a in T_0, {b, c, dots.c, n} in T_1 union T_2 union dots.c union T_n.
  $

  To compute the Cartesian product, I have not found in my bibliographic sources an efficient
  method, so I will proceed with a manual implementation. The `BTreeSet` in the Rust `std` library
  does not implement such an operation, but perhaps I can use the `intersection()` method on that
  type to produce the rhs of the above Cartesian product.

  Nay. The best way is going to be taking into consideration the current node being iterated over,
  and then manually computing the Cartesian product with each of the nodes (not just the
  representative nodes) the iterator is keeping tabs on.

  The iterator implementation is done. It's not tested, though. The next step is going to be
  deciding whether I should override the implementation of `min()` in `Iterator` to provide perfect
  semantics with the context in question (i.e. the ordered pair denotes an edge and thus the
  ordering is denoted by the weight of such edge, not by the node indices themselves.) The
  alternative to this would be to, once I'm actually solving the problem, call `min_by_key()` and
  pass a closure that transforms each element into the weight denoted by the edge (which could also
  be done in a similar fashion with `min_by()`.)

  The thing here is that the semantics of the `Pairs` iterator would be wrong if the `min()` method
  weren't overridden. Sure `min()` just calls `min_by()` with the standard `Ord` implementation of
  the iterated--over `Item`, but this is no excuse for not overriding the implementation. We have an
  answer.

  To override `min()`, I believe the implementation should transform the iterated sequence in the
  same vein as `min_by_key()`'s parameterized closure, after which a regular `min()` may be called.
  The only issue here is that `min()` would force a reduction, which itself forces a fold, which
  itself forces complete consumption of the iterator. In and of itself, this is no issue,
  considering this is a `self`--owning method in `Iterator`, but the way the iterator is built, this
  would make it so that `Pairs` would have to compute the complete set of... nothing.

  It's actually pretty simple. I need only call `min_by_key()` with a closure transforming the pairs
  into the equivalent edge weight in the adjacency matrix, after which I can either #l-enum[derive
    an implementation of `Ord` for `Edge`, or][further destructure the `Edge` enum into the
    underlying weight].

  We're going with the second option.

  None of the `min_*` operations yield the right semantics, as they all consume the iterator, and
  thus compute in the process all cartesian products, selecting always the lightest edge of the
  graph after already having formed the smallest forest of disjoint chains (i.e. a forest of two
  trees.) There's two alternatives: #l-enum[Continue overriding the `min` implementation of
    `Iterator` with the right collection of values, or][Implement a method on the `Pairs` type with
    the same logic but fully correct semantics (such that no cloning is involved in the process of
    computing the minimum weight edge in the #smallcaps[TSP] algorithm in which it is used.)]

  Embedding the right semantics for unordered pairs of edges in the graph within the `min()` method
  of `Iterator`, such that it doesn't consume the whole iterator, is hard. The trait method
  signature cannot be overridden, so even if the iterator performed the correct set of operations,
  the mere call to the consumer method would force a move on the `Pairs` instance we intended on
  keeping valid throughout the whole #smallcaps[TSP] algorithm implementation. The only real option
  is using a method on the type, such that it makes use of a mutable receiver and allows for
  consistent mutation through select calls to the `Iterator` trait methods to advance the Cartesian
  product pair so long as the next call to the iterator does not yield a Cartesian product where the
  first element is different from the `self.current_node` taking up the lhs of said operation.

  An alternative to the method implementation would be to implement `min()` as `Iterator` under the
  assumption that the method will be called after `by_ref()`, which should yield a mutable reference
  that would advance the iterator without completely consuming it. Basically, the `min()`
  implementation would be one where it is assumed that the minimum value is only to be considered
  within the range of the `self.current_product`. Then, on the call site within the `tsp()` method,
  the `Pairs` instance would call first, on each iteration, the `by_ref()` method from `Iterator`,
  after which `min()` would both safely provide the minimum value in the currently considered
  Cartesian product, and allow for the original iterator to be reused in the next iteration of the
  overarching loop over $n - 1$ nodes of the graph.

  The implementation for `min()` should now be modified from the original use of `min_by_key()`, as
  all logic pre--implemented on the iterator is not going to stop at the end of the current
  Cartesian product. The implementation should follow that the `min()` method would call `next()`
  for as long as the internal state on `Pairs` yields the same value on the `current_node` field.
  For that, an infinite loop over `next()` calls, checking everytime that `self.currrent_node`
  hasn't changed, while collecting all yielded values in a vector, should do just fine. Then we can
  call `min_by_key()` on an interator over the vector such that we first transform each pair into
  the corresponding weighted edge in the graph (including an `unreachable!()` macro call for the
  case of an `Edge::None` variant,) and let this method do its thing with the `Ord` trait
  implementation on the underlying weight (the `usize` field within the `Edge` tuple variant.)

  Maybe the whole implementation is wrong. Revisiting Skiena's book, it does mention that the
  algorithm checks for the smallest weight among all pairs made out of vertices of differing chains.
  This implies the `min()` operation ought act on all computed Cartesian products, which itself
  means that the iterator should be completely iterated over but not consumed. This is so that after
  finding the minimum weight edge at the end of a full iteration, we may use the `unite()` operation
  on the UFDS--like DS inside `Pairs` to update the forest of disjoint vertex chains. So this still
  means the approach with `by_ref()` is correct, but the more complex implementation of `min()` is
  not. Though this does imply a fully consuming operation with `min_by_key()` is possible, and very
  much simpler than the current implemenation with `min()`. If anything, the same operation may be
  reimplemented as it was before.

  The thing to consider here is that the `Pairs` iterator ought be reset to a pre--iteration state
  on all fields but `chains`, which keeps track of the vertex chain across #smallcaps[TSP] algorithm
  iterations. More specifically, the iterator must call `min()` inside the hot loop of `tsp()`, then
  `unite()` with the returned 2--tuple, and then it must reset all fields but `chains` prior to
  continuing with the next iteration of said loop. The `output` auxiliary vector on the `tsp()`
  implementation should not be required once the forest in `pairs_iter` is made out of only a single
  vertex chain. Still, at the end of `tsp()` there's no indication of which node in field `chains`
  is the "leaf" of the chain, and for that matter, there's no invariant that holds between non--leaf
  nodes and branch nodes. The only invariant is that of the root node, which will refer to itself.
  So the auxiliary variable is necessary.

  So the implementation is done, but it may be faulty in some respects. Considering the simple
  adjacency matrix $M_1$ as follows,

  $
    M_1 = mat(
      -1, 1, 3;
      1, -1, 4;
      3, 4, -1;
    ), "where negative weights denote nonexistent edges".
  $

  This should yield the 0--indexed--based #smallcaps[TSP] ${0, 1, 2, 0}$.

  I've stumbled upon what seems like an issue in the way Rust is considering overridden
  implementations of trait methods after calling a non--overridden implementation of some other
  (same) trait method, both outfit with a default implementation in the `trait` block. The call
  chain in question follows that for some trait `E` implemented on a type `T`, where a method `a`
  with a by--value, consuming receiver `self`, is overridden, such overridden method is _not_
  correctly handled when called after another non--overidden method returning a `&mut self` to the
  type `T`.

  The most natural way for the issue to arise is by a (non--overridden) call to `by_ref()` on a type
  `T` implementing the `Iterator` trait, followed by an overridden call to some method `a()` taking
  in as a receiver a non--mutable, consuming, `self`. Instead of having the overriden method `a()`
  act on the mutable reference returned by `by_ref()`, thus still changing the iterator's state but
  not consuming it, the call is performed to the default implementation in `Iterator` of method
  `a()` (the compiler does not complain and the reference doesn't seem to speak of any issues with
  this.)

  Strangely enough, upon using fully qualified syntax, the compiler _does_ complain about the fact
  that the call to the overridden method `a()` on the trait expects a consuming `self`, and not a
  `&mut self`. For reference, following I include a "generic" MWE using fully qualified syntax, and
  comments on the experimental results obtained while running latest stable Rust (1.92.0 as of this
  writing) under AArch64, Apple Silicon.

  ```
  // This complains about method `a()` expecting a `self` receiver.
  let _ = <T as Iterator>::a(<T as Iterator>::by_ref());

  // For some instance of type `T: Iterator`,
  let t = T::new();
  // this does not complain at all, but the call to `a()` does not resolve to
  // the overridden implementation.
  let _ = t.by_ref().a();
  ```

  For a real MWE, consider the `min()` method on the `Iterator` trait, as well as the `by_ref()`
  method on the same trait. Assumming that `by_ref()` has not been overridden, but `min()` _has_
  been overridden, the following iterator call chain does not resolve to the overridden
  implementation of `min()`, but rather to the default implementation under module `std::iter`.

  ```
  // For some instance of type `T: Iterator`,
  let t = T::new();
  // the following call does not resolve to the overridden implementation of
  // `min()`, but to the default implementation in crate `std`.
  let _ = t.by_ref().min();
  ```

  A temporary solution on the Rust user's end would be to create a new method on the type `T`'s own
  `impl` block with slightly modified semantics from those of the overridden `a()` method of trait
  `Iterator` (this trait specifically as I further comment on `by_ref()`.) Instead of taking in a
  `self` receiver, it should take in a `&mut self` receiver, as otherwise the compiler seems to
  assume that even after a call to `by_ref()`, the source pointee of the mutable reference yield
  will still be consumed. As a consequence, `by_ref()` may as well be removed from the iterator call
  chain.

  *Note: _the following comments use the word_ permutation _quite loosely to describe the result of
  a combinatorial process whereby a set of integer numbers is mapped to an equivalent--length set of
  Cartesian products where each one of those numbers is the lhs of such operation, and an improper
  subset comprising the complement of the intersection of the prior singleton set with the original
  set makes up the rhs._*

  After having finished the implementation of the the two (feasible) #smallcaps[TSP] heuristics in
  Section 1.1 of the book, I can not say which is more efficient based on benchmarks, as I happen to
  have found what seems like a compiler error in my langauge of choice when implementing the second
  heuristic.

  But purely out of manual, theoretical (and personally primitive) algorithm time complexity
  analysis, my implementation of the nearest neighbor heuristic seems to run in
  $Theta(n^3), "where" n "is the number of points to tour through"$. This is due to the fact the
  procedure needs to visit all nodes prior to considering that they have all been visited, which
  already incurs a fixed cost of $n$. Upon checking that some node is, indeed, yet to be visited,
  two more linear operations on the number of nodes $n$ (assumming the problem is modeled after a
  complete, simple, weighted, embedded graph) would be required to #l-enum[check which edges of the
    vertex considered in the current iteration are weighted _and_ haven't yet been visited, as well
    as][check which of those edges yields the lightest weight (distance.)]

  Note that the performance could have been improved to $Theta(n^2)$ had the condition of unvisited
  vertices not been imposed on each subsequent iteration, as that way the traversal required to find
  the smallest weight would have only forced a single factor of the linear cost incurred on the
  number of nodes. Unfortunately, without making use of some other auxiliary data structure (like a
  binary heap) to compute the minimum element in less than linear time, each iteration must
  repeatedly consider which nodes have not yet been visited, and must subsequently traverse the
  edges of such a list of nodes to find the one arc of lightest weight.

  The closest pair heuristic yields a performance that is, yet again, initially linear over the
  number of nodes $n$ as only $n - 1$ fixed iterations are performed. For each of these iterations,
  the algorithm runs multiple UFDS sublinear cost operations to try to amortize the costly
  computation of having a disjoint set of vertices constantly permuted. More specifically, each
  iteration considers a persistent forest of trees but does not optimize through path compression as
  that would go against the nature of the problem. Because each iteration requires finding the
  smallest edge out of all edges (thus considering the permutation of all Cartesian products where
  the lhs is the permuted vertex in question, and the rhs is each of the vertices resulting from the
  union of all trees (in the overarching disjoint set) other than the one in which the current node
  is found at,) the resulting cost in a single iteration of the linear loop is dependent of the
  number of current trees in the forest.

  This behavior is completely deterministic in nature, as each iteration is assured to decrease the
  number of disjoint trees in the forest by exactly 1. Thus the total cost upon exitting the loop
  can be modeled as follows on any run of the algorithm so long as the precondition on the type of
  graph used is upkept (which always holds true if the problem is a symmetric instance of the
  #smallcaps[TSP].)

  $
    underbrace(sum_(i = 1)^(n - 1), "loop")
    (overbrace(n, "min"#repr([--])"finding operation") dot
      underbrace((n dot k), "permutation of Cartesian products")).
  $ <p130-initialformula>

  The actual cost $k$ of a single Cartesian product in the above formula should be $n - 1$ on the
  first iteration, but on subsequent iterations should become $n - i$ only for those nodes now
  contained within the same tree (i.e. whichever two nodes were UFDS--`unite`d at the end of the
  prior iteration upon computing the minimum value of all Cartesian products.) This, though, is
  still deterministic in nature; At any given iteration one should expect a cost of $i dot (n - i)$
  for the trees that are not in the same node, and $(n - i) dot n$ for whichever trees are still
  only made out of single--vertex roots.

  This would model the $k$--term of @p130-initialformula as follows.

  $
    &k = i dot (n - i) + (n - i) dot n, "such that" \
    &sum_(i = 1)^(n - 1) n dot (n dot k) =
    sum_(i = 1)^(n - 1) n dot (n dot (i dot (n - i) + (n - i) dot n)) && = \
    &&& = sum_(i = 1)^(n - 1) n dot (n dot ((n - i) dot (n + i))) \
    &&& = sum_(i = 1)^(n - 1) n dot (n dot (n^2 - i^2)).
  $ <p130-secondformula>

  This is incorrect. The behavior in the first iteration is flawed, as
  $n^2 - i^2 != (n - i) dot n, "for some" n, "and" i = 1$. Still, because this is not an unknown, we
  can factor out of the sum the first term, and continue using @p130-secondformula in terms of
  $i > 1$. Or not, because the whole term in @p130-initialformula considering the cost of the
  permutation may very well be wrong, as it considers the existence of $n$--equivalent permutations,
  independent of the developed formula for $k$ in @p130-secondformula. This latter term already
  covers the behavior of each individual instance of either #l-enum[nodes in the same tree,
    or][single--vertex nodes in a disjoint tree], so adding a factor of $n$ to the resulting
  computation does not seem logical. I could be wrong, though, as these notes are being taken while
  I solve another, more practical software engineering problem.

  Temporarily leaving aside the analysis, I have found a way of testing my implementation of the
  closest pair heuristic, and, of course, there's bugs so I need to solve them. The `ancestors()`
  method on the `Pairs` iterator is wrong. It's supposed to determine which nodes are part of the
  tree the passed node is at, but the way it's implemented, it only works with leaf nodes. A more
  apt implementation, off the top of my head, is to determine the root of the passed node's tree,
  and then determine the roots of all other nodes in the forest, matching afterwards the former with
  the latter to check which nodes are in the same tree as the parameterized node. This is already
  part of the UFDS `same()` function, so maybe a good implementation would use `repeat_n()` and
  `zip()` to fetch an iterator of ordered pairs where the first element is the passed node and the
  second is each of the other nodes in the forest. Then destructuring the tuple and computing
  `same()` with the destructured node indices should yield boolean results that can be used to
  `filter_map()` the iterator into containing only the nodes (indices) in the same tree as the
  function parameter node.

  So the algorithm has been implemented successfully. Now the thing that remains is to add a new
  iterator type that can perform #smallcaps[DFS] on the resulting tree, such that it yields each of
  the nodes in the path, while the caller of the iterator adds those nodes to the vector that is to
  be returned in the `tsp()` trait method of `TSPClosestPair`.

  The implementation should follow that `Pairs` should have a method of its own, such that provided
  the index of a node within the bounds the of its underlying `forest` field, it returns a `DFS`
  iterator that traverses the tree in which the passed node is found at.

  To more easily compute the `DFS` of the provided tree, it's best if the method on `Pairs` also
  includes logic to compute a graph structure (not `AdjacencyMatrix` because that's only valid for
  the symmetric instance of the #smallcaps[TSP],) such that `DFS` keeps ownership over that
  structure, but the structure itself is made out of references to the `Pairs` iterator, thus tying
  the lifetime of the whole `DFS` to that of the `Pairs` iterator, which makes sense.

  The actual traversal is likely going to follow the same idea as the original `DFS`, but will not
  be recursive in nature. Instead, it will approach the problem as what it really is; a traversal
  with a stack--based data structure holding the _discovered_ nodes.

  The underlying structure to be used as a representation of the graph (the tree of `Pairs`) should
  be an adjacency list, considering it's a simple #smallcaps[DAG]. The structure could then be
  modeled after the `std::collections::LinkedList` container, such that it would act as a wrapper
  around the type, where an `inner` field would hold a `Vec` of `LinkedList`s to refer to the edges
  of each of the vertices in the graph. This is going to require some preprocessing on the side of
  the `Pairs` builder method.

  Because the structure that will hold the graph that will then be part of `DFS` is a graph DS, the
  notion of _root node_ is devoid of meaning, and this logic follows that same idea. Thus the node
  passed to the method on `Pairs` should try to find the root of the tree it belongs to not for
  purposes of prioritizing it in the graph, but rather to allow finding the rest of the nodes in the
  UFDS of `Pairs` that are in the same tree. Upon finding the root, the method should contain logic
  to perform a forest--wide search to find the nodes that also evaluate to the same tree root. Even
  though I don't plan to call this until the disjoint set is reduced to a single tree, it's best if
  I provide a more generic implementation because the method can't be restricted to be called once a
  single tree is left, and neither is the implementation of such a check any better than performing
  the graph building logic directly on the provided node, trusting the user knows best if the logic
  is right.

  In terms of the logic to actually build the edge relationships between nodes, I belive it's best
  if this is also performed in--place along with vertex creation in the graph builder. And maybe
  this whole logic can be transferred to the graph type's `new()` function, passing it a shared
  reference to the method--calling `Pairs` instance. In here, as each node in the `Pairs` tree is
  found to be related to each other by means of belonging to the same tree (as determined by the
  UFDS `same()` operation,) the overarching graph type being built should consider either
  #l-enum[pushing the node if its vector doesn't yet contain the corresponding node, or][adding to
    the corresponding node's linked list an edge relating to the other node being pushed].

  To implement this dual--node addition, where one node is always a node that's present (except the
  first time we push onto the adjacency list) and another node is always a new node, such that there
  exists an edge between these, the `ancestors()` method on the passed `Pairs` instance can be used.

  In fact, to implement this part of the routine, the `ancestors()` method should replace the whole
  `same()` operation. This should prove to be more efficient, because the first element of the
  `ancestors()` method is always going to be the root node of the overarching tree. Because the
  vector returned describes the path between the first (root) and last (caller of `ancestors()`)
  nodes, we can conclude that the edges of the graph will be determined by the linear progression of
  each of the elements in that vector. This way, so long as the vector is longer than a single
  element (i.e. the requested node in the call to `ancestors()` is not a root node) then a call to
  the `windows()` method on that vector should yield a sliding view into each of the edges in the
  tree.

  The only thing left is then to take each of the yielded edges and consider whether they're already
  part of the graph we're building in the `AdjacencyList::new()` method (I've already settled on the
  name of the graph DS type.) This is not going to be ideal because each of those checks is going to
  be $Omega(n + m), "where" G = (V, E), abs(V) = n, abs(E) = m$. This has been solved by means of
  replacing the underlying DS to be a hashmap instead of a contiguous chunk of memory, each hashmap
  contaning itself another hashset to perform as well $upright(O)(1)$ queries into the existence of
  some key. This should still prove to be correct, as the container an adjacency list is modelled
  after doesn't require there being an order between nodes, nor does it expect to keep an order
  between the edges each of those nodes has.

  The `dfs()` method to return the `Dfs` iterator may end up having to perform the check I mentioned
  on all nodes being part of a single tree, because I implemented `AdjacencyList::new()` in terms of
  a constructor that doesn't require another node to be provided alongside the forest of `Pairs` to
  actually get a full traversal. Either way, the invariant holds that the method should only be
  called once the underlying disjoint set of trees has been unified into a single tree, which should
  be fairly simple to either #l-enum[avoid by only calling after the hot loop in `tsp()` of
    `TSPClosestPair`, or][ensure it holds by performing some boolean check and passing the resultant
    expression to `assert!()`]. We're going to go with both, and see how testing goes once it's all
  readily implemented.

  For the latter case, the implementation should follow that the forest only contains a single tree
  whenever, in the underlying contiguous DS, there exists only a single index containing as its
  element the index itself (i.e. there's only a single root, and thus there's only a single tree.) I
  am lead to believe this could be implemented in terms of an `all()` or `any()` method on an
  iterator over the array, but both of these happen to check for the exact same condition
  irrespective of the element in question, and they're short--circuiting. Maybe there's something
  else in either the `Vec` docs or the `Iterator` docs that better fits the need of this particular
  context. Maybe a `filter()` on a shared reference iterator over the array and a subsequent
  iterator consumer method like `count()` to assure that there's only a single element whose index
  is equal to the element it contains? Seems good enough, considering it performs an $upright(O)(n)$
  operation and linear cost for what will likely be a single call to the assertion should prove to
  be good enough. This, though, will require also performing a call to `enumerate()` for the
  subsequent iterator call chain to be capable of using the elements _and_ their indices for
  equality comparison between them.

  That part of the implementation is done, and now the only thing that remains is implementing
  `Iterator` for `Dfs`. As commented in previous notes, the implementation of the algorithm follows
  a stack--based DS traversal, which is often implemented in terms of a recursive routine with
  backtracking once there are no more edges to be traversed. This being an iterator that must
  resolve to a `Some` variant for each element of such traversal, it will require implementing the
  stack behavior manually. The implementation should then be similar in nature to the implementation
  of `Iterator::next()` on `Pairs`, where an initial `match` should check whether iteration has
  already started by probing the variant of an `Option` field of the structure (this being the
  `current_iter` field of `Dfs`.) Upon determining that iteration has not yet begun, the stateful
  part of the traversal must be initialized; Only to what must it be initialized is, indeed, the
  question I have not yet found an answer for.

  The only other field that, at this point, isn't yet provided with a value is the `stack` field,
  which is expected to hold each of the processed vertices, such that upon hitting a "dead end,"
  where no adjacent vertex remains to be explored, it starts popping off elements. Because, unlike
  the `Pairs` iterator where all elements to be yield were known the moment the `next()` call was
  made, `Dfs` cannot know which of the nodes in the graph (tree) have already been visited without
  precomputing the whole traversal instead of advancing the state on each call to `next()` as the
  user of the iterator sees fit; It's quite likely that the logic to update `stack` will be very
  similar between the starting state (the one time when `current_iter` is `None`,) and all other
  states (whenever `current_iter` yields a `Some` variant.) There's going to be need for another
  field holding the array of discovered elements, because otherwise there's no condition to be
  checked for returning a `None` and finishing iteration. This can be either implemented in terms of
  a `Vec<bool>` or in terms of a `usize` where each bit of the bit mask determines the state of one
  of the vertices in the graph. Of course, on most platforms (_most_ here meaning platforms
  following either one of the #smallcaps[LP] or #smallcaps[LLP] memory model abstraction) this would
  allow up to 256 bits to be used, assumming an 8--byte pointer size on the target the program would
  run on. This would also pair well with the current unit tests, which consider graphs with a very
  small amount of vertices (well within bounds of a graph with $abs(V) = 256$.) Still, a
  conservative approach would use an estimate based on the comments of Skiena's book, where
  apparently, one of the robots for which such a symmetric instance of the #smallcaps[TSP] is
  required may expect to visit up to 1000 different points. For this reason, the implementation will
  use a `Vec<bool>`.

  Upon entering the `None` arm of the `match`, the state of the `current_iter` should be set to
  `Some(0)`. Then the `stack` field will use such index as the initial vertex with which to start
  the traversal in the underlying `AdjacencyList`. Because this graph DS is using a hashmap that
  considers as its keys the `usize` indices of the vertices in the graph, and yields as its values
  the hashsets with the `usize` indices of the adjacent vertices (i.e. those the vertex key has
  edges with,) the first thing that should be done is to mark index 0 (whichever element that turned
  out to be) as having been discovered by setting the flag inside the `discovered` field (itself
  already provided with as many elements as there were nodes in the `Pairs` tree.) Then the same
  loop as the one performed with a recursive #smallcaps[DFS] should consider each of the adjacent
  nodes by traversing the hashset returned from accessing the value of key 0, after which each of
  those elements should be pushed to the `stack` field. Then it holds that the element to be
  returned from the first call to `next()` should be the element at index 0.

  Indeed, if the element to be returned is the one at index 0, then the `None` branch should not do
  anything beyond setting `current_iter` to `Some(0)`, and returning that same `Some(0)`. Then the
  `discovered` field should still set the flag for the elemetn (index here) for that node, namely
  index 0, and actually proceed to return the `Option`--wrapped index.

  Even though this could yield an valid traversal out of context, this doesn't assure that traversal
  starts at the root of the original tree. This implies that, contrary to what was mentioned before
  about the root node of the `Pairs` tree not having any special meaning in the graph DS used to
  perform #smallcaps[DFS], the root node _does_ need to be recorded in some way, such that traversal
  of the graph starts at that node, and not at other nodes. This also implies that the resulting
  graph stored in the `AdjacencyList` of `Dfs` will have to be a directed graph, and thus not add to
  both nodes considered in its `new()` asssociated function the same arc. Instead, during the hot
  loop in that routine, only `node1` should have added `node2` to its list of adjacent nodes. I
  believe the simplest way to keep track of the root node in the `Pairs` tree is going to be using
  the `stack` field to get stored in it (initally as its single element) that vertex (its index,
  really.) Then, because it only get used once, and future calls to `next()` will yield other
  vertices of the graph, that first element can be scraped after the first iteration.

  The revised implementation of `next()` upon entering the `None` branch should follow that
  `current_iter` should be set to `Some(idx)` where `idx` denotes the index of that first element
  within `stack` used for the root of the tree. Then once the value is copied (and prior to
  returning `current_iter` from the routine,) `stack` should have its one value popped off.

  The implementation of the `Some` branch starting on the second call to `next()` should then
  actually consider traversing the hashset of the vertex given by `current_iter`, and push onto the
  stack whichever element is yield first in that hashset. The only issue is that because this is an
  unordered container, each run of #smallcaps[DFS] is going to yield a different traversal, as
  iterating through the hashset multiple times, even with the same graph layout, provides no
  guarentees on the order of the yielded elements across runs. This routine, though, relies heavily
  on performing a pre--order traversal akin to that of binary trees to have the tree in `Pairs` link
  its leaves together. The solution could quite possibly go through replacing the hashset in the
  adjacency list used for the `Dfs` graph to a collection storing its elements in contiguous memory.
  The downside is going to be refactoring `new()` from `AdjacencyList` such that it still performs
  the required checks when creating a new arc in the graph.

  The hashset was replaced with a binary tree set, though the issue on the order in which edges are
  added to this container still holds because the `Pairs` tree is traversed in terms of the
  contiguous elements of the array, and not in terms of the actual tree node layout (this being the
  whole purpose of performing #smallcaps[DFS] and thus creating a graph proper.) We'll ignore it for
  now, and see how things turn out. Because of this, the container will be restored to a hashset as
  we've concluded that order is not even maintained by the iterated elements of the `Pairs` tree.

  Back to the implementation of `next()`, the element yield initially on the `None` branch should
  quite possible not be popped off the stack, as only values that have had all descendant nodes
  processed should have this happen to them. This implies that the actual invariant to be upheld by
  the iterator to yield `Some` values should be that of the presence of elements in its `stack`
  field, and not that of the presence of any values with their `discovered` flag not set. Thus,
  `discovered` would be relegated to the role of performing a check on which elements should be
  added to the stack.

  The value of the element that is first yield in the `next()` call should now remain the same as
  the one determined prior to this discussion, but instead the stack should _not_ have it popped. On
  the next iteration's traversal across `current_iter` (the root node,) it should decide which
  vertices adjacent to it are added to the stack. The implementation for that should first consider
  the state of the `discovered` field, with an `any()` call on a shared reference iterator that
  would consider whether there are still elements to be yield as not all of the tree as been
  discovered, or whether iteration should end. The Rust way of doing this would be to call on the
  resulting boolean value of that iterator consumer method another `then_some()` call providing as
  the inner value the unit value. Then a call to `?` to propagate a `None` up the call stack would
  allow iteration to finish, while allowing the next (possibly costly) computations to be avoided.

  The next part of the `Some` branch should include the logic concerning adding elements to the
  stack and properly updating the value of `current_iter`, which holds the vertex (index) to be
  returned next from the iterator. For that, one must consider both the current state of the stack,
  as well as the vertex (index) yielded from destructuring the `Some` variant. The stack should add
  all vertices adjacent to the destructured vertex, and proceed to update `current_iter` to some
  element of that list. The intricate part of the problem here is determinig which one of the
  elements of the stack should be _the chosen one_. Technically speaking, if the underlying vector
  is used as a stack adaptor in the spirit of C++, then there's only one possible element to be
  returned, and that's the top of the stack; For any other random access operation would prove to be
  too costly (even for the vector.) Let us model how should the iterator behave in a less abstract
  setting.

  Given a tree with three child nodes stemming from the root, the second call to `next()` would add
  all child nodes to the stack, and then select one of those (this being "non--deterministic" in
  nature because the source of those child nodes is the hashset holding the vertices adjacent to, in
  this instance, the root node of the tree.) Assume then that 2 of those child nodes are leaf nodes,
  and the other roots itself a subtree with a single descendant. Assume as well that the top of the
  stack turned out to be one of the two former leaves. The behavior in `next()` should backtrack and
  advance to the next element in the stack by performing the mandatory check for adjacent nodes to
  be added to the vector, to then fetch the value at the top the stack and return it as the next
  item in the iterator sequence. Because the elements added to the stack are not concerned with the
  order of nodes in the orignal `Pairs` tree, the node that the iterator would move on to next could
  be any one of #l-enum[the sibling leaf, or][the subtree with a single child]. At the
  #smallcaps[TSP] level, which one it moves to next is of great importance, but at the tree level,
  this is not so much the case. Thus, because the abstraction seems to be leaning further towards
  the latter (and because I've spent too much time on this problem,) we will ignore the improvements
  that could be made from, off the top of my head, #l-enum(
    numbering: "(a)",
  )[performing the inherent conversion into a binary tree that is possible with any simple
    #smallcaps[DAG] and then more easily implementing a pre--order traversal iterator such that the
    child nodes are actually modelled after an equivalent, embedded graph, or][keep track of the
    original nodes from the `Pairs` tree such that the stack can decide which of the equivalently
    possible nodes to move on to `current_iter` should actually move on to].

  To recap the high--level sequence of steps that this execution branch should go through: Update
  the stack by pushing to it the vertices adjacent to `current_iter`, then fetch the top of the
  stack and update `current_iter` with that value, prior to popping the top of the stack off, and
  finally setting the flag in `discovered` for the new value of `current_iter`.

  Testing has already started with the supposedly completed `tsp()` method for `TSPClosestPair`. The
  thing is done, but the tests fail with a panic in the `Iterator` implementation for the `Dfs`
  adaptor I designed to perform traversal. It seems that the iterator is at some point attempting to
  perform an indexed access operation over the hashmap being used in the adjacency list abstraction,
  and that hashmap doesn't seem to contain the look up key. This shouldn't even be possible,
  considering the thing that indexes the hashmap is `current_iter`, and this only ever gets values
  from the original `Pairs` tree. If there really is no way the logic in the non--graph--exclusive
  fields of the `Dfs` iterator is wrong, then maybe what's wrong is the logic building the
  `AdjacencyList` in its `new()` associated function.

  I've identified the issue. It turns out the `new()` function on `AdjacencyList` was not correctly
  considering each of the edges as I had planned on, but I've yet to solve this.

  The closest pair heuristic now finally solves the tests correctly. It only took #(6 * 5) hours. We
  can now go back to the time complexity analysis we were having on the algorithm used.

  The last comments I made on the analysis were debating the possibility for it to be completely
  wrong, so instead of trying to accomodate some new conclusion to the existing material, I believe
  it best to simply analyze the implementation from the scratch. This was going to be necessary
  anyway, considernig the latest, working implementation adds more non--constant time operations.

  Prior to jumping onto the analysis, it's important to note the three main components with linear
  or sublinear cost in the operations performed within the hot loop of the algorithm. First, the
  `min_fix()` operation computes all possible Cartesian products of every node in the forest, given
  the constraint that for any tree made out of nodes $T = {a, b, dots.c, k}$, any one node $c$ may
  not consider as part of the rhs of their Cartesian products any other node in $T dif {c}$. Second,
  the `unite()` operation performs some sublinear cost operation to determine the root node in the
  trees of both nodes passed to it. And finally, depending on whether the node passed to `unite()`
  to be made a child of the other node (also passed to `unite()`) is a root of another disjoint tree
  in the forest, a call to `ancestors()` to determine all nodes preceding it will be required to
  keep the underlying array holding the forest in a consistent state.

  Adding up the values of each of these for every single one of the $n - 1$ iterations in the main
  loop should yield the total cost for the algorithm, where $n$ is the number of nodes in the tree,
  and on a higher--level, the number of points that the robot arm must go through.

  We'll start the analysis with the `min_fix()` operation. The core of this routine considers all
  trees in the forest, and given some node $a$, determines the lightest edge in the graph, denoted
  by ordered pair $(a, b)$, sourced from the union of all sets yielded by each of the Cartesian
  products. The behavior of this follows that for each of the $n - 1$ iterations of the main loop,
  the program experiments two different costs: #l-enum[on the first iteration, the Cartesian product
    computed is the same for all nodes in the tree, as they are single--vertex disjoint trees, this
    being the initial state of the #smallcaps[UFDS]][on subsequent iterations, the nodes belonging
    to the same tree will compute a Cartesian product equivalent to $n - i$, where $i$ denotes the
    (0--indexed) running iteration count]. The operation should thus have a cost of $n dot n$ when
  iterating through the the first value of `current_node` in `Pairs` (i.e. the first iteration of
  the hot loop,) and for all future iterations should compute $(i dot (n - i) + (n - i) dot n)$,
  where $i$ is the control variable keeping track of the iteration count ($[1, n)$.)

  In a sum formula, this would be expressed as

  $
    n dot n + sum_(i = 2)^(n - 1) i dot (n - i) + (n - i) dot n =
    n^2 + sum_(i = 2)^(n - 1) n i - i^2 + n^2 - n i & = \
    & = n^2 + sum_(i = 2)^(n - 1) n^2 - i^2 \
    & = n^2 + n^2 dot sum_(i = 2)^(n - 1) 1 - sum_(i = 2)^(n - 1) i^2 \
    & = n^2 + n^2 dot (sum_(i = 1)^(n - 1) (1) - 1) - sum_(i = 2)^(n - 1) i^2 \
    & = n^2 + n^2 dot (n - 2) - (sum_(i = 1)^(n - 1) (i^2) - 1) \
    & = n^2 + n^2 dot (n - 2) - (n (n + 1) (2n + 1) - 6) / 6 \
    & = n^2 + n^3 - 2n^2 - ((n^2 + n) (2n + 1) - 6) / 6 \
    & = n^2 + n^3 - 2n^2 - (2n^3 + n^2 + 2n^2 + n - 6) / 6 \
    & = n^3 - n^2 - (2n^3 + 3n^2 + n - 6) / 6 \
    & = (6n^3 - 6n^2 - 2n^3 + 3n^2 + n - 6) / 6 \
    & = 1 / 6 (4n^3 - 3n^2 + n - 6) \
    & approx Theta(n^3).
  $

  I consider this a tight bound on $f(n) = n^3$ because the running time will always be tied to the
  fixed $n - 1$ iterations of the overarching loop in the implementation, and each iteration is
  assured to decrease the number of trees in the forest by one; Also, all Cartesian products are
  computed without any consideration for caching, which forces the exact same procedure no matter
  the case.

  We move on now to the second part of the algorithm, namely the `unite()` #smallcaps[UFDS]
  operation that is (also) guaranteed to run once on each $n - 1$ iteration. This operation is known
  to have $upright(O)(lg n)$ sublinear performance on a traditional implementation of a union--find
  DS, but this problem required slightly altering its usual behavior. Under normal circumstances,
  `unite()` would incurr a (constant factors included) cost of $upright(O)(2 dot lg n)$, where $n$
  denotes the upper bound for the sublinear `find()` operation (left unchanged from the regular
  #smallcaps[UFDS] implementation) to reach the root of the tree nodes passed to the subroutine
  (i.e. the largest cost of any one of the two `find()` operations per (the two) nodes.) The
  implementation for this problem instead holds the invariant that the only, truly sublinear
  operation is that of the node that will become the parent, as the other node (through the prior
  workings of the reverse `ancestors()` in the algorithm's main loop, that we'll comment on later)
  is always guaranteed to be a root node, and thus a call to `find()` on it would resolve
  immediately with $approx Theta(1)$.

  Because determining which edge (ordered pair) get selected as the lightest edge through the
  `min_fix()` operation is not possible without prior knowledge of the set of points the
  #smallcaps[TSP] tour is expected to go through, we will analyze the behavior of this operation in
  terms of an upper asymptotic bound, instead of a tight bound as we did with the combination of
  Cartesian products.

  Let us define first the roles of each of the ordered pairs returned by the `min_fix()` operation.
  Given some pair $(a, b)$, the `unite()` operation, as per the prior discussion, will be assured
  that node $b$ is always a root, and thus a `find()` operation to reach the root of its tree will
  resolve in $Theta(1)$. Thus node $a$ is the one node that may or may not be a root itself, and
  will become the new parent of node $b$ in the disjoint set.

  The worst case scenario here would be for some set of points the robot arm ought go through,
  namely the closest edge $(a, b)$, to always bound to hold true that $a < b$ in the extended line
  of $RR$. This would imply that setting up a forest of trees would always force the leaf node of
  the largest tree to be joined with some other single--vertex tree. The effect of this would be
  that for such a leaf node $a$ taking the role of the new parent in the #smallcaps[UFDS]--`unite()`
  operation, the root node of the tree it would be contained in would be the largest possible tree
  at any given iteration.

  Thus, for some number of iterations $n - 1$, if a single tree is the one tree that always keep
  growing, the resulting height of that tree at any given (0--indexed) iteration $i$ would be $i$
  itself, such that by the end iteration, namely $n - 1$, the tree height would finally become a
  linked list--like structure akin to a chain of vertices with height (length or size for linked
  DSs) $n$ (upon completion of the last `unite()` operation prior to exitting the algorithm's main
  loop.) This means that each `unite()` operation calling the `find()` routine on the
  (to--be--parent) node would incurr $i$ stack frame allocations to find the actual parent of $a$.

  In such a worst case, the total cost of the `unite()` operation would be

  $
    sum_(i = 1)^(n - 1) i & = sum_(i = 1)^n (i) - n = (n(n + 1)) / 2 - n \
                          & = (n^2 + n - 2n) / 2 = 1 / 2 (n^2 - n) approx upright(O)(n^2).
  $

  The total running cost so far, accounting for both the prior, fixed--cost combination of Cartesian
  products and for the latest conclusion on the cost of the (modified) `unite()` operation, is
  $Theta(n^3) dot upright(O)(n^2)$. Note *there's a mistake* in the computation of the asymptotic
  running time of the `min_fix()` operation, as it uses the known result on sums
  $i = [1, n] "for" i^2 equiv (n(n + 1)(2n +1)) / 6$ even though the actual range for the treated
  sum is $i = [1, n)$. Either way, the approximate behavior should compute similarly even with the
  right treatment of the formula.

  Finally, we discuss the time complexity of the conditional operation performed on some node $b$ in
  some resulting min--edge denoted by ordered pair $(a, b)$, where node $b$ is the vertex to become
  the child of node $a$ through the `unite()` operation. Note the routine about to be treated is
  what actually allows node $b$ to be assumed to be a root node of the forest, which further allows
  the assumption on the cost of `find()`ing the root of such node to be constant in the analysis of
  the `unite()` operation.

  First, much as with the `unite()` subroutine, we will operate on a worst--case scenario basis as
  the actual behavior is instance--dependent. To that extent, we assume as a worst--case the
  possibility for the node to--be--child to be a leaf node in a long tree within the disjoint set.
  Such a situation would take place if say, the points the robot arm had to go through were
  separated in such way so as to have half of those nodes tightly gathered around one side, and the
  other half tightly gathered around the opposite side. This would force the disjoint set to, in
  general, reach iteration $n - 1$ and have as the last ordered pair of points some node $a$ on one
  side and some other node $b$ on the opposite side. This is due to the fact the closest pair
  heuristic computes disjoint edges, and in all prior iterations would have kept adding up nodes to
  one of two main trees, each representing a side of the surface area.

  The behavior here would then be modelled in the exact same way as the cost of the (modified)
  `unite()` operation, as the `ancestors()` function on the disjoint set fundamentally performs the
  same steps as the `find()` operation, only iteratively instead of recursively. Upon yielding the
  ancestors to some such node $b$, that part of the algorithm in the main loop traverses anew the
  collection of ancestors from the root until node $b$ to perform an $upright(O)(1)$
  parent--reversing operation on each of them. This would total
  $2 dot upright(o)(n^2) approx upright(o)(n^2)$.

  The final cost of the algorithm is then $Theta(n^3) dot (upright(O)(n^2) + upright(O)(n^2))$. This
  is definitely worse than the fixed, sure cost of the nearest neighbor heuristic, and indeed, it
  aligns with the expected performance drop that Skiena's book speaks of when proposing this
  alternative approach.

  Finally, the last part of the problem asks to implement a more optimized heuristic for the
  instance of the #smallcaps[TSP] considered in the robot arm problem. For that, I can implement a
  known method to solve for a 15--20% suboptimal heuristic based on finding an #smallcaps[MST] of
  the complete graph under consideration, and then performing #smallcaps[BFS] on it (or was it
  #smallcaps[DFS]? I need to check out the algorithm catalogue on the book,) while keeping a record
  of each fully processed vertex in the graph serving as the sequence of points to be visited in the
  tour. After having read the section on Skiena's catalogue, I can say the heuristic I'm going to
  implement is the #smallcaps[MST]--finding one, followed by a #smallcaps[DFS] on the resulting
  tree, which counter to what I said before, considers as the resulting path the set of vertices as
  they are _discovered_, and not once they are _processed_. This approach should also allow me to
  reuse the `Dfs` iterator I created for the purposes of solving the closest pair heuristic.

  I may also want to research on Kernighan--Lin _k--opt tours_ to apply a 2--opt tour to the result
  of performing #smallcaps[DFS] on the result of the #smallcaps[MST]. If time allows, research on
  simulated annealing to further enhance the result of the heuristic would also be great.

  To start off, I'll look into both sections of Skiena's book that treat with finding an
  #smallcaps[mst] for a graph, namely the one on the chapter about graphs and the one receiving its
  own section on the catalogue. The book chapter on graphs covering #smallcaps[MST]s does give some
  pointers to sections of the catalogue that include content that may be of interest without
  strictly being part of the core algorithm routine, per se.
  *Following, I note such sections, for future reference.*

  - Section 18.3, on the #smallcaps[MST] algorithm itself.
  - Section 17.6, on techniques for quickly building set partitions (may be of use to improve
    building the disjoint set at the core of the #smallcaps[UFDS] used for Kruskal's.)
  - Section 15.5, on techniques to further improve the #smallcaps[UFDS] DS, beyond mere path
    compression.

  Based on readings about the asymptotic behavior of the #smallcaps[MST]--finding algorithms
  discussed in the book chapter, I may consider implementing Prim's instead of Kruskal's as the
  latter seems more fit for applications where the subject graph is sparse in nature. In the
  instance of the #smallcaps[TSP] for the robot arm tour, the graph is known to be a complete,
  simple graph so there's bound to be $m = n - 1 "edges, where" G = (V, E), abs(V) = n, abs(E) = m$.
  This is an inherently dense graph, and for a quick test with $2^80$ vertices, the worst--case
  result of Kruskal's is two orders of magnitude worse than that of Prim's.

  The next step is going to be actually reading the above sections on the topic and seeing which
  approach should work best.

  After having browsed the section on the catalogue, it seems that the most efficient implementation
  is going to go through solving the problem as a geometric instance initially, such that after
  computing the Delauney triangulation on the set of points (vertices of the complete graph,) and
  then running Kruskal's on the resulting graph, we are left with an $upright(O)(n lg n)$ total
  running time. This should be the most optimal solution, considering such a complete, simple graph
  would contain $n$ vertices and $m = n^2 - n approx n^2$ edges (which for some large graphs makes
  the cost of more conventional solutions to finding an #smallcaps[MST], like Prim's
  $upright(O)(m + n lg n)$, inefficient in comparison.)

  We'll go with this. For that, I'm going to need browsing through the pages of the catalogue on
  Delauney triangulation (and possibly the chapters covering material on computational geometry
  algorithms.) Once I've implemented this, I'll have to look into Kruskal's algorithm and optimizing
  the #smallcaps[UFDS] involved in it (see the list of sections included above.) Once that is done,
  I'll have to implement a #smallcaps[DFS] traversal on the resulting graph, and that should about
  do it.

  I'm done reading the catalogue, and I think I have an overall idea of the abstractions behind both
  Delauney triangulation and Voronoi diagrams. Most of the material discussed, though, is not
  relevant to the geometric instance of the robot arm tour problem, as I assume the surface in which
  the robot is epxected to work in is 2--dimensional. Still, modelling the problem in terms of the
  simpler approach whereby a convex hull polygon is formed from the sorted x--component of the
  target points in Euclidean space likely won't do. This is because Skiena himself makes explicit in
  the #smallcaps[TSP] section that solving it as a geometric problem requires specifically using
  Delauney triangulation, and not merely _a_ method of triangulation.

  I'll look now into the chapter on computational geometry to better understand the possibilities I
  have, considering I will not have access to the Internet for some time.

  I think I have a way to go, though it's not the most optimal. I'm going to be using the method
  proposed by Skiena to build the convex hull of the point set, and as each point gets removed from
  the convex hull because it turns left (as per Andrew's algorithm in _Guide to Competitive
  Programming_,) the edge between the last point in the hull and the one that got cut off is added
  as an edge that is part of the target triangulation. For this, I would need to further expand the
  tests that I currently use for the previously implemented #smallcaps[TSP] heuristics, such that
  they also include 2--dimensional geometric information on each the points the robot arm must go
  through. Then I should hold in a contiguous collection all such points, and sort them in
  $upright(O)(n lg n)$ first by their $x$--components, and second by their $y$--component. For that,
  I must define a relation of total ordering for the algebraic data type that will represent the
  2--dimensional points, such that the sorting algorithm performs unstable sorting where elements
  with differing values of $y$ but equal values of $x$ are further sorted and not just grouped
  together in their original (or possibly some other) order.

  The next thing would be to implement the convex hull algorithm in such way so as to allow an
  external closure to add elements onto another container (the target triangulation) whenever some
  element of the point set is cut off from the points that denote the polygon's perimeter. For that,
  I belive it best if I implement the convex hull algorithm first and make sure that both the above
  sorting routine and the convex hull produce satisfactory results, prior to attempting anything
  triangulation related.

  In terms of implementation design, the trait should have an interface for the `tsp()` method that
  serves as the only real call to solve the problem, as well as a method for performing the two main
  routines involved in this heuristic; Namely, finding the #smallcaps[MST] of the input graph, and
  performing #smallcaps[DFS] on that graph. The only difference now is that the input to `tsp()`
  should augment the information held on each edge of the graph, such that it collects both the
  weight and the 2--dimensional coordinates of each point. This is going to require new graph and
  edge primitives. The edge primitive should continue being an enumeration, except for the variant
  holding a weighted edge, which should be changed from a tuple--like `struct` to a named--field
  `struct` containing the `weight: usize` and the `coord: Point`. The graph primitive should
  continue being an adjacency matrix because the graph has the same high--density vertex count, but
  it should replace the old edge primitive with the new edge data type. The new macro I already
  introduced to build a vector of `Point`s should be refactored into taking in the weight of the
  edge in the same way as the `matrix!` macro does, and pass it off to the `new()` function of the
  new adjacency matrix.

  The implementation of `AugAdjacencyMatrix::new()` should follow the same input requirements as
  those enforced in `AdjacencyMatrix::new()`; namely that #l-enum[the input matrix should be a
    square matrix][that it should have weighted edges everywhere but in the main diagonal, and][that
    its transpose is equal to the original matrix (which for a square matrix implies that computing
    the inverse twice returns the same original matrix.)] Implementation--wise, checking for the
  matrix to be square involves having the function check each of the inner container elements of the
  overarching container, and making sure the length of each of the formers is equal to the length of
  the latter. the check for weighted edges should be two--part: #l-enum[filter out the nonexistent
    edges, and ensure the length of the resulting collection is one less than the length of the
    iterated--over row vector, and][].

=== LeetCode problems

/ Problem 1--1: \
  *Daily temperatures*

  Given an array of integers `temperatures` representing daily temperatures, return an array
  `answer` such that `answer[i]` is the number of days you have to wait after the `i`th day to get a
  warmer temperature. If there is no future day for which this is possible, keep `answer[i] == 0`
  instead.

  _Example 1_:

  - Input: `temperatures = [73,74,75,71,69,72,76,73]`
  - Output: `answer = [1,1,4,2,1,1,0,0]`

  _Example 2_:

  - Input: `temperatures = [30,40,50,60]`
  - Output: `answer = [1,1,1,0]`

  _Example 3_:

  - Input: `temperatures = [30,60,90]`
  - Output: `answer = [1,1,0]`

  _Constraints_:

  - $1 <= mono("temperatures.length") <= 10^5$
  - $30 <= mono("temperatures[i]") <= 100$

/ Problem 1--2: \
  *Rotate list*

  Given the `head` of a linked list, rotate the list to the right by `k` places.

  _Example 1_:

  - Input: `head = [1,2,3,4,5], k = 2`
  - Output: `[4,5,1,2,3]`

  _Example 2_:

  - Input: `head = [0,1,2], k = 4`
  - Output: `[2,0,1]`

  _Constraints_:

  - The number of nodes in the list is in the range $[0, 500]$.
  - $-100 <= mono("Node.val") <= 100$
  - $0 <= mono("k") <= 2 dot 109$

/ Problem 1--3: \
  *Wiggle Sort II*

  Given an integer array `nums`, reorder it such that
  $mono("nums[0]") < mono("nums[1]") > mono("nums[2]") < mono("nums[3]") dots.c$
  You may assume the input array always has a valid answer.

  _Example 1_:

  - Input: `nums = [1,5,1,1,6,4]`
  - Output: `[1,6,1,5,1,4]`

  Explanation: `[1,4,1,5,1,6]` is also accepted.

  _Example 2_:

  - Input: `nums = [1,3,2,2,3,1]`
  - Output: `[2,3,1,3,1,2]`

  _Constraints_:

  - $1 <= mono("nums.length") <= 5 times 10^4$
  - $0 <= mono("nums[i]") <= 5000$
  - It is guaranteed that there will be an answer for the given input `nums`.

  _Follow Up_: Can you do it in $upright(O)(n)$ time and/or in-place with $upright(O)(1)$ extra
  space?
