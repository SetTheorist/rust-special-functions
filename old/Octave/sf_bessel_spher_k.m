## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_spher_k (@var{n}, @var{z})
## Compute spherical Bessel k_n(z) function
## @end deftypefn

function res = sf_bessel_spher_k(n, z)
  if (nargin<2 || !all(sf_is_nonnegint(n))) print_usage; endif
  #frm = sf_sqrt(pi./(2*z)) .* sf_bessel_k(n+1/2, z)
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    zz = z(kk);
    if (real(zz)<0)
      res(kk) = -pi/2 * (sf_bessel_spher_i1(n, -zz) + sf_bessel_spher_i2(n, -zz));
    elseif (zz==0)
      res(kk) = Inf;
    elseif (n==0)
      res(kk) = k0(zz);
    elseif (n==1)
      res(kk) = k1(zz);
    else
      res(kk) = fore(n, zz);
    endif
  endfor
endfunction

#
function res = k0(z)
  res = pi/2 * sf_exp(-z) * (1/z);
endfunction

# 
function res = k1(z)
  res = pi/2 * sf_exp(-z) * (1/z + 1/z^2);
endfunction

function res = back(n, z)
  NN = 10;
  arr = zeros(n+1+NN, 1);
  arr(n+1+NN) = 0;
  arr(n+1+NN-1) = 1;
  for jj = (n+1+NN-2):(-1):1
    arr(jj) = (2*(jj-1)+3)/z*arr(jj+1) + arr(jj+2);
  endfor
  nnn = (0:(n+NN)) .';
  #scale = sqrt(sum((2*nnn+1) .* arr.^2));
  scale = arr(1) / k0(z);
  arr /= scale;
  res = (-1)^(rem(n,2)) * arr(n+1);
endfunction

function res = fore(n, z)
  arr = zeros(n+1, 1);
  arr(1) = k0(z);
  arr(2) = -k1(z);
  for jj = 2:n
    arr(jj+1) = -(2*jj-1)/z*arr(jj) + arr(jj-1);
  endfor
  res = (-1)^(rem(n,2)) * arr(n+1);
endfunction
