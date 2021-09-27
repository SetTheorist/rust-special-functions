function [res,drs] = air(z0, h)
  wz = airy(0, z0);
  dwz = airy(1, z0);
  [res,drs] = step_fore(z0, h, wz, dwz);
  [reb,drb] = step_back(z0, h, wz, dwz);
  fzh = airy(0, z0+h)
  res
  reb
  dzh = airy(1, z0+h)
  drs
  drb
endfunction

function [f,df] = step_fore(z, h, wz, dwz)
  NN = 100;
  coeffs = zeros(1,NN);
  #coeffs(1) = wz;
  #coeffs(2) = dwz;
  #coeffs(3) = z * wz / 2;
  coeffs(1) = wz;
  coeffs(2) = dwz*h;
  coeffs(3) = z * wz *h^2/ 2;
  #coeffs(3) = z * wz;
  for k = 4:NN
    n = k-1;
    #coeffs(k) = z*coeffs(k-2)/(n*(n-1)) + (n-3)*coeffs(k-3)/(n*(n-1)*(n-2));
    coeffs(k) = h^2*z*coeffs(k-2)/(n*(n-1)) + h^3*(n-3)*coeffs(k-3)/(n*(n-1)*(n-2));
    #coeffs(k) = z*coeffs(k-2) + (n-3)*coeffs(k-3);
  endfor
  #coeffs
  #f = sum(h.^(0:(NN-1)) .* coeffs, 'extra');
  #df = sum(h.^(0:(NN-2)) .* (coeffs(2:end) .* (1:(NN-1))), 'extra');
  f = sum(coeffs, 'extra');
  df = sum((coeffs(2:end) .* (1:(NN-1)))/h, 'extra');
  #f = sum(h.^(0:(NN-1)) .* coeffs ./ factorial(0:(NN-1)), 'extra');
  #df = sum(h.^(0:(NN-2)) .* (coeffs(2:end) .* (1:(NN-1))) ./ factorial(1:(NN-1)), 'extra');
endfunction

function [f,df] = step_back(z, h, wz, dwz)
  NN = 100;
  coeffs = zeros(1,NN);
  coeffs(NN) = 1;
  coeffs(NN-1) = 0;
  coeffs(NN-2) = 0;
  for k = NN-3:(-1):2
    n = k-1;
    coeffs(k) = -(n+1)/n * z * coeffs(k+1) + (n+1)*(n+2)*(n+3)*coeffs(k+3);
  endfor
  coeffs *= dwz / coeffs(2);
  coeffs(1) = wz;
  #coeffs
  f = sum(h.^(0:(NN-1)) .* coeffs, 'extra');
  df = sum(h.^(0:(NN-2)) .* (coeffs(2:end) .* (1:(NN-1))), 'extra');
endfunction
