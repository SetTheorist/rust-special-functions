## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_is_negint (@var{x})
## True if $x$ represents an integer < 0
## @end deftypefn
function res = sf_is_negint(x)
  if (nargin < 1) print_usage; endif
  res = isreal(x) & (x<0) & (x==fix(x));
endfunction
