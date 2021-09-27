## -*- texinfo -*-
## @deftypefn {Function File} {[@var{U},@var{V}] =} sf_pcf_uv (@var{z}, @var{a})
## Compute parabolic cylinder functions $U(a,z)$, $V(a,z)$
## @end deftypefn

function [U,V] = sf_pcf_uv(z,a)
  if (nargin < 2)
    print_usage;
  endif
  uu1 = uu_1(z,a);
  uu2 = uu_2(z,a);
  U = u_a_0(a)*uu1 + du_a_0(a)*uu2;
  if (nargout >1)
    V = v_a_0(a)*uu1 + dv_a_0(a)*uu2;
  endif
endfunction

function res = u_a_0(a)
  res = sqrt(pi) * 2^(-a/2-1/4) / gamma(a/2 + 3/4);
endfunction
function res = du_a_0(a)
  res = -sqrt(pi) * 2^(-a/2+1/4) / gamma(a/2 + 1/4);
endfunction

function res = v_a_0(a)
  res = 2^(a/2+1/4) * sin(pi*(3/4-a/2)) / gamma(3/4 - a/2);
endfunction
function res = dv_a_0(a)
  res = 2^(a/2+3/4) * sin(pi*(1/4-a/2)) / gamma(1/4 - a/2);
endfunction

function res = uu_1(z,a)
  res = 1.0;
  term = 1.0;
  n = 1;
  z2 = z^2;
  do
    term *= z2 * (a + (2*n-1)/2) / (n*(n+1));
    old_res = res;
    res += term;
    n += 2;
    if (n>999) break; endif
  until (res == old_res)
  res *= exp(-z2/4);
endfunction

function res = uu_2(z,a)
  res = z;
  term = z;
  n = 2;
  z2 = z^2;
  do
    term *= z2 * (a + (2*n-1)/2) / (n*(n+1));
    old_res = res;
    res += term;
    n += 2;
    if (n>999) break; endif
  until (res == old_res)
  res *= exp(-z2/4);
endfunction
