## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_theta (@var{z}, @var{q}, @var{j})
## Compute Jacobi theta function $\theta_j(z,q)$ for $j=1,2,3,4$
## @end deftypefn
function res = sf_theta(z,q,j)
  if (nargin != 3)
    print_usage;
  endif
  sz = max([size(z);size(q);size(j)]);
  if (any(size(z)!=sz))
    if (isscalar(z)) z *= ones(sz);
    else error("sf_theta: mismatched parameter sizes");
    endif
  endif
  if (any(size(q)!=sz))
    if (isscalar(q)) q *= ones(sz);
    else error("sf_theta: mismatched parameter sizes");
    endif
  endif
  if (any(size(j)!=sz))
    if (isscalar(j)) j *= ones(sz);
    else error("sf_theta: mismatched parameter sizes");
    endif
  endif
  res = zeros(sz);
  for kk = 1:prod(sz)
    if (j(kk)==1)
      res(kk) = sf_theta_1(z(kk),q(kk));
    elseif (j(kk)==2)
      res(kk) = sf_theta_2(z(kk),q(kk));
    elseif (j(kk)==3)
      res(kk) = sf_theta_3(z(kk),q(kk));
    elseif (j(kk)==4)
      res(kk) = sf_theta_4(z(kk),q(kk));
    else
      print_usage;
    endif
  endfor
endfunction

function res = sf_theta_1(z,q)
  res = 0.0;
  n = 0;
  do
    qpow = q^((n+0.5)^2) * (-1)^n;
    res += qpow * sf_sin((2*n+1)*z);
    ++n;
    if (n>999) break; endif
  until (abs(res) + abs(qpow) == abs(res))
  res *= 2;
endfunction

function res = sf_theta_2(z,q)
  res = 0.0;
  n = 0;
  do
    qpow = q^((n+0.5)^2);
    res += qpow * sf_cos((2*n+1)*z);
    ++n;
    if (n>999) break; endif
  until (abs(res) + abs(qpow) == abs(res))
  res *= 2;
endfunction

function res = sf_theta_3(z,q)
  # use transform ...
  phi = -sf_log(q)/pi;
  q_prime = sf_exp(-pi/phi);
  res = 0.0;
  n = 1;
  do
    qpow = q_prime^(n^2);
    old_res = res;
    res += sf_exp(-n^2*pi/phi + 2*n*z/phi)*(1.0 + sf_exp(-4*n*z/phi))*0.5;
    ++n;
    if (n>999) break; endif
  until (old_res == res)
  res = sf_exp(-z^2/(pi*phi) + sf_log_p1(2*res))/sqrt(phi);
endfunction
function res = sf_theta_3_v1(z,q)
  res = 0.0;
  n = 1;
  do
    qpow = q^(n^2);
    res += qpow * sf_cos((2*n)*z);
    ++n;
    if (n>999) break; endif
  until (abs(res) + abs(qpow) == abs(res))
  res = 1 + 2*res;
endfunction

function res = sf_theta_4(z,q)
  res = 0.0;
  n = 1;
  do
    qpow = q^(n^2) * (-1)^n;
    res += qpow * sf_cos((2*n)*z);
    ++n;
    if (n>999) break; endif
  until (abs(res) + abs(qpow) == abs(res))
  res = 1 + 2*res;
endfunction
