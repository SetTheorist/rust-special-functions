## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hypergeom_2f1 (@var{a}, @var{b}, @var{c}, @var{z})
## Compute the Gauss hypergeometric function 2F1(a,b;c;z).
## @end deftypefn

function res = sf_hypergeom_2f1(a, b, c, z)
  if (nargin<4) print_usage; endif
  rho = 0.9;
  if (abs(z) < rho)
    res = series(a, b, c, z);
    #res = (1-z)^(c-a-b)*series(c-a, c-b, c, z);
  elseif (abs(1/z) < rho)
    res = sf_gamma(c)*sf_gamma(b-a)/sf_gamma(b)/sf_gamma(c-a) * (-z)^(-a) * series(a, 1-c+a, 1-b+a, 1/z) ...
        + sf_gamma(c)*sf_gamma(a-b)/sf_gamma(a)/sf_gamma(c-b) * (-z)^(-b) * series(b, 1-c+b, 1-a+b, 1/z);
  elseif (abs(1-z) < rho)
    res = sf_gamma(c)*sf_gamma(c-a-b)/sf_gamma(c-a)/sf_gamma(c-b) * series(a, b, a+b-c+1, 1-z) ...
        + sf_gamma(c)*sf_gamma(a+b-c)/sf_gamma(a)/sf_gamma(b) * (1-z)^(c-a-b) * series(c-a, c-b, c-a-b+1, 1-z);
  elseif (abs(1/(1-z)) < rho)
    res = sf_gamma(c)*sf_gamma(b-a)/sf_gamma(b)/sf_gamma(c-a) * (1-z)^(-a) * series(a, c-b, a-b+1, 1/(1-z)) ...
        + sf_gamma(c)*sf_gamma(a-b)/sf_gamma(a)/sf_gamma(c-b) * (1-z)^(-b) * series(b, c-a, b-a+1, 1/(1-z));
  elseif (abs(z/(z-1)) < rho)
    res = (1-z)^(-a) * series(a, c-b, c, z/(z-1));
    #res = (1-z)^(-b) * series(c-a, b, c, z/(z-1));
  elseif (abs((z-1)/z) < rho)
    res = sf_gamma(c)*sf_gamma(c-a-b)/sf_gamma(c-a)/sf_gamma(c-b) * z^(-a) * series(a, a-c+1, a+b-c+1, 1-1/z) ...
        + sf_gamma(c)*sf_gamma(a+b-c)/sf_gamma(a)/sf_gamma(b) * z^(a-c)*(1-z)^(c-a-b) * series(c-a, 1-a, c-a-b+1, 1-1/z);
  else
    z0 = 1/2;
    res = sf_gamma(c)*sf_gamma(b-a)/sf_gamma(a)/sf_gamma(c-a) * (z0-z)^(-a) * buehring(a, b, c, z, a, z0) ...
        + sf_gamma(c)*sf_gamma(a-b)/sf_gamma(a)/sf_gamma(c-b) * (z0-z)^(-b) * buehring(a, b, c, z, b, z0);
  endif
endfunction

# for |z|<1, -c not a nonnegative integer
function res = series(a, b, c, z)
  res = term = 1.0;
  n = 1;
  e_ = 0;
  do
    term *= z / n * (a+n-1) / (c+n-1) * (b+n-1);
    old_res = res;
    #res += term;
      t_ = res;
      y_ = term + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    ++n; if (n>999) break; endif
  until (res == old_res)
  if (n>999) warning('sf:convergence', "sf_hypergeom_2f1 failed to converge"); endif
  n
endfunction

# for b-a non-integer, |ph(z0-z)|<pi
function res = buehring(a, b, c, z, s, z0)
  dn1 = 0;
  dn = 1;
  res = 1;
  n = 1;
  do
    dn2 = dn1; dn1 = dn;
    dn = (n+s-1)/(n*(n+2*s-a-b)) * ( z0*(1-z0)*(n+s-2)*dn2 + ((n+s)*(1-2*z0)+(a+b+1)*z0-c)*dn1 );
    old_res = res;
    res += dn * (z-z0)^(-n);
    ++n; if (n>999) break; endif
  until (res == old_res)
  if (n>999) warning('sf:convergence', "sf_hypergeom_2f1 failed to converge"); endif
  bn = n
endfunction

