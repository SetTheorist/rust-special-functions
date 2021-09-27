## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gamma (@var{z})
## Compute Gamma function
## @end deftypefn

function res = sf_gamma(z)
  res = ones(size(z));
  for n = 1:prod(size(z))
    abz = abs(z(n));
    if (sf_is_nonposint(z(n)))
      res(n) = Inf;
    elseif (abz<35)
      if (abs(imag(z(n)))<2)
        res(n) = 1/sqrt(2*pi) * 2^(z(n)-1/2) * ln_series2(z(n)/2) * ln_series2(z(n)/2 + 1/2);
      elseif (abs(imag(z(n)))<=1)
        res(n) = ln_series2(z(n));
      else
        res(n) = spouge_approx(11, z(n));
      endif
    else
      res(n) = asympt(z(n));
    endif
  endfor
endfunction

# real(z)>0
function res = gamq(z)
  #K = 1;
  if (isreal(z))
    #res = (K)^z * quad(@(t)( t^(z-1)*sf_exp(-(K)*t) ), 0, inf, 1e-15);
    res = quad(@(t)( t^(z-1)*sf_exp(-t) ), 0, inf, 1e-15);
  else
    res = quad(@(t)(real( t^(z-1)*sf_exp(-t) )), 0, inf, 1e-16) ...
       +I*quad(@(t)(imag( t^(z-1)*sf_exp(-t) )), 0, inf, 1e-16);
  endif
endfunction

# very slowly convergent
function res = cx_mag(z)
  x = real(z); y = imag(z);
  res = 0.0;
  n = 0;
  do
    old_res = res;
    res += sf_log_p1((y/(x+n))^2);
    ++n;
    if (n>999) break; endif
  until (res == old_res)
  res = abs(gamma(x))*sf_exp(-0.5*res);
  n
endfunction

# only gets ~13 digits accuracy, even for large z ?!
function res = asympt(z)
  persistent bn = sf_bernoulli_number([1:250]);
  if (real(z)<0.5)
    res = pi / (sf_sin(pi*z) * asympt(1-z));
    return;
  endif

  res = 0.0;
  e_ = 0;
  # shift z upwards
  while (abs(z)<100)
    #res -= sf_log(z*(z+1));
    t_ = res;
    y_ = (-sf_log(z*(z+1))) + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    z += 2;
  endwhile

  term = (z-0.5)*sf_log(z) - z + 0.5*sf_log(2*pi);
  #res += term;
    t_ = res;
    y_ = term + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
  m = 2;
  do
    old_term = term;
    term = bn(m) / (m*(m-1)*z^(m-1));
    if (abs(term) > abs(old_term)) break; endif
    old_res = res;
    #res += term;
      t_ = res;
      y_ = term + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    m += 2;
    if (m>250) break; endif
  until (res == old_res)
  res = sf_exp(res)*sf_exp(e_);
endfunction

# for |z| < 2
function res = ln_series2(z)
  persistent zm = sf_zeta_m1(2:250);
  if (real(z)<0.5)
    res = pi / (sf_sin(pi*z) * ln_series2(1-z));
    return;
  endif

  e_ = 0.0;
  res = 0.0;
  while (real(z)>1.5)
    #res += sf_log(z-1);
    t_ = res;
    y_ = sf_log(z-1) + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    z -= 1;
  endwhile

  x = z-1;
  res += -sf_log(z) + 0.4227843350984671393934879099*x;
  term = -x;
  n = 2;
  do
    term *= -x;
    old_res = res;
    #res += term * sf_zeta_m1(n) / n;
    t_ = res;
    y_ = (term * zm(n-1) / n) + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    ++n;
    if (n>250) break; endif
  until (res == old_res)
  res += e_;
  res = sf_exp(res);
endfunction

function res = spouge_approx(a, z)
  # Spouge's approximation
  z -= 1;
  res = (z+a)^(z+1/2) * sf_exp(-(z+a));
  sm = sqrt(2*pi);
  e_ = 0;
  for k = 1 : (a-1)
    #sm += spouge_c(k,a) / (z+k);
    t_ = sm;
    y_ = (spouge_c(k,a) / (z+k)) + e_;
    sm = t_ + y_;
    e_ = (t_ - sm) + y_;
  endfor
  res *= sm;
  res += res * e_;
endfunction
function res = spouge_c(k,a)
  res = ((-1)^(k-1) / factorial(k-1)) * (-k+a)^(k-1/2) * sf_exp(-k + a);
endfunction
