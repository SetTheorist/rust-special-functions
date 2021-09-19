## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cvsin (@var{z})
## Compute the coversine function $cvsin(z) = 1 - \sin(z)$
## @end deftypefn

function res = sf_cvsin(z)
  res = ones(size(z));
  for n = 1:prod(size(z));
    res(n) = sf_cvsin_1(z(n));
  endfor
endfunction

function res = sf_cvsin_1(z)
  persistent qpi4 = sf_qdouble("0.78539816339744830961566084581987572104929234984378");
  zz = (qpi4 - z/2).hi;
  res = 2*sf_sin(zz)^2;
  #res = 1 - sf_sin(z);
endfunction
