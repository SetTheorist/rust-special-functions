## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_genocchi_number_scaled (@var{n})
## Compute scaled Genocchi number $G_n / n!$
## @end deftypefn

function res = sf_genocchi_number_scaled(n)
  if (nargin < 1)
    print_usage;
  endif
  res = 2 * (1 - 2.^n) .* sf_bernoulli_number_scaled(n);
endfunction
