#import "@local/typst-template:0.17.0": *

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

  That doesn't look like the result on Skiena's book. Still, it's technically correct.
