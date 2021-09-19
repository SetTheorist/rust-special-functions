# -*- texinfo -*-
# @deftypefn {Function File} {@var{res} =} hyperg_1f1 (@var{a}, @var{b}, @var{z})
# Compute the confluent hypergeometric series
# @end deftypefn

function res = carrphi(a, b, c, x, y)
  res = quad(@(u)(u^(a-1) * (1-u)^(c-a-1) * (1-u*x)^(-b) * exp(u*y)), 0, 1);
endfunction
