## -*- texinfo -*-
## @deftypefn  {Function File} {} sf_qdouble ()
## @deftypefnx {Function File} {} sf_qdouble (@var{a})
## @deftypefnx {Function File} {} sf_qdouble (@var{a}, @var{b})
## @end deftypefn
function p = sf_qdouble(a, b)
  %fprintf(stderr, "sf_qdouble(%i)\n", nargin);
  if (nargin < 2) b = 0.0; endif
  if (nargin < 1) a = 0.0; endif

  if (ischar(a))
    %fprintf(stderr, "<1c>[a=%s]", a);
    p = qdouble_atodd(a);
  elseif (strcmp(class(a), "sf_qdouble"))
    %fprintf(stderr, "<1q>");
    p = a;
  else
    %fprintf(stderr, "<2>[a=%g|b=%g]", a, b);
    p.hi_ = a + b;
    p.lo_ = b + (a - p.hi_);
    p = class(p, "sf_qdouble");
  endif
endfunction

function res = qdouble_atodd(a)
  %fprintf(stderr, "qdouble_atodd(a=%s)", a);
  res = sf_qdouble(0.0);
  ex = 0;
  i = 1;
  l = length(a);
  # eat whitespace
  while (a(i)==' ' || a(i)=='\t' || a(i)=='\n' || a(i)=='\r')
    ++i;
  endwhile
  # sign of mantissa
  switch (a(i))
  case '-'
    sign = -1;
    ++i;
  case '+'
    sign = +1;
    ++i;
  otherwise
    sign = 1;
  endswitch
  # digits before decimal point
  %fprintf(stderr, "**digits before decimal point\n");
  while (i<=l)
    n = a(i) - '0';
    ++i;
    %fprintf(stderr, "n=%g i=%g\n", n, i);
    if ((n<0) || (n>=10)) break; endif
    %fprintf(stderr, "...");
    res = 10*res + n;
    %fprintf(stderr, "...");
  endwhile
  --i;
  # digits after decimal point
  %fprintf(stderr, "**digits after decimal point\n");
  if (a(i)=='.')
    ++i;
    while (i<=l)
      n = a(i) - '0';
      i = i + 1;
      if ((n<0) || (n>=10)) break; endif
      res = 10*res + n;
      --ex;
    endwhile
    --i;
  endif
  %fprintf("**get exponent\n");
  # get exponent
  if ((i<=l) && (a(i)=='e' || a(i)=='E'))
    ex += str2num(a(i+1:end));
  endif
  while (ex-- > 0)
    res *= 10;
  endwhile
  while (++ex < 0)
    res /= 10;
  endwhile
  if (sign < 0)
    res = -res;
  endif
endfunction
