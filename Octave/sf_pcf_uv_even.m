## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pcf_uv_even (@var{a}, @var{z})
## Compute even series from parabolic cylinder functions $U(a,z), U(a,z)$
## @end deftypefn

function res = sf_pcf_uv_even(a, z)
  if (nargin < 2) print_usage; endif
  if (any(size(z)!=size(a)))
    if (isscalar(z)) z*=ones(size(a));
    elseif (isscalar(a)) a*=ones(size(z));
    else error("sf_pcf_u: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = uu_1(a(kk), z(kk));
  endfor
endfunction

function res = uu_1(a, z)
  NN = 50;
  res = sum([1, cumprod(z^2*ones(1,NN/2) ./ (1:2:NN) .* (a+(2*(1:2:NN)-1)/2) ./ (1+(1:2:NN)))], 'extra');
  res *= sf_exp(-z^2/4);
endfunction
