## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gamma_star (@var{z})
## Compute Gamma^* function -- x^(-x+1/2)*e^(x)*Gamma(x)/sqrt(2 pi)
## @end deftypefn

function res = sf_gamma_star(z)
  if (nargin<1)
    print_usage;
  endif
  if (abs(z)<130)
    # use this unless it blows up
    res = sf_gamma(z) * z^(1/2-z) * exp(z) / sqrt(2*pi);
  else
    # Spouge's approximation (modified for scaling)
    a = 13;
    z -= 1;
    res = ((z+a)/(z+1))^(z+1/2) * sf_exp(1-a);
    sm = sqrt(2*pi);
    for k = 1 : (a-1)
      sm += spouge_c(k,a) / (z+k);
    endfor
    res *= sm/sqrt(2*pi);
  endif
endfunction

function res = spouge_c(k,a)
  res = ((-1)^(k-1) / factorial(k-1)) * (-k+a)^(k-1/2) * sf_exp(-k + a);
endfunction

# this looks simply broken - values returned are wrong
function res = lanczos_from_wikipedia(z)
  persistent g = 7;
  persistent p = [
      0.99999999999980993, 676.5203681218851, -1259.1392167224028,
      771.32342877765313, -176.61502916214059, 12.507343278686905,
      -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7];
  if (real(z) < 0.5)
    res = pi / (sin(pi*z) * lanczos_from_wikipedia(1-z));
    return;
  endif
  z -= 1;
  a_g = p(1);
  for k = 1:(g+1)
    a_g += p(1+k) / (z+k);
  endfor
  t = z + g + 0.5;
  res = sqrt(2*pi) * t^(z+0.5) * exp(-t) * a_g;
endfunction
