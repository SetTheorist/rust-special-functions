## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_dilog (@var{z})
## Compute the dilogarithm $Di_2(z) = \sum_(n=1)^\infty z^n/n^2$
## $z \not\in [1,+\infty)$
## @end deftypefn
function res = sf_dilog(z)
  if (nargin < 1)
    print_usage;
  endif
  res = zeros(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_dilog_1(z(n));
  endfor
endfunction

function res = sf_dilog_1(z)
  # branch cut on positive real axis [1,+\infty)
  if (imag(z)==0 && z>=1) res = NaN; return; endif

  if (imag(z)==0 && z>1/2)
    # for efficiency for real values
    # L(z) + L(1-z) = pi^2/6 - (ln z)(ln 1-z)
    res = pi^2/6 - sf_log(z)*sf_log_p1(-z) - power_series(1-z);
  elseif (abs(z) <= 2/3)
    # direct series (radius of convergence for |z|<1)
    res = power_series(z);
  elseif (abs(z) >= 3/2)
    # reflect large values to small
    # L(z) + L(1/z) = -pi^2/6 - ln(-z)^2/2
    res = -pi^2/6 - sf_log(-z)^2/2 - power_series(1/z);
  else
    # clean up the rest
    # L(z) + L(z/(z-1)) = -ln(1-z)^2/2
    # and
    # L(z/(z-1)) + L((z-1)/z) = -pi^2/6 - ln(-z/(z-1))^2/2
    if (abs(z/(z-1))<1)
      res = -sf_log_p1(-z)^2/2 - power_series(z/(z-1));
    else
      res = -sf_log_p1(-z)^2/2 - ( -power_series((z-1)/z) - pi^2/6 - sf_log(-z/(z-1))^2/2 );
    endif
  endif
endfunction

function res = power_series(z)
  term = z;
  smm = res = res2 = term;
  e_ = 0;
  n = 2;
  lnz = sf_log(z);
  do
    term *= z;

    #smm += term / (n^2);
      t_ = smm;
      y_ = (term / (n^2)) + e_;
      smm = t_ + y_;
      e_ = (t_ - smm) + y_;

    old_res = res;
    if (true)
      res = smm;
    else
      # unfortunately, this EM summation is significantly slower, though it reduces the number of iterations needed
      persistent bns = sf_bernoulli_number_scaled(1:20);
      res = smm ...
          + sf_expint_en(-n*lnz, 2)/n ...
          - z^n / (2*n^2) ...
          - bns(2)*(lnz - 2/n)*z^n/n^2 ...
          - bns(4)*(lnz^3 - 6*lnz^2/n + 18*lnz/n^2 - 24/n^3)*z^n/n^2 ...
          - bns(6)*(lnz^5 - 10*lnz^4/n + 60*lnz/n^2 - 240*lnz^2/n^3 + 600*lnz/n^4 - 720/n^5)*z^n/n^2 ...
          +e_;
    endif

    ++n;
    if (n>999) break; endif
  until (res == old_res)
endfunction
