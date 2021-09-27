## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_h_zeros (@var{n})
## Compute the zeros of the $n$'th Hermite polynomial: $H_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_hermite_h_zeros(n)
  persistent cache = {}
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [0];
  else
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return
    endif
    if (n<500)
      # eigenvalue approach for small n
      m = zeros(n);
      for k=1:n-1
        m(k,k+1) = m(k+1,k) = sf_sqrt(k/2);
      endfor
      res = sort(eig(m));

      if (n<=205)
        # "polish" the results with Newton
        fx = sf_orthpoly_hermite_h_value(n,res);
        dfx = 2*n*sf_orthpoly_hermite_h_value(n-1,res);
        res -= fx./dfx;
        fx = sf_orthpoly_hermite_h_value(n,res);
        dfx = 2*n*sf_orthpoly_hermite_h_value(n-1,res);
        res -= fx./dfx;
      endif
    else
      # use Dominici approach to approximate zeros
      # (eigenvalue breaks down numerically for large n)
      # this seems stable (enough)
      # we save a little work by just computing the positive roots and using symmetry
      if (true)
        # Kapteyn series for approximation to roots
        N = 2*n+1;
        K = (floor(n/2):(-1):1).';
        res = pi/2 - pi/2 * (4*K-1) / N;
        J = (10 + sf_log(1+n)*10 + n);
        JJ = 1:J;
        besj = sf_bessel_j(JJ, JJ*(1-1/N));
        for jj=1:J
          res -= besj(jj)*sin(jj*pi*(4*K-1)/N)/jj; #TODO: replace with sf_sin() --- performance is the issue here
        endfor
        res = sf_sqrt(2*n) * sf_sin(res);
      else
        # asymptotic approximation for roots
        K = (floor(n/2):(-1):1).';
        kap = 3*pi*(4*K - 1);
        res = sqrt(2) * (
            n^(1/2)
            - kap.^(2/3)/8*n^(-1/6)
            + 1/3*n^(-1/2)
            - (kap.^2+80)./(640*kap.^(2/3))*n^(-5/6)
            - (11*kap.^2 + 3920)/179200*n^(-3/2)
            + (5*kap.^4 + 96*kap.^2 + 640)./(7680*kap.^(8/3))*n^(-11/6)
            - (823*kap.^6 + 647200*kap.^4 - 2464000*kap.^2 - 25088000)./(258048000*kap.^(10/3))*n^(-13/6)
            + (3064 + 33*kap.^2)/(716800)*n^(-5/2)
            );
      endif

      # and then do a few steps of Newton to narrow in
      old_old_old_res = old_old_res = old_res = res;
      its = 0;
      do
        old_old_old_old_res = old_old_old_res;
        old_old_old_res = old_old_res;
        old_old_res = old_res;
        old_res = res;
        fx = sf_orthpoly_hermite_h_value(n,res);
        dfx = 2*n*sf_orthpoly_hermite_h_value(n-1,res);
        res -= fx./dfx;
        ++its;
        if (its>19) break; endif # small number here - it shouldn't take many steps...
      until all((res == old_res) | (res == old_old_res) | (res == old_old_old_res) | (res == old_old_old_old_res))
      # (yes, we can actually get a 4-cycle)
      if (sf_is_oddint(n))
        res = [-flipud(res); 0; res];
      else
        res = [-flipud(res); res];
      endif
    endif
    cache{n} = res;
  endif
endfunction
