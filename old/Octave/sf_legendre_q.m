## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_legendre_q (@var{nu}, @var{z}, [@var{m}=0])
## Evaluate (associated) Legendre functions of the second kind
## of order m (integer), degree nu at z, $Q^m_\nu(z)$.
## TODO: BUG FOR -ve m !!!!
## @end deftypefn
function res = sf_legendre_q(nu, z, m)
  if (nargin < 2) print_usage; endif
  if (nargin < 3) m = 0; endif

  if (!sf_is_int(m)) res = NaN; return; endif

  # symmetry to ensure degree nu>=0 
  if (nu<0)
    nu_adj = -pi*sf_cot(-nu*pi) * sf_legendre_p(-nu-1, z, m);
    nu = -nu-1;
  else
    nu_adj = 0.0;
  endif

  # symmetry to ensure order m>=0
  if (m<0)
    m_mult = (-1)^m * sf_gamma(nu+m+1) / sf_factorial(nu-m-1);
    m = -m;
  else
    m_mult = 1.0;
  endif

  if (sf_is_int(nu))
    res = sf_legendre_nu_integer(nu, z, m);
  else
    res = sf_legendre_nu_arbitrary(nu, z, m);
  endif

  res = m_mult*res + nu_adj;
endfunction

############################################################
function res = sf_legendre_nu_arbitrary(nu, z, m)
  if (isreal(z))
    if (z<=1)
      res = sf_legendre_nu_arbitrary_sum1(nu, z, m);
    elseif (z>=-1)
      res = sf_legendre_nu_arbitrary_sum2(nu, z, m);
    else
    endif
  else
  endif
endfunction

function res = sf_legendre_nu_arbitrary_sum1(nu, z, m)
  res = 1.0;
  zm2 = (1-z)/2;
  term = 1.0;
  k = 1;
  do
    term *= (m-nu + k-1)*(m+nu+1 + k-1) * zm2 / k / (m+1 + k-1);
    old_res = res;
    res += term;
    ++k;
    if (k>999) break; endif
  until (res == old_res);
  if (m!=0)
    res *= (-1)^m * (1-z^2)^(m/2) * sf_gamma(nu+m+1) / sf_gamma(nu-m+1) / (2^m * sf_factorial(m));
  endif
endfunction

############################################################
# n integer
function res = sf_legendre_nu_integer(n, z, m)
  if (m==0)
    #slow and inaccurate:
    #res = sf_legendre_nu_integer_m0_sum(n, z)
    if (n<=5) 
      res = sf_legendre_nu_integer_m0_special(n, z);
    else
      # need n>=2
      res = sf_legendre_nu_integer_m0_recur(n, z);
    endif
  else
    if (m>n)
      res = 0.0;
    else
      res = sf_legendre_nu_integer_many_recur(n, z, m);
    endif
  endif
endfunction

# integer n but m>0
function res = sf_legendre_nu_integer_many_recur(n, z, m)
  # compute Q_m,m
  qm2 = (-1)^m * (1-z^2)^(m/2) * prod(1:2:(2*m-1)) * sf_legendre_nu_integer_m0_special(0, z);
  if (n==m) res = qm2; return; endif
  # compute Q_m,(m+1)
  qm1 = (2*m+1)*z*qm2;
  if (n==m+1) res = qm1; return; endif
  # use recurrence
  for k = (m+2):n
    qm = ((2*k-1)*z*qm1 - (k+m-1)*qm2) / (k-m);
    qm2 = qm1;
    qm1 = qm;
  endfor
  res = qm;
endfunction

#integer n, m==0
function res = sf_legendre_nu_integer_m0_recur(n, z)
  qk2 = sf_legendre_nu_integer_m0_special(0, z);
  qk1 = sf_legendre_nu_integer_m0_special(1, z);
  for k = 2:n
    qk = ((2*k-1)*z*qk1 - (k-1)*qk2)/k;
    qk2 = qk1;
    qk1 = qk;
  endfor
  res = qk;
endfunction

#integer n, m==0
function res = sf_legendre_nu_integer_m0_sum(n, z)
  # direct summation
  res = sf_legendre_p(n, z) * (sf_log((1+z)/(1-z))/2 - sf_harmonic_number(n));
  for k = 1:n
    res += (-1)^k * sf_factorial(n+k) * sf_harmonic_number(k) * ((1-z)/2)^k / sf_factorial(k)^2 / sf_factorial(n-k);
  endfor
endfunction

#integer n<=5, m==0
function res = sf_legendre_nu_integer_m0_special(n, z)
  q0 = sf_log((1+z)/(1-z)) / 2;
  # special cases
  switch (n)
  case 0
    res = q0;
  case 1
    res = sf_legendre_p(1, z) * q0 - 1;
  case 2
    res = sf_legendre_p(2, z) * q0 - z*3/2;
  case 3
    res = sf_legendre_p(3, z) * q0 - z^2*5/2 + 2/3;
  case 4
    res =sf_legendre_p(4, z) * q0 - z*(z^2*35/8 - 55/24);
  case 5
    res = sf_legendre_p(5, z) * q0 - 8/15 + z^2*(49/8 - z^2*63/8);
  endswitch
endfunction
