## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_exp_m1vx (@var{z})
## Compute $(e^z - 1) / z$
## @end deftypefn

function res = sf_exp_m1vx(z)
  res = ones(size(z));
  for n = 1:prod(size(z));
    if (isinf(z(n)))
      if (z(n)>0)
        res(n) = +Inf;
      else
        res(n) = 0;
      endif
    elseif (isnan(z(n)))
      res(n) = z(n);
    else
      res(n) = sf_exp_m1vx_1(z(n));
    endif
  endfor
endfunction

function res = sf_exp_m1vx_1(z)
  if (abs(z)>1/2)
    # TODO: this loses accuracy for some complex cases...
    res = (sf_exp(z) - 1) / z;
  else
    z2 = z^2;
    res = 2/(2 - z + z2/6/(1
          + z2/(4*(2*3-3)*(2*3-1))/(1
          + z2/(4*(2*4-3)*(2*4-1))/(1
          + z2/(4*(2*5-3)*(2*5-1))/(1
          + z2/(4*(2*6-3)*(2*6-1))/(1
          + z2/(4*(2*7-3)*(2*7-1))/(1
          + z2/(4*(2*8-3)*(2*8-1))/(1
          ))))))));
  endif
endfunction
