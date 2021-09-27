## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_weber_e (@var{nu}, @var{z})
## Compute Weber's function $E_\nu(z)$
## @end deftypefn

function res = sf_weber_e(nu, z)
  if (nargin<2)
    print_usage;
  endif
  if (any(size(nu)!=size(z)))
    if (isscalar(z))
      z *= ones(size(nu));
    elseif (isscalar(nu))
      nu *= ones(size(z));
    else
      error("sf_weber_e: parameter sizes mismatch");
    endif
  endif
  res = zeros(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_weber_e_1(nu(n), z(n));
  endfor
endfunction

function res = sf_weber_e_1(nu, z)
  if (abs(z)<10 || abs(z/nu)<1)
    res = weber_series(nu, z);
  else 
    # direct asymptotic
    res = weber_asympt(nu, z);
  endif
endfunction

function res = weber_series(nu, z)
  sinnupi = sf_sin(nu*pi) / pi;
  # series expansions
  lom_0 = sf_lommel_s( 0, nu, z);
  lom_1 = sf_lommel_s(-1, nu, z);
  res = -(1+sf_cos(nu*pi)) * lom_0 / pi - nu * (1 - sf_cos(nu*pi)) * lom_1 / pi;
endfunction

function res = weber_asympt(nu, z)
  # asymptotic expansions
  lom2_0 = sf_lommel_s2( 0, nu, z);
  lom2_1 = sf_lommel_s2(-1, nu, z);
  res = -sf_bessel_y(z, nu) - (1 + sf_cos(nu*pi)) * lom2_0 / (pi*z) - nu * (1 - sf_cos(nu*pi)) * lom2_1 / (pi*z^2);
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
