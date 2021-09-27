## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_pcf_u (@var{a}, @var{z})
## Compute parabolic cylinder function $U(a,z)$
## @end deftypefn

function res = sf_pcf_u(a, z)
  if (nargin < 2) print_usage; endif
  if (any(size(z)!=size(a)))
    if (isscalar(z)) z*=ones(size(a));
    elseif (isscalar(a)) a*=ones(size(z));
    else error("sf_pcf_u: mismatched parameter sizes");
    endif
  endif

  res = zeros(size(z));
  for kk = 1:prod(size(z))
    if (abs(z(kk))<0.5)
      if (a(kk)<5)
        # this is vector-able, but for now do the loop:
        res(kk) = sf_pcf_u_u0(a(kk)).*sf_pcf_uv_even(a(kk),z(kk)) + sf_pcf_u_du0(a(kk)).*sf_pcf_uv_odd(a(kk),z(kk));
      else
        res(kk) = recur_dn2(a(kk),z(kk));
      endif
    else
      #valid if real(a)>-1/2
      if (real(a(kk)) && real(z(kk)))
        res(kk) = sf_exp(-z(kk)^2/4)/sf_gamma(a(kk)+1/2) * quad(@(t)( t^(a(kk)-1/2) * exp(-t^2/2-z(kk)*t) ), 0, inf, 1e-14);
      else
        res(kk) = sf_exp(-z(kk)^2/4)/sf_gamma(a(kk)+1/2) * ( ...
             quad(@(t)(real( t^(a(kk)-1/2) * exp(-t^2/2-z(kk)*t) )), 0, inf, 1e-14) ...
          +I*quad(@(t)(imag( t^(a(kk)-1/2) * exp(-t^2/2-z(kk)*t) )), 0, inf, 1e-14) );
      endif
    endif
  endfor
  #rup = recur_up(a,z)
  #rdn = recur_dn(a,z)
endfunction

# better for z<1 ?
# gives good values for large a and small z
# (better than direct summation in those cases)
function res = recur_up(a,z)
  #assume a>0
  nn = fix(a);
  afrac = a - nn;
  m2 = sf_pcf_u_u0(  afrac).*sf_pcf_uv_even(  afrac,z) + sf_pcf_u_du0(  afrac).*sf_pcf_uv_odd(  afrac,z);
  m1 = sf_pcf_u_u0(1+afrac).*sf_pcf_uv_even(1+afrac,z) + sf_pcf_u_du0(1+afrac).*sf_pcf_uv_odd(1+afrac,z);
  for n = 2:nn
    mm = (m2 - z*m1) / (afrac + n-1 + 1/2);
    m2 = m1;
    m1 = mm;
  endfor
  res = m1;
endfunction

# better for z>1 ?
# but still need initial values...
function res = recur_dn(a,z)
  #assume a>0
  nn = fix(a);
  afrac = a - nn;

  v0 = sf_pcf_u_u0(  afrac).*sf_pcf_uv_even(  afrac,z) + sf_pcf_u_du0(  afrac).*sf_pcf_uv_odd(  afrac,z);
  m2 = 0;
  m1 = 1;
  res = [m1,m2];
  for n = (nn+100):-1:0
    mm = z*m1 + (afrac + n+1 + 1/2)*m2;
    m2 = m1;
    m1 = mm;
    res = [mm, res];
  endfor
  res *= v0/m1;
  res = res(nn+1);
endfunction

# better for z>1 ?
# works a bit better when previous downward fails (if z>>1)
function res = recur_dn2(a,z)
  #assume a>0
  nn = fix(a);
  afrac = a - nn;

  aa = afrac + nn + 100 + 2;
  p = sqrt(aa);
  v = -(2/3)*(z/2)^3/(2*p) - (z/2)^2/(2*p)^2 - (z/2 - (2/5)*(z/2)^5)/(2*p)^3 + 2*(z/2)^4/(2*p)^4;
  m2 = sqrt(pi)/(2^(aa/2+1/4) * gamma(aa/2+3/4)) * exp(-p*z + v);

  aa = afrac + nn + 100 + 1;
  p = sqrt(aa);
  v = -(2/3)*(z/2)^3/(2*p) - (z/2)^2/(2*p)^2 - (z/2 - (2/5)*(z/2)^5)/(2*p)^3 + 2*(z/2)^4/(2*p)^4;
  m1 = sqrt(pi)/(2^(aa/2+1/4) * gamma(aa/2+3/4)) * exp(-p*z + v);

  res = [m1,m2];
  for n = (nn+100):-1:0
    mm = z*m1 + (afrac + n+1 + 1/2)*m2;
    m2 = m1;
    m1 = mm;
    res = [mm, res];
  endfor
  res = res(nn+1);
endfunction
