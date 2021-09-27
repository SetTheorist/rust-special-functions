## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_tangent_number (@var{n})
## Compute tangent number $T_n$
## @end deftypefn

function res = sf_tangent_number(n)
  if (nargin < 1)
    print_usage;
  endif
  rm2 = rem(n,2);
  neven = (rm2==0);
  nodd = (rm2==1);
  nother = (rm2!=0 & rm2!=1);
  n(nother) = NaN;
  n(neven) = 0.0;
  np = n(nodd)+1;
  n(nodd) = 2.^(np) .* (2.^(np) - 1) .* abs(sf_bernoulli_number(np)) ./ (np);
  res = n;
endfunction
