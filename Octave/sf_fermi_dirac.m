## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_fermi_dirac (@var{z}, @var{s})
## Compute the Fermi-Dirac integral
##   $F_s(z) = \frac{1}{\Gamma(s+1)}\int_0^\infty\frac{t^s}{e^{t-x}+1}\,ds$
##   $(s>-1)$
## (simple implementation using quadrature)
## @end deftypefn
function res = sf_fermi_dirac(z, s)
  if (nargin < 2)
    print_usage;
  endif
  if (s<=-1) res = nan; endif
  q = quad(@(t)(t^s / (e^(t-z)+1)), 0, Inf, 1e-12);
  res = q/gamma(1+s);
endfunction
