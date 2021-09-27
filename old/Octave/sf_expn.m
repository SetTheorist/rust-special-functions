## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_expn (@var{n}, @var{z})
## Compute initial part of series for exponential, $\sum_(k=0)^n z^k/k!$ 
## ($n=0,1,2,...$)
## @end deftypefn

function res = sf_expn(n, z)
  if (nargin<2 || !sf_is_nonnegint(n)) print_usage; endif
  if (any(size(n)!=size(z)))
    if (isscalar(n))
      n *= ones(size(z));
    elseif (isscalar(z))
      z *= ones(size(n));
    else
      error("sf_expn: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(z));
  for k = 1:prod(size(z))
    if (isinf(z(k)))
      if (z(k)>0)
        res(k) = Inf;
      else
        res(k) = Inf * (-1)^(rem(n(k),2));
      endif
    elseif (isnan(z(k)))
      res(k) = z(k);
    else
      #sf_exp(z(k)) - sf_exp_men(n(k)+1, z(k))
      #direct summation generally works fine...
      res(k) = series_k(n(k), z(k));
    endif
  endfor
endfunction

# Kahan summation
# for a little more accuracy in a few cases
function res = series_k(n, z)
  res = 1.0;
  e_ = 0.0;
  term = 1.0;
  for k=1:n
    term *= z/k;
    old_res = res;
    t_ = res;
    y_ = term + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    if (res == old_res) break; endif
  endfor
  res += e_;
endfunction
