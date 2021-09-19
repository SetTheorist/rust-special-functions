## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sievert (@var{theta}, @var{z})
## Compute Sievert's integral $S(\theta,z) = \int_0^\theta \exp(-z * \sec(t)) dt
## ($0\leq\theta\leq\pi/2$)
## ($z\in\RR$, $z\geq0$)
## @end deftypefn
function res = sf_sievert(theta, z)
  if (nargin < 2)
    print_usage;
  endif
  if (theta<0 || theta>pi/2 || imag(theta)!=0 || imag(z)!=0 || z<0) res = NaN; return; endif
  if (theta==0) res = 0.0; return; endif

  if (theta==pi/2)
    # special case with special integral
    res = quad(@(t)( sf_bessel_k(0, t) ), z, Inf);
    return;
  endif

  if (theta>pi/4 && z>1)
    # fancy series: faster convergence for larger z/cos(theta)
    costh = sf_cos(theta);
    res = sf_sievert(pi/2, z) - costh * sf_expint_en(z/costh, 2);
    ak = 1.0 * costh;
    k = 1;
    do
      ak *= costh^2 * (2*k-1) / (2*k);
      old_res = res;
      res -= ak * sf_expint_en(z/costh, 2*k+2);
      ++k;
      if (k>999) break; endif
    until (res == old_res)
  else
    # quadrature
    res = quad(@(t)( sf_exp(-z*sf_sec(t)) ), 0, theta, 1e-14);
  endif

  # asymptotics: generally bad accuracy
  #res = sqrt(pi/(2*z)) * sf_exp(-z) * sf_erf(theta * sqrt(z/2))
endfunction
