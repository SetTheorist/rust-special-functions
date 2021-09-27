## -*- texinfo -*-
## @deftypefn {Function File} {@var{res}=} sf_pcf_u_du0 (@var{a})
## Compute value of parabolic cylinder function $U'(a,0)$
## @end deftypefn

function res = sf_pcf_u_du0(a)
  persistent sqpi = 1.772453850905516027298167483;
  if (nargin < 1) print_usage; endif
  res = -sqpi * 2.^(-a/2+1/4) ./ sf_gamma(a/2 + 1/4);
endfunction
