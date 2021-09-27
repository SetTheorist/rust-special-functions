## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},...,@var{rn}] =} sf_nthroot (@var{n}, @var{z})
## Compute the nth-root(s) of $z$, $z^(1/n)$
## ($n=1,2,3,...$)
## @end deftypefn
function varargout = sf_nthroot(n, z)
  if (nargin<2 || nargout>n) print_usage; endif
  if (any(size(n)!=size(z)))
    if (isscalar(n)) n *= ones(size(z));
    elseif (isscalar(z)) z *= ones(size(n));
    else error("sf_nthroot: mismatched parameter sizes");
    endif
  endif
  res = ones(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_nthroot_1(n(kk), z(kk));
  endfor
  varargout{1} = res;
  if (nargout > 1)
    root_of_unity = sf_exp(I*2*pi/n);
    for kk = 2:nargout
      varargout{kk} = res * sf_exp(I*2*pi*(kk-1)/n);
    endfor
  endif
endfunction

function res = sf_nthroot_1(n, z)
  if (z==0.0)
    res = 0.0;
  elseif (isreal(z))
    # real z
    res = real_newton(n, z);
  else
    # fully complex z
    # copout approach...
    r = abs(z);
    th = arg(z);
    res = real_newton(n, r) * sf_exp(I*th/n);
  endif
endfunction

# newton method
function res = real_newton(n, x)
  sgn = sign(x); x = abs(x);
  res = 1 + (x-1)/n;
  k = 0;
  old_res = 0;
  do
    old_old_res = old_res;
    old_res = res;
    res = (n-1)*res/n + x/(n*res^(n-1));
    ++k;
    if (k>999) break; endif
  until (res == old_res) || (res == old_old_res);
  res *= sgn^(rem(n,2));
endfunction
