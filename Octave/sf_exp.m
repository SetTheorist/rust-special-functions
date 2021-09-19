## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_exp (@var{z})
## Compute exponential 
## @end deftypefn

function res = sf_exp(z)
  res = zeros(size(z));
  for n = 1:prod(size(z))
    if (isinf(z(n)))
      if (z(n)>0)
        res(n) = +Inf;
      else
        res(n) = 0.0;
      endif
    elseif (isnan(z(n)))
      res(n) = z(n);
    else
      res(n) = sf_exp_1__ln2_cf(z(n));
    endif
  endfor
endfunction

function res = sf_exp_1__ln2_cf(z)
  x = real(z); y = imag(z);

  if (x == 0.0)
    res = 1.0;
  else
    # make x positive
    if (x<0) x=-x; recip = true; else recip = false; endif

    # range-reduction:
    # write x as (ln(2)*kk + f)
    # with 0<=f<ln(2)
    persistent qln2 = sf_qdouble("0.69314718055994530941723212145817656807550013436026");
    if (x>0.69)
      qx = sf_qdouble(x);
      kk = floor((x/qln2).hi);
      x = (qx - kk*qln2).hi;
      e_ = (qx - kk*qln2).lo;
    else
      kk = 0;
      e_ = 0;
    endif

    # fixed-length continued-fraction
    # good, but sum is about the same speed and
    # sometimes a little more accurate
    #x2 = x^2;
    #res = 1 + 2*x/(2 - x ...
    #    +x2/  6/(1 ...
    #    +x2/ 60/(1 ...
    #    +x2/140/(1 ...
    #    +x2/252/(1 ...
    #    +x2/396/(1 ...
    #    +x2/572/(1 ...
    #    +x2/780)))))));

    NN = 20; # empirically determined
    res =  sum([1, cumprod(x*ones(1,NN) ./ (1:NN)), e_], 'extra');

    # undo reductions
    res = pow2(res, kk);
    if (recip) res = 1/res; endif
  endif

  if (y != 0.0)
    res *= (sf_cos(y) + I*sf_sin(y));
  endif
endfunction


%!test load "exp_1.dat";
%!     assert(sf_exp(exp_1(:,1)), exp_1(:,2), -1e-15);
%!test load "exp_2.dat";
%!     assert(sf_exp(exp_2(:,1)+I*exp_2(:,2)), exp_2(:,3)+I*exp_2(:,4), -1e-15);
