## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_excsc (@var{z})
## Compute the excosecant function
## @end deftypefn

function res = sf_excsc(z)
  persistent pi_2 = sf_qdouble("1.570796326794896619231321691639751442099");
  res = zeros(size(z));
  for n = 1:prod(size(z))
    zx = (pi_2 - z(n)).hi;
    res(n) = sf_exsec(zx);
  endfor
endfunction
