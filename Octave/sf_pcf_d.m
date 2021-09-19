## -*- texinfo -*-
## @deftypefn {Function File} {@var{D} =} sf_pcf_d (@var{a}, @var{z})
## Compute parabolic cylinder function $D_a(z)$
## @end deftypefn

function D = sf_pcf_d(a, z)
  if (nargin < 2) print_usage; endif
  D = sf_pcf_u(-a-1/2, z);
endfunction
