## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_struve_h (@var{nu}, @var{z})
## Compute the Struve function HH_nu(z)
## @end deftypefn

function res = sf_struve_h(nu, z)
  if (nargin<2) print_usage(); endif
  if (any(size(nu)!=size(z)))
    if (isscalar(nu)) nu *= ones(size(z));
    elseif (isscalar(z)) z *= ones(size(nu));
    else error("sf_struve_h: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    #asy = struve_h_asympt(nu(kk), z(kk))
    res(kk) = struve_series_bes3(nu(kk), z(kk));
#    #asy_nu = z/(pi*nu(kk)*sqrt(2)) * (e*z/2/nu(kk))^nu(kk)
#    if (abs(z(kk))>25 && isreal(nu(kk)) && (!isreal(z(kk)) || z(kk)>0))
#      res(kk) = struve_h_asympt(nu(kk), z(kk));
#    elseif (abs(z)>5 && !sf_is_negint(nu(kk)))
#      res(kk) = struve_series_bes3(nu(kk), z(kk));
#      #res(kk) = struve_h_besseries(nu(kk), z(kk));
#      #res(kk) = struve_h_besseries_2(nu(kk), z(kk));
#    else
#      res(kk) = struve_h_series(nu(kk), z(kk));
#    endif
  endfor
endfunction

# this is actually slower than calling besselj many times (below) ... (!?)
function res = struve_series_bes3__2(nu, z)
  NBESS = 150;
  bj = zeros(1,NBESS);
  bj(end) = 0;
  bj(end-1) = 1;
  for k=(NBESS-2):(-1):1
    bj(k) = 2*(k+nu+1)/z*bj(k+1) - bj(k+2);
    if (bj(k)>1024) bj /= 1024; endif
  endfor
  bj *= besselj(nu+1, z)/bj(1);

  res = bj(1)*sf_gamma(nu+1)*(nu+1)/(2*nu+1);
  k = 1;
  term = sf_gamma(nu+1);
  do
    term *= (k+nu)/k;
    old_res = res;
    res += bj(2*k+1)*term*(2*k+nu+1)/(2*k+1)/(2*k+2*nu+1);
    ++k; if (2*k>NBESS) break; endif
  until (res == old_res)
  res *= 4/sf_sqrt(pi)/sf_gamma(nu+1/2);
endfunction

# so far THIS seems to be the most effective (except for very large z~nu...)
function res = struve_series_bes3(nu, z)
  res = besselj(nu+1, z)*sf_gamma(nu+1)*(nu+1)/(2*nu+1);
  k = 1;
  term = sf_gamma(nu+1);
  do
    term *= (k+nu)/k;
    old_res = res;
    res += besselj(2*k+nu+1, z)*term*(2*k+nu+1)/(2*k+1)/(2*k+2*nu+1);
    ++k; if (k>999) break; endif
  until (res == old_res)
  res *= 4/sf_sqrt(pi)/sf_gamma(nu+1/2);
endfunction

# for large |z|, fixed nu
function res = struve_h_asympt(nu, z)
  term = res = sqrt(pi) * (z/2)^(nu-1) / sf_gamma(nu+1/2);
  k = 1;
  do
    old_term = term;
    term = sf_gamma(k+1/2) * (z/2)^(nu-2*k-1) / sf_gamma(nu+1/2 - k);
    if (abs(term) > abs(old_term)) break; endif
    old_res = res;
    res += term;
    ++k;
    if (k>999) break; endif
  until (res == old_res);
  #ask = k
  res /= pi;
  res += sf_bessel_y(nu, z);
endfunction


function res = recur_backwards(nu, z, NN)
  jj = zeros(1,NN+10);
  jj(end-1) = 1;
  jj_e_ = zeros(1,NN+10);
  jj_e_(end) = 1e-40;
  jj_e_(end-1) = 1e-20;
  for m = (NN+10-2):(-1):1
    nu_m = nu + m+1;
    #jj(m) = (2*(nu_m-1)/z)*jj(m+1) - jj(m+2);
    s_ = -jj(m+2);
    e_ = -jj_e_(m+2);
    # add high
      t_ = s_;
      y_ = (2*(nu_m-1)/z)*jj(m+1) + e_;
      s_ = t_ + y_;
      e_ = (t_ - s_) + y_;
    # add low
      t_ = s_;
      y_ = ((2*nu_m-1)/z)*jj_e_(m+1) + e_;
      s_ = t_ + y_;
      e_ = (t_ - s_) + y_;
    jj(m) = s_;
    jj_e_(m) = e_;
    if (s_>1e6)
      jj /= 1048576;
      jj_e_ /= 1048576;
    endif
  endfor
  #jj
  #scale = sum([jj(1)^2, jj_e_(1)^2, 2*jj(2:end).^2, 2*jj_e_(2:end).^2], 'extra');
  #res = jj/sqrt(scale);
  scale = jj(1)/sf_bessel_j(nu, z);
  res = jj/scale;
endfunction

function res = struve_h_besseries_2(nu, z)
  res =0.0;
  k = 0;
  # build bessel sequence via (backwards recursion)
  NJJ = 51 + floor(log(1+abs(z))*50); # empirical kludge; TODO: think and get motivated count
  NK = (NJJ-1)/2;
  kk = 0:NK;
  jj = recur_backwards(nu+1, z, NJJ);
  # vectorized summation
  trms = jj(2*kk+1) .* (2*kk+nu+1) ./ (2*kk+1) ./ (2*kk+2*nu+1);
  gg = sf_gamma(nu+1) * cumprod([1,kk(2:end)+nu] ./ [1,kk(2:end)]);
  res = 4 / sqrt(pi) / sf_gamma(nu+1/2) * sum(gg .* trms, 'extra');
endfunction

function res = struve_h_besseries(nu, z)
  res = 0.0;
  k = 0;
  do
    old_res = res;
    res += (2*k+nu+1)*sf_gamma(k+nu+1) / sf_factorial(k) / (2*k+1) / (2*k + 2*nu + 1) * sf_bessel_j(2*k+nu+1, z);
    ++k; if (k>999) break; endif
  until (res == old_res)
  #k
  res *= 4 / sqrt(pi) / sf_gamma(nu + 1/2);
endfunction

function res = struve_h_series(nu, z)
  res = term = 1.0 / (sf_gamma(3/2) * sf_gamma(nu + 3/2));
  n = 0;
  z2 = -(z/2)^2;
  do
    term *= z2 / ((3/2 + n)*(3/2 + nu + n));
    old_res = res;
    res += term;
    ++n; if (n>999) break; endif
  until (res == old_res)
  #n
  res *= (z/2)^(nu+1);
endfunction
