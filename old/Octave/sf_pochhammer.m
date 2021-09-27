## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pochhammer (@var{z}, @var{alpha})
## Compute the Pochhammer symbol $(z)_\alpha$
## @end deftypefn

function res = sf_pochhammer(z, al)
  if (nargin<2) print_usage; endif
  if (any(size(z)!=size(al)))
    if (isscalar(z)) z*=ones(size(al));
    elseif (isscalar(al)) al*=ones(size(z));
    else error("sf_pochhammer: mismatched parameter sizes");
    endif
  endif

  res = zeros(size(z));
  for kk = 1:prod(size(z));
    res(kk) = sf_pochhammer__1(z(kk), al(kk));
  endfor
endfunction

function res = sf_pochhammer__1(z, al)
  if (sf_is_nonnegint(al) || al<100)
    res = prod(z + (0:(al-1)));
    return;
  endif
  res = sf_exp(sf_ln_gamma(z + al) - sf_ln_gamma(z));
  # kludge cleanup
  if (sf_is_int(z) && sf_is_int(al)) res = round(res); endif
  if (imag(z)==0 && imag(al)==0) res = real(res); endif
endfunction
