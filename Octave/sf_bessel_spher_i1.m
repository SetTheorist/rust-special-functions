## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_spher_i1 (@var{n}, @var{z})
## Compute spherical Bessel i^(1)_n(z) function
## @end deftypefn

function res = sf_bessel_spher_i1(n, z)
  if (nargin<2 || !all(sf_is_nonnegint(n))) print_usage; endif
  #frm = sf_sqrt(pi./(2*z)) .* sf_bessel_i(n+1/2, z)

  res = zeros(size(z));
  for kk = 1:prod(size(z))
    zz = z(kk);
    if (real(zz)<0)
      zz = -zz;
      mul = (-1)^(rem(n,2));
    else
      mul = 1;
    endif
    if (zz==0)
      if (n==0) res(kk) = 1;
      else      res(kk) = 0;
      endif
    elseif (n==0)
      res(kk) = mul*i1_0(zz);
    elseif (n==1)
      res(kk) = mul*i1_1(zz);
    else
      if (abs(zz)>n)
        res(kk) = mul*fore(n, zz);
      else
        res(kk) = mul*back(n, zz);
      endif
    endif
  endfor
endfunction

# sinh(z)/ z
function res = i1_0(z)
  # testing indicates no need for this
  #if (abs(z)<0.5)
  #  NN = 24;
  #  terms = z^2 * 1 ./ (2*(1:NN)) ./ (2*(1:NN)+1);
  #  res = sum(cumprod([1,terms]));
  #else
    res = sf_sinh(z)/z;
  #endif
endfunction

#  -sinh(z)/z^2 + cosh(z)/z
function res = i1_1(z)
  if (abs(z)<0.5)
    # mostly to get correct rounding
    NN = 24;
    terms = z^2 * 1 ./ (2*(1:NN)) ./ (2*(1:NN)+1);
    res = sum(cumprod(terms) .* (2*(1:NN)))/z;
  else
    res = -sf_sinh(z)/z^2 + sf_cosh(z)/z;
  endif
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
  scale = arr(1) / i1_0(z);
  arr /= scale;
  res = arr(n+1);
endfunction

function res = fore(n, z)
  arr = zeros(n+1, 1);
  arr(1) = i1_0(z);
  arr(2) = i1_1(z);
  for jj = 2:n
    arr(jj+1) = -(2*jj-1)/z*arr(jj) + arr(jj-1);
  endfor
  res = arr(n+1);
endfunction
