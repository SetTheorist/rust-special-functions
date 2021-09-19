## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_erfc (@var{z})
## Compute complementary error-function, $1-\erf(z)$
## @end deftypefn

function res = sf_erfc(z)
  if (nargin<1) print_usage; endif
  res = zeros(size(z));
  for k=1:prod(size(z))
    abz = abs(z(k));
    if (abz<1 || (abz<5 && real(z(k))==0))
      # not clear which is preferable...
      res(k) = erfc_series2(z(k));
      #res(k) = erfc_series(z(k));
    elseif (abz<2)
      res(k) = erfc_contfrac_l(z(k));
    elseif (abz>15)
      res(k) = erfc_asympt(z(k));
      if (real(z)==0) res(k) = 1 + I*imag(res(k)); endif
    else
      res(k) = erfc_contfrac2_l(z(k));
      if (real(z)==0) res(k) = 1 + I*imag(res(k)); endif
    endif
  endfor
endfunction

# only for case z = I*y
function res = erfc_imag_quad(z)
  persistent qpi = 1.772453850905516027298167483;
  res = quad(@(t)( sf_exp(t^2) ), 0, imag(z), 1e-16) * 2/qpi;
endfunction

function res = erfc_asympt(z)
  persistent qpi = 1.772453850905516027298167483;
  if (real(z)<0)
    res = 2 - erfc_asympt(-z);
    return;
  endif
  res = 1;
  term = 1;
  m = 1;
  z2 = 2*z^2;
  do
    old_term = term;
    term *= -(2*m-1)/z2;
    if (abs(term) > abs(old_term)) break; endif
    old_res = res;
    res += term;
    ++m;
    if (m>999) break; endif
  until (res == old_res)
  #m
  res *= sf_exp(-z^2) / qpi / z;
endfunction

function res = erfc_series2(z)
  persistent tqp = 2.0 / sf_sqrt(pi);
  NN = 32 + 48 * floor(abs(z));
  #res = z*sum(cumprod([1, (2*z^2)*ones(1,NN)]./(2*(0:NN)+1)), 'extra');
  #res = 1 - res*sf_exp(-z^2)*tqp;
  res = sum([1, -sf_exp(-z^2)*tqp*z*cumprod([1, (2*z^2)*ones(1,NN)]./(2*(0:NN)+1))], 'extra');
  #res = sum([1, -sf_exp(-z^2)*tqp*z*cumprod([1, (2*z^2)*ones(1,NN)]./(2*(0:NN)+1))]);
endfunction

function res = erfc_series(z)
  res = z;
  term = z;
  n = 1;
  z2 = 2*z^2;
  e_ = 0;
  do
    term *= z2 / (2*n+1);
    old_res = res;
    #res += term;
    t_ = res;
    y_ = term + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    ++n;
    if (n>999) break; endif
  until (res == old_res)
  #n
  persistent tqp = 2.0 / sf_sqrt(pi);
  res *= sf_exp(-z^2) * tqp;
  e_ *= sf_exp(-z^2) * tqp;
  res = (1 - res) - e_;
endfunction

function res = erfc_contfrac2_l(z)
  persistent qpi = 1.772453850905516027298167483;
  if (real(z)<0)
    res = 2 - erfc_contfrac2_l(-z);
    return;
  endif
  eps = 1e-16;
  zeta = 1e-150;
  z2 = 2*z^2;
  fj = z2 + 1; if (fj==0) fj=zeta; endif
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    aj = -(2*j)*(2*j-1);
    bj = z2 + 4*j + 1;
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j;
    if (j>999) break; endif
  until (abs(Deltaj - 1) < eps)
  #j
  res = sf_exp(-z^2)*2*z/fj/qpi;
endfunction

function res = erfc_contfrac2(z)
  persistent qpi = 1.772453850905516027298167483;
  if (real(z)<0)
    res = 2 - erfc_contfrac2(-z);
    return;
  endif
  z2 = 2*z^2;
  res = 2*z/(z2+1 ...
        - ( 1* 2)/(z2+ 5 - ( 3* 4)/(z2+ 9 ...
        - ( 5* 6)/(z2+13 - ( 7* 8)/(z2+17 ...
        - ( 9*10)/(z2+21 - (11*12)/(z2+25 ...
        - (13*14)/(z2+29 - (15*16)/(z2+33 ...
        - (17*18)/(z2+37 - (19*20)/(z2+41 ...
        - (21*22)/(z2+45 - (23*24)/(z2+49 ...
        - (25*26)/(z2+53 - (27*28)/(z2+57 ...
        - (29*30)/(z2+61 - (31*32)/(z2+65 ...
        - (33*34)/(z2+69 - (35*36)/(z2+73 ...
        - (37*38)/(z2+77 - (39*40)/(z2+81 ...
        )))))))))))))))))))));
  res *= sf_exp(-z^2)/qpi;
endfunction

function res = erfc_contfrac_l(z)
  persistent qpi = 1.772453850905516027298167483;
  if (real(z)<0)
    res = 2 - erfc_contfrac_l(-z);
    return;
  endif
  eps = 1e-16;
  zeta = 1e-150;
  z2 = z^2;
  fj = z2; if (fj==0) fj=zeta; endif
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    aj = j/2;
    if (rem(j,2)==0)
      bj = z2;
    else
      bj = 1;
    endif
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j;
    if (j>999) break; endif
  until (abs(Deltaj - 1)<eps)
  #j
  res = sf_exp(-z^2)*z/fj/qpi;
endfunction

# works better and better for large values
function res = erfc_contfrac(z)
  persistent qpi = 1.772453850905516027298167483;
  if (real(z)<0)
    res = 2 - erfc_contfrac(-z);
    return;
  endif
  z2 = z^2;
  res = z/(z2 ...
      + ( 1/2)/(1 +  1/(z2 ...
      + ( 3/2)/(1 +  2/(z2 ...
      + ( 5/2)/(1 +  3/(z2 ...
      + ( 7/2)/(1 +  4/(z2 ...
      + ( 9/2)/(1 +  5/(z2 ...
      + (11/2)/(1 +  6/(z2 ...
      + (13/2)/(1 +  7/(z2 ...
      + (15/2)/(1 +  8/(z2 ...
      + (17/2)/(1 +  9/(z2 ...
      + (19/2)/(1 + 10/(z2 ...
      + (21/2)/(1 + 11/(z2 ...
      + (23/2)/(1 + 12/(z2 ...
      + (25/2)/(1 + 13/(z2 ...
      + (27/2)/(1 + 14/(z2 ...
      + (31/2)/(1 + 15/(z2 ...
      + (35/2)/(1 + 16/(z2 ...
      + (39/2)/(1 + 17/(z2 ...
      )))))))))))))))))))))))))))))))))));
  res *= sf_exp(-z^2)/qpi;
endfunction
