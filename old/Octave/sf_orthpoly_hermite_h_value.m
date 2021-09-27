## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_h_value (@var{n}, @var{z})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_hermite_h_value (@var{n}, @var{z}, [], @var{k})
## Compute the value of the $n$'th Hermite polynomial: $H_n(z)$,
## (or its $k$'th derivative)
## $n=0, 1, 2, ...$, typically $z\in(-\infty,\infty)$
## @end deftypefn

function res = sf_orthpoly_hermite_h_value(n, z, dum, k)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin>2)
    if (!isempty(dum) || !sf_is_nonnegint(k)) print_usage; endif
    if (k==0)
      res = sf_orthpoly_hermite_h_value(n, z);
    elseif (k>n)
      res = zeros(size(z));
    else
      res = 2*n*sf_orthpoly_hermite_h_value(n-1, z, [], k-1);
    endif
    return;
  endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = 2*z;
  otherwise
    rm1 = ones(size(z)); rm1_e_ = 0;
    rm0 = 2*z; rm0_e_ = 0;
    for k=2:n
      rm2 = rm1; rm2_e_ = rm1_e_;
      rm1 = rm0; rm1_e_ = rm0_e_;
      #rm0 = 2*z.*rm1 - 2*(k-1)*rm2;
        s_ = -2*(k-1)*rm2; e_ = -2*(k-1)*rm2_e_;
        t_ = s_;
          y_ = 2*z.*rm1 + e_;
          s_ = t_ + y_;
          e_ = (t_ - s_) + y_;
        t_ = s_;
          y_ = 2*z.*rm1_e_ + e_;
          s_ = t_ + y_;
          e_ = (t_ - s_) + y_;
        rm0 = s_; rm0_e_ = e_;
    endfor
    res = rm0 + rm0_e_;
  endswitch
endfunction
