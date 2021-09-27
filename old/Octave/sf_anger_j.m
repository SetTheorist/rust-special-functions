## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_anger_j (@var{nu}, @var{z})
## Compute Anger's function $J_\nu(z)$
## @end deftypefn

function res = sf_anger_j(nu, z)
  if (nargin<2)
    print_usage;
  endif
  if (any(size(nu)!=size(z)))
    if (isscalar(z))
      z *= ones(size(nu));
    elseif (isscalar(nu))
      nu *= ones(size(z));
    else
      error("sf_anger_j: parameter sizes mismatch");
    endif
  endif
  res = zeros(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_anger_j_1(nu(n), z(n));
  endfor
endfunction

function res = sf_anger_j_1(nu, z)
  if (imag(nu)==0 && nu==fix(nu))
    # reduces to bessel function for integer nu
    res = sf_bessel_j(nu, z);
  endif
  if (abs(z)<10 || abs(z/nu)<1)
    res = anger_series(nu, z);
  else 
    # direct asymptotic
    res = anger_asympt(nu, z);
  endif

  if (false)
    # This approach doesn't work:
    # assume nu>0:
    # let nu = nup + N
    N = floor(nu);
    # use asymptotic for nup, nup+1
    # then recurse upward to nu
    nup = nu - floor(nu);
    asy_num1 = anger_asympt(nup  , z);
    asy_nu   = anger_asympt(nup+1, z);
    for n = 2:N
      nu0 = nup + n-1;
      asy = 2*nu0*asy_nu/z - asy_num1 - 2*nu0*sf_sin(pi*nu0) / (pi*z);
      asy_num1 = asy_nu;
      asy_nu = asy;
    endfor
    res = asy
  endif
endfunction

function res = anger_series(nu, z)
  sinnupi = sf_sin(nu*pi) / pi;
  # series expansions
  lom_0 = sf_lommel_s( 0, nu, z);
  lom_1 = sf_lommel_s(-1, nu, z);
  res = sinnupi * lom_0 - nu * sinnupi * lom_1;
endfunction

function res = anger_asympt(nu, z)
  sinnupi = sf_sin(nu*pi) / pi;
  # asymptotic expansions
  lom2_0 = sf_lommel_s2( 0, nu, z);
  lom2_1 = sf_lommel_s2(-1, nu, z);
  res = sf_bessel_j(nu, z) + sinnupi * lom2_0 / z - nu * sinnupi * lom2_1 / z^2;
endfunction


# quadrature --- very slow, not a good approach
function res = anger_quad(nu, z)
  if (imag(z)!=0 || imag(nu)!=0)
    qre = quad(@(th)(real( sf_cos(nu*th - z*sf_sin(th))/pi )), 0, pi, 1e-8);
    qim = quad(@(th)(imag( sf_cos(nu*th - z*sf_sin(th))/pi )), 0, pi, 1e-8);
    res = qre + I*qim
  else
    res = quad(@(th)( sf_cos(nu*th - z*sf_sin(th))/pi ), 0, pi, 1e-8)
  endif
endfunction
