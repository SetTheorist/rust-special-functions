## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_is_int (@var{x})
## True if $x$ represents an integer
## @end deftypefn

function res = sf_is_int(x)
  if (nargin < 1) print_usage; endif
  res = isreal(x) & (x==fix(x)) & !isinf(x) & !isnan(x);
endfunction

%!test assert( sf_is_int(-1e100))
%!test assert( sf_is_int(-3))
%!test assert( sf_is_int(-2))
%!test assert( sf_is_int(-1))
%!test assert( sf_is_int(0))
%!test assert( sf_is_int(1))
%!test assert( sf_is_int(2))
%!test assert( sf_is_int(3))
%!test assert( sf_is_int(1e100))
%!test assert(!sf_is_int(3.5))
%!test assert(!sf_is_int(-0.2))
%!test assert(!sf_is_int(-Inf))
%!test assert(!sf_is_int(Inf))
%!test assert(!sf_is_int(NaN))

%!demo
%!  sf_is_int(3)
%!  sf_is_int([(1:10)/2, Inf, nan])
