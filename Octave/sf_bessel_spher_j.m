## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_spher_j (@var{n}, @var{z})
## Compute spherical Bessel j_n(z) function
## @end deftypefn

function res = sf_bessel_spher_j(n, z)
  if (nargin<2 || !all(sf_is_nonnegint(n))) print_usage; endif
  #sf_sqrt(pi./(2*z)) .* sf_bessel_j(n+1/2, z)
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
      res(kk) = mul*j0(zz);
    elseif (n==1)
      res(kk) = mul*j1(zz);
    elseif (abs(zz)>=n)
      res(kk) = mul*fore(n, zz);
    else
      res(kk) = mul*back(n, zz);
    endif
  endfor
endfunction

# sin(z)/z
function res = j0(z)
  if (abs(z)<0.5)
    NN = 24;
    terms = (-z^2)*ones(1,NN) ./ (2*(1:NN) .* (1+2*(1:NN)));
    res = sum(cumprod([1, terms]), 'extra');
    #res = sum(cumprod([1, terms]));
  else
    res = sf_sin(z)/z;
  endif
endfunction

# sin(z)/z^2 - cos(z)/z
function res = j1(z)
  if (abs(z)<0.5)
    NN = 24;
    terms = (-z^2)*ones(1,NN) ./ (2*(1:NN) .* (2*(1:NN)+1), 'extra');
    #terms = (-z^2)*ones(1,NN) ./ (2*(1:NN) .* (2*(1:NN)+1));
    res = sum(cumprod([z, terms])./(2*(0:NN)+3));
  else
    res = sf_sin(z)/z^2 - sf_cos(z)/z;
  endif
endfunction

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
  scale = arr(1) / j0(z);
  arr /= scale;
  res = arr(n+1);
endfunction

function res = fore(n, z)
  arr = zeros(n+1, 1);
  arr(1) = j0(z);
  arr(2) = j1(z);
  for jj = 2:n
    arr(jj+1) = (2*jj-1)/z*arr(jj) - arr(jj-1);
  endfor
  res = arr(n+1);
endfunction
