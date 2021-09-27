## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pcf_uv_odd (@var{a}, @var{z})
## Compute odd series from parabolic cylinder functions $U(a,z)$, $V(a,z)$
## @end deftypefn

function res = sf_pcf_uv_odd(a, z)
  if (nargin < 2) print_usage; endif
  if (any(size(z)!=size(a)))
    if (isscalar(z)) z*=ones(size(a));
    elseif (isscalar(a)) a*=ones(size(z));
    else error("sf_pcf_uv_odd: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = uu_2(a(kk), z(kk));
  endfor
endfunction

function res = uu_2(a, z)
  NN = 51;
  res = sum(cumprod([z, z^2*ones(1,NN/2) ./ (2:2:NN) .* (a+(2*(2:2:NN)-1)/2) ./ (1+(2:2:NN))]), 'extra');
  res *= sf_exp(-z^2/4);
endfunction
