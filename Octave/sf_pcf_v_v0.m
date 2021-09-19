## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pcf_v_v0 (@var{a})
## Compute value of parabolic cylinder function $V(a,0)$
## @end deftypefn

function res = sf_pcf_v_v0(a)
  if (nargin < 1) print_usage; endif
  res = 2.^(a/2+1/4) .* sf_sin(pi*(3/4-a/2)) ./ sf_gamma(3/4 - a/2);
endfunction
