## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gamma_p1m1 (@var{z})
## Compute $\Gamma(1+z)-1$ (without losing precision for small z)
## @end deftypefn

function res = sf_gamma_p1m1(z)
  if (nargin<1)
    print_usage;
  endif
  if (abs(z)>=1/2)
    res = sf_gamma(1+z)-1;
  else
    res = -sf_log_p1(z) + z*(0.4227843350984671393934879099);
    k = 2;
    do
      old_res = res;
      term = (-1)^k * sf_zeta_m1(k) * z^k / k
      res += term
      ++k;
      if (k>999) break; endif
    until (res == old_res)
    res = sf_exp_m1(res);
  endif
endfunction
