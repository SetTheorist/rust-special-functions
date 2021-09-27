## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_j (@var{nu}, @var{z})
## Compute Bessel J_nu(z) function -- just a wrapper of built-in besselj()
## @end deftypefn

function res = sf_bessel_j(nu, z)
  #asy = asympt_z(z, nu);
  #ser = series(z, nu);
  sys = besselj(nu,z);
  #rec = recur_back(z, nu);
  #ref = recur_fore(z, nu);
  #re2 = recur_backwards(nu, z, round(abs(max(z, nu)))+21);
  res = sys;
endfunction

# for |z|>>nu
# for |arg(z)|<pi
function res = asympt_z(z, nu)
  chi = z - (nu/2 + 1/4)*pi;
  mu = 4*nu^2;
  res = sqrt(2/(pi*z)) * (asymp_p(z,nu)*sf_cos(chi) - asymp_q(z,nu)*sin(chi));
endfunction
function res = asymp_p(z, nu)
  mu = 4*nu^2;
  res = term = 1;
  z8 = -(8*z)^2;
  k = 1;
  do
    old_term = term;
    term *= (mu-(2*k-1)^2) * (mu-(2*k+1)^2) / ((2*k-1)*(2*k)*z8);
    old_res = res;
    res += term;
    ++k;
    if (k>999) break; endif
  until (res == old_res) || (abs(term) > abs(old_term))
endfunction
function res = asymp_q(z, nu)
  mu = 4*nu^2;
  res = term = (mu-1)/(8*z);
  z8 = -(8*z)^2;
  k = 2;
  do
    old_term = term;
    term *= (mu-(2*k-1)^2) * (mu-(2*k+1)^2) / ((2*k-2)*(2*k-1)*z8);
    old_res = res;
    res += term;
    ++k;
    if (k>999) break; endif
  until (res == old_res) || (abs(term) > abs(old_term))
endfunction

# straightforward power-series
function res = series(z, nu)
  res = term = 1.0;
  z2 = -(z/2)^2;
  m = 1;
  do
    term *= z2 / (m * (m + nu));
    old_res = res;
    res += term;
    ++m;
    if (m>999) break; endif
  until (res == old_res)
  res *= (z/2)^nu / sf_gamma(nu+1);
endfunction

# recursion in order (backwards)
function res = recur_back(z, nu)
  N = fix(nu);
  nuf = nu - N;
  Nx = N + 10;
  jj = zeros(Nx, 1);
  jj(Nx) = 0.0;
  jj(Nx-1) = 1.0;
  for j = (Nx-2):(-1):1
    jj(j) = jj(j+1)*2*(nuf+j)/z - jj(j+2);
  endfor
  if (abs(z)<10)
    scale = series(z, nuf);
  else
    scale = asympt_z(z, nuf);
  endif
  jj *= scale / jj(1);
  res = jj(N+1);
endfunction

# recursion in order (forewards)
function res = recur_fore(z, nu)
  N = fix(nu);
  nuf = nu - N;
  Nx = N + 10;
  jj = zeros(Nx, 1);
  jj(1) = series(z, nuf);
  jj(2) = series(z, nuf+1);
  for j = 3:Nx
    jj(j) = jj(j-1)*2*(nuf+j)/z - jj(j-2);
  endfor
  jj
  res = jj(N+1);
endfunction

function res = recur_backwards(n, z, topper)
  jjp2 = zeros(size(z));
  jjp1 = ones(size(z));
  jjp2_e_ = 1e-40 * ones(size(z));
  jjp1_e_ = 1e-20 * ones(size(z));
  scale = 2*ones(size(z));
  res = zeros(size(z));
  for m = (topper-2):(-1):1
    #jj(m) = (2*nu/z)*jj(m+1) - jj(m+2);
    s_ = -jjp2;
    e_ = -jjp2_e_;
    # add high
      t_ = s_;
      y_ = ((2*m./z).*jjp1) + e_;
      s_ = t_ + y_;
      e_ = (t_ - s_) + y_;
    # add low
      t_ = s_;
      y_ = ((2*m./z).*jjp1_e_) + e_;
      s_ = t_ + y_;
      e_ = (t_ - s_) + y_;
    jjp2 = jjp1;
    jjp2_e_ = jjp1_e_;
    jjp1 = s_;
    jjp1_e_ = e_;

    if (m==n+1)
      # store the desired result,
      # but keep recursing to get scale factor
      res = jjp1;
    endif

    if (m!=1)
      scale += 2 * (s_.^2 + e_.^2 + 2*s_.*e_);
    else
      scale += 1 * (s_.^2 + e_.^2 + 2*s_.*e_);
    endif

    if (scale>1e20)
      jjp2 /= 1024;
      jjp2_e_ /= 1024;
      jjp1 /= 1024;
      jjp1_e_ /= 1024;
      res /= 1024;
      scale /= 1024^2;
    endif
  endfor
  res ./= sqrt(scale);
endfunction
