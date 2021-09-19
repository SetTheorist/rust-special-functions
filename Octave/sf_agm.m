## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_agm (@var{a}, @var{b})
## @deftypefnx {Function File} {[@var{an},@var{bn},@var{cn}] =} agm (@var{a}, @var{b}, @var{c0})
## @deftypefnx {Function File} {[@var{an},@var{bn},@var{cn},@var{phin}] =} agm (@var{a}, @var{b}, @var{c0}, @var{phi0})
## Compute arithmetico-geometric mean of $a$, $b$, return either the scalar result or the arrays of intermediate values
## @end deftypefn

function [res1,res2,res3,res4] = sf_agm(a,b,c0,phi0)
  if (nargin < 2 || (nargout>=3 && nargin<3) || (nargout>=4 && nargin<4))
    print_usage;
  endif
  if (nargout >= 3)
    res1 = [];
    res2 = [];
    res3 = [];
    a1 = a;
    b1 = b;
    c1 = (a-b)/2;
    do
      res1(end+1) = a1;
      res2(end+1) = b1;
      res3(end+1) = c1;
      a = a1;
      b = b1;
      a1 = (a + b)/2;
      b1 = sqrt(a * b);
      c1 = (a - b)/2;
    until ((a==a1) && (b==b1))
    res1(end+1) = a1;
    res2(end+1) = b1;
    res3(end+1) = c1;
    res3(1) = c0;
    if (nargout >= 4)
      res4 = zeros(size(res1));
      res4(1) = phi0;
      for n = 2:length(res4)
        res4(n) = res4(n-1) + sf_atan(sf_tan(res4(n-1))*res2(n-1)/res1(n-1));
      endfor
    endif
  else
    a1 = a;
    b1 = b;
    do
      a = a1;
      b = b1;
      a1 = (a + b)/2;
      b1 = sqrt(a * b);
    until ((a==a1) && (b==b1))
    res1 = a1;
  endif
endfunction
