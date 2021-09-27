## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hypergeom_series_2f1 (@var{a}, @var{b}, @var{c}, @var{z})
## Compute the hypergeometric 2F1 *series*.  Direct (naive) implementation of summation.
## @end deftypefn

function res = sf_hypergeom_series_2f1(a, b, c, z)
  if (nargin != 4) print_usage; endif
  res = 1.0;
  term = 1.0;
  n = 1;
  e_ = 0;
  do
    term *= z / n * (a+n-1) / (c+n-1) * (b+n-1);

    old_res = res;
    #res += term;
      t_ = res;
      y_ = term + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;

    ++n; if (n>999) break; endif
  until (res == old_res)
  res += e_;
endfunction
