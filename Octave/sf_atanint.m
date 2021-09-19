## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_atanint (@var{x})
## Compute the arctangent-integral $ataint(x)=\int_0^x \atan(t)/t dt$
## for real x
## @end deftypefn

function res = sf_atanint(z)
  if (nargin<1 || !isreal(z)) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    #qua = quad(@(t)(sf_atan(t)/t), 0, z(kk))
    #cvz = sers_cvz(z(kk))
    if (abs(z(kk)-1) <= 0.02)
      if (z(kk)<0) sgn=-1; zz=-z(kk); else sgn=1; zz=z(kk); endif
      res(kk) = sgn*serat1(zz);
    elseif (abs(z(kk))<1)
      #boo = sers_bool(z(kk))
      res(kk) = sers(z(kk));
    else
      res(kk) = bigser(z(kk));
    endif
  endfor
endfunction

# |z|~1
function res = serat1(z)
  z1 = z - 1;
  res = ...
      + (3924 - 1260*pi) * z1^8 / 40320 ...
      + (180*pi - 567)   * z1^7 / 5040 ...
      + (97 - 30*pi)     * z1^6 / 720 ...
      + (6*pi - 20)      * z1^5 / 120 ...
      + (5 - 3*pi/2)     * z1^4 / 24 ...
      + (pi/2 - 3/2)     * z1^3 / 6 ...
      + (1/2 - pi/4)     * z1^2 / 2 ...
      + pi/4 * z1 ...
      + 0.91596559417721901505460351493238411077414937428167 ...
      ;
endfunction

# |z|>=1
# Euler-Maclaurin correction, not clear if it's worth the extra computation time for exponential integral
function res = bigser_em(z)
  if (real(z)<0) sgn=-1; z=-z; else sgn=1; endif
  lnz = sf_log(z);
  res = smm = (pi/2) * lnz;
  n = 0;
  persistent bns = sf_bernoulli_number_scaled(1:20);
  do
    smm += z^(-4*n-1) / (4*n+1)^2 - z^(-4*n-3) / (4*n+3)^2;
    old_res = res;
    res = smm ...
        + 1/(4*z) * sf_expint_en((4*n+1)*lnz, 2) / (4*n+1) ...
            - z^(-4*n-1) / (4*n+1)^2 ...
            + bns(2) * z^(-4*n-1) * ((4*n+1)*lnz + 2)/(4*n+1)^3 ...
            + bns(4)*64 * z^(-4*n-1) * ((4*n+1)^3*lnz^3 + 6*(4*n+1)^2*lnz^2 + 18*(4*n+1)*lnz + 24)/(4*n+1)^5 ...
        - 1/(4*z) * sf_expint_en((4*n+3)*lnz, 2) / (4*n+3) ...
            + z^(-4*n-3) / (4*n+3)^2 ...
            - bns(2) * z^(-4*n-3) * ((4*n+3)*lnz + 2)/(4*n+3)^3 ...
            - bns(4)*64 * z^(-4*n-3) * ((4*n+3)^3*lnz^3 + 6*(4*n+3)^2*lnz^2 + 18*(4*n+3)*lnz + 24)/(4*n+3)^5 ...
            ;
    ++n; if (n>999) break; endif
  until (res == old_res)
  res *= sgn;
endfunction

# |z|>=1
function res = bigser(z)
  if (real(z)<0) sgn=-1; z=-z; else sgn=1; endif
  res = (pi/2) * sf_log(z);
  n = 0;
  do
    old_res = res;
    res += (-1)^(rem(n,2)) * z^(-2*n-1) / (2*n+1)^2;
    ++n; if (n>999) break; endif
  until (res == old_res)
  res *= sgn;
endfunction

# for |z|=<1, z!=+/-I
function res = sers(z)
  res = z;
  term = z;
  n = 1;
  z2 = -z^2;
  do
    term *= z2;
    old_res = res;
    res += term/((2*n+1)^2);
    ++n; if (n>999) break; endif
  until (res == old_res);
  #n
endfunction

# for |z|=<1, z!=+/-I
# with Boole's summation formula
function res = sers_bool(z)
  res = smm = term = z;
  n = 1;
  z2 = -z^2;
  ens = sf_euler_number_scaled(1:10);
  lnz = sf_log(z);
  do
    term *= z2;
    smm += term/((2*n+1)^2);
    old_res = res;
    res = smm + (-1)^(rem(n,2)+1) * 0.5 * z^(2*n+2) * ( ...
        + 1 / (4*(n+1)^2) ...
        + 0*ens(2) * (3 - 4*(n+1)*lnz + 2*(n+1)^2*lnz^2) / (2*(n+1)^4) ...
        + 0*ens(4) * 2*(15 - 24*(n+1)*lnz + 18*(n+1)^2*lnz^2 - 8*(n+1)^3*lnz^3 + 2*(n+1)^4*lnz^4) / (n+1)^6 ...
        );
    ++n; if (n>999) break; endif
  until (res == old_res);
  #boo_n = n
endfunction

# for |z|=<1, z!=+/-I
# C-Z-V sequence acceleration
function res = sers_cvz(z)
  N = 50;
  d = (3+sqrt(8))^N;
  d = (d + 1/d)/2;
  b = -1;
  c = -d;
  s = 0;
  for k = 0:(N-1)
    c = b - c;
    s += c*z^(2*k+1)/(2*k+1)^2;
    b *= (k+N) * (k-N) / (k+1/2) / (k+1);
  endfor
  res = s/d;
endfunction


