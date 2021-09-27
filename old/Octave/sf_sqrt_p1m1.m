## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sqrt_p1m1 (@var{z})
## Compute $\sqrt(1 + x) - 1$ (keeping accuracy for small x)
## @end deftypefn
function res = sf_sqrt_p1m1(z)
  if (nargin < 1)
    print_usage;
  endif

  if (abs(z)>0.15)
    # direct
    res = sf_sqrt(1.0 + z) - 1.0;
  else
    # series
    res = 0.0;
    n = 1;
    e_ = 0.0;
    do
      old_res = res;

      #res += z^n * bincoeff(1/2, n);
      # Kahan summation
      t_ = res;
      y_ = (z^n*bincoeff(1/2, n)) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;

      ++n;
      if (n>999) break; endif
    until (res == old_res);
  endif
endfunction
