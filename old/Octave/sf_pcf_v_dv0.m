## -*- texinfo -*-
## @deftypefn {Function File} {[@var{U},@var{V}] =} sf_pcf_v_dv0 (@var{a})
## Value of parabolic cylinder function $V'(a,0)$
## @end deftypefn

function res = sf_pcf_v_dv0(a)
  if (nargin < 1) print_usage; endif
  res = 2.^(a/2+3/4) .* sf_sin(pi*(1/4-a/2)) ./ sf_gamma(1/4 - a/2);
endfunction
