## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bose_einstein (@var{z}, @var{s})
## Compute the Bose-Einstein integral
## @tex
##   $$
##      G_s(z) = {1 \over \Gamma(s+1)}\int_0^\infty{{t^s}\over{e^{t-z}+1}}\,ds
##   $$
##   $(s>-1,z<0; s>0,z\leq0)$
## @end tex
## @ifnottex
##
##   G_s(z) = 1/Gamma(s+1) int_0^Inf  t^s / (e^(t-z)+1) ds
##
##   (s>-1,z<0; s>0,z<=0)
##
## @end ifnottex
## (simple implementation using quadrature)
## @end deftypefn
function res = sf_bose_einstein(z, s)
  if (nargin < 2)
    print_usage;
  endif
  if (z>0) res = nan; return; endif
  if (s<=-1) res = nan; return; endif
  if (z==0 && s<=0) res = nan; return; endif
  q = quad(@(t)(t^s / (e^(t-z)-1)), 0, Inf, 1e-12);
  res = q/gamma(1+s);
endfunction
