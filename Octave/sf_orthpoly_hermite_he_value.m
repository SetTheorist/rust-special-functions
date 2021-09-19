## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_he_value (@var{n}, @var{z})
## Compute the value of the $n$'th Hermite polynomial: $He_n(z)$,
## $n=0, 1, 2, ...$, typically $z\in(-\infty,\infty)$
## @end deftypefn

function res = sf_orthpoly_hermite_he_value(n, z)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = z;
  otherwise
    rm1 = ones(size(z));
    rm1_e_ = zeros(size(z));
    rm0 = z;
    rm0_e_ = zeros(size(z));
    for k=2:n
      rm2 = rm1; rm2_e_ = rm1_e_;
      rm1 = rm0; rm1_e_ = rm0_e_;
      #rm0 = z.*rm1 - (k-1)*rm2;
        s_ = -(k-1)*rm2; e_ = -(k-1)*rm2_e_;
        t_ = s_;
          y_ = z.*rm1 + e_;
          s_ = t_ + y_;
          e_ = (t_ - s_) + y_;
        t_ = s_;
          y_ = z.*rm1_e_ + e_;
          s_ = t_ + y_;
          e_ = (t_ - s_) + y_;
        rm0 = s_;
        rm0_e_ = e_;
    endfor
    res = rm0;
  endswitch
endfunction
