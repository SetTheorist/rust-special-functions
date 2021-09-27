## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pcf_v (@var{a}, @var{z})
## Compute parabolic cylinder function $V(a,z)$
## @end deftypefn

function res = sf_pcf_v(a,z)
  if (nargin < 2) print_usage; endif
  if (any(size(z)!=size(a)))
    if (isscalar(z)) z*=ones(size(a));
    elseif (isscalar(a)) a*=ones(size(z));
    else error("sf_pcf_v: mismatched parameter sizes");
    endif
  endif
  res = sf_pcf_v_v0(a).*sf_pcf_uv_even(a,z) + sf_pcf_v_dv0(a).*sf_pcf_uv_odd(a,z);
endfunction
