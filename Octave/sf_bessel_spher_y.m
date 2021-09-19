## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_spher_y (@var{n}, @var{z})
## Compute spherical Bessel y_n(z) function
## @end deftypefn

function res = sf_bessel_spher_y(n, z)
  if (nargin<2 || !all(sf_is_nonnegint(n))) print_usage; endif
  #wrp = sf_sqrt(pi./(2*z)) .* sf_bessel_y(n+1/2, z)
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    zz = z(kk);
    if (real(zz)<0)
      zz = -zz;
      mul = (-1)^(rem(n+1,2));
    else
      mul = 1;
    endif
    if (zz == 0)
      res(kk) = -Inf;
    elseif (n==0)
      res(kk) = mul*y0(zz);
    elseif (n==1)
      res(kk) = mul*y1(zz);
    else
      res(kk) = mul*fore(n, zz);
    endif
  endfor
endfunction

function res = y0(z)
  res = -sf_cos(z)/z;
endfunction

function res = y1(z)
  res = -sf_cos(z)/z^2 - sf_sin(z)/z;
endfunction

# unstable direction
function res = back(n, z)
  NN = 10;
  arr = zeros(n+1+NN, 1);
  arr(n+1+NN) = 0;
  arr(n+1+NN-1) = 1;
  for jj = (n+1+NN-2):(-1):1
    arr(jj) = (2*(jj-1)+3)/z*arr(jj+1) - arr(jj+2);
  endfor
  nnn = (0:(n+NN)) .';
  #scale = sqrt(sum((2*nnn+1) .* arr.^2));
  scale = arr(1) / y0(z);
  arr /= scale;
  res = arr(n+1);
endfunction

function res = fore(n, z)
  arr = zeros(n+1, 1);
  arr(1) = y0(z);
  arr(2) = y1(z);
  for jj = 2:n
    arr(jj+1) = (2*jj-1)/z*arr(jj) - arr(jj-1);
  endfor
  res = arr(n+1);
endfunction
