function res = hermz(n)
  N = 2*n+1;
  for k = 1:n
    res(k) = pi/2 - pi/2 * (4*k-1) / N;
    for jj=1:(10 + sf_log(1+n)*15 + 2*n)
      res(k) -= besselj(jj, jj*(1-1/N))*sin(jj*pi*(4*k-1)/N)/jj ;
    endfor
    res(k) = sqrt(2*n) * sin(res(k));
  endfor
  res

  old_old_old_nwt = old_old_nwt = old_nwt = nwt = res;
  nwt_n = 0;
  do
    old_old_old_old_nwt = old_old_old_nwt;
    old_old_old_nwt = old_old_nwt;
    old_old_nwt = old_nwt;
    old_nwt = nwt;
    fx = sf_orthpoly_hermite_h_value(n,nwt);
    dfx = 2*n*sf_orthpoly_hermite_h_value(n-1,nwt);
    nwt = nwt - fx./dfx;
    #oo1 = nwt(40:50) - old_nwt(40:50)
    #oo2 = nwt(40:50) - old_old_nwt(40:50)
    #oo3 = nwt(40:50) - old_old_old_nwt(40:50)
    #oo4 = nwt(40:50) - old_old_old_old_nwt(40:50)
    ++nwt_n;
    if (nwt_n>10+n) break; endif
  until all((nwt == old_nwt) | (nwt == old_old_nwt) | (nwt == old_old_old_nwt) | (nwt == old_old_old_old_nwt))
  nwt_n

  old_hal = hal = res;
  hal_n = 0;
  do
    old_old_hal = old_hal;
    old_hal = hal;
    fx = sf_orthpoly_hermite_h_value(n,hal);
    dfx = 2*n*sf_orthpoly_hermite_h_value(n-1,hal);
    ddfx = 2*n*2*(n-1)*sf_orthpoly_hermite_h_value(n-2,hal);
    hal = hal - (2*fx.*dfx) ./ (2*dfx.^2 - fx.*ddfx);
    ++hal_n;
    if (hal_n>10+n) break; endif
  until all(hal == old_hal) || all(hal == old_old_hal)
  hal_n

  [hal.',sf_orthpoly_hermite_h_value(n,hal).',...
   nwt.',sf_orthpoly_hermite_h_value(n,nwt).',...
   res.',sf_orthpoly_hermite_h_value(n,res).']

endfunction
