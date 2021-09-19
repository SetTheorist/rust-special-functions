## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_dirichlet_beta (@var{z})
## Compute Dirichlet beta function $\beta(z)$
## @end deftypefn
function res = sf_dirichlet_beta(z)
  if (nargin<1)
    print_usage;
  endif
  res = ones(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_dirichlet_beta_1(z(n));
  endfor
endfunction

function res = sf_dirichlet_beta_1(z)
#  # use Hurwitz zeta function
#  if (z==1.0)
#    res = pi/4
#  else
#    res = (sf_hurwitz_zeta(z,1/4) - sf_hurwitz_zeta(z,3/4))*4^(-z);
#  endif

  # Reflection
  if (z<0)
    z = 1 - z;
    mult = (pi/2)^z / (sf_gamma(z) * sf_sin(z * pi/2));
  else
    mult = 1.0;
  endif

#  # Direct sum
#  res = 1.0;
#  n = 1;
#  do
#    old_res = res;
#    res += (-1)^n * (2*n+1)^(-z);
#    ++n;
#    if (n>999) break; endif
#  until (res == old_res)
#  n
#  res *= mult;

  # Euler-Maclaurin sum (self computed)
  res = sm = 1 - 3^(-z);
  e_ = 0;
  n = 2;
  persistent bns = sf_bernoulli_number_scaled(1:12);
  term1 = bns( 2)*(-4)^1 *z;
  term2 = bns( 4)*(-4)^3 *z*(z+1)*(z+2);
  term3 = bns( 6)*(-4)^5 *z*(z+1)*(z+2)*(z+3)*(z+4);
  term4 = bns( 8)*(-4)^7 *z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6);
  term5 = bns(10)*(-4)^9 *z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8);
  term6 = bns(12)*(-4)^11*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)*(z+9)*(z+10);
  do
    #sm += (2*n+1)^(-z) - (2*n+3)^(-z);
      t_ = sm;
      y_ = (2*n+1)^(-z) + e_;
      sm = t_ + y_;
      e_ = (t_ - sm) + y_;
      t_ = sm;
      y_ = -(2*n+3)^(-z) + e_;
      sm = t_ + y_;
      e_ = (t_ - sm) + y_;
    old_res = res;
    res = sm ...
          + ((2*n + 1)^(1-z) - (2*n + 3)^(1-z))/(4*(z-1)) ...
          - ((2*n+1)^(-z) - (2*n+3)^(-z))/2 ...
          - term1*((2*n+1)^(-z-1) - (2*n+3)^(-z-1)) ...
          - term2*((2*n+1)^(-z-3) - (2*n+3)^(-z-3)) ...
          - term3*((2*n+1)^(-z-5) - (2*n+3)^(-z-5)) ...
          - term4*((2*n+1)^(-z-7) - (2*n+3)^(-z-7)) ...
          - term5*((2*n+1)^(-z-9) - (2*n+3)^(-z-9)) ...
          - term6*((2*n+1)^(-z-11) - (2*n+3)^(-z-11)) ...
          +e_;
    n += 2;
    if (n>999) break; endif
  until (res == old_res)

#  # Euler-Maclaurin sum (from Atlas)
#  res = sm = 1.0;
#  e_ = 0;
#  n = 1;
#  em1 = sf_bernoulli_number_scaled(1);
#  em2 = 2^( 2-1)*(2^2  - 1)*z*sf_bernoulli_number_scaled(2);
#  em3 = 2^( 4-1)*(2^4  - 1)*z*(z+1)*(z+2)*sf_bernoulli_number_scaled(4);
#  em4 = 2^( 6-1)*(2^6  - 1)*z*(z+1)*(z+2)*(z+3)*(z+4)*sf_bernoulli_number_scaled(6);
#  em5 = 2^( 8-1)*(2^8  - 1)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*sf_bernoulli_number_scaled(8);
#  em6 = 2^(10-1)*(2^10 - 1)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)*sf_bernoulli_number_scaled(10);
#  em7 = 2^(12-1)*(2^12 - 1)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)*(z+9)*(z+10)*sf_bernoulli_number_scaled(12);
#  em8 = 2^(14-1)*(2^14 - 1)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)*(z+9)*(z+10)*(z+11)*(z+12)*sf_bernoulli_number_scaled(14);
#  do
#    #sm += (-1)^n * (2*n+1)^(-z);
#      t_ = sm;
#      y_ = ((-1)^n * (2*n+1)^(-z)) + e_;
#      sm = t_ + y_;
#      e_ = (t_ - sm) + y_;
#    old_res = res;
#    res = sm ...
#        + em1 / (2*n+1)^(z) ...
#        + em2 / (2*n+1)^(1+z) ...
#        + em3 / (2*n+1)^(3+z) ...
#        + em4 / (2*n+1)^(5+z) ...
#        + em5 / (2*n+1)^(7+z) ...
#        + em6 / (2*n+1)^(9+z) ...
#        + em7 / (2*n+1)^(11+z) ...
#        + em8 / (2*n+1)^(13+z) ...
#        +e_;
#    ++n;
#    if (n>999) break; endif
#  until (res == old_res)
#  res *= mult;

#  # Quadrature
#  if (isreal(z))
#    res = quad(@(t)( t^(z-1)*sf_sech(t) ), 0, Inf);
#  else
#    res = quad(@(t)(real( t^(z-1)*sf_sech(t) )), 0, Inf) + I*quad(@(t)(imag( t^(z-1)*sf_sech(t) )), 0, Inf);
#  endif
#  res /= 2 * sf_gamma(z);

endfunction
