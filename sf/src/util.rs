use crate::traits::*;
use crate::log::{Log,sf_log};

pub fn relerr<V:Value+Log>(exact:V, approx:V) -> V {
  let ε : V = ι(V::epsilon);
  if exact == approx {
    sf_log(ε.sqr()) / sf_log(ι(10):V)
  } else {
    sf_log(vabs(approx - exact) / (ε.sqr() + vabs(exact))) / sf_log(ι(10):V)
  }
}


////////////////////////////////////////////////////////////////////////////////

/*
\section{Continued fraction evaluation}

\subsection{Backwards recurrence algorithm}

We can compute the $n$'th convergent $C_n$ for a predetermined $n$ by evaluating
\[ u_k = b_k + \frac{a_{k+1}}{u_{k+1}} \]
for $k=n-1, n-2, \dots, 0$, with $u_n = b_n$.  Then $u_0 = C_n$.

\begin{titled-frame}{\color{blue}\tt sf\_cf\_back}
\begin{code}
sf_cf_back :: forall v.(Value v) => Int -> [v] -> [v] -> v
sf_cf_back !n !as !bs =
  let !an = reverse $ take n as
      !(un:bn) = reverse $ take (n+1) bs
  in go un an bn
  where
    go :: v -> [v] -> [v] -> v
    go !ukp1 ![] ![] = ukp1
    go !ukp1 !(a:an) !(b:bn) =
        let uk = b + a/ukp1
        in go uk an bn
\end{code}
\end{titled-frame}

\subsection{Steed's algorithm}
This is Steed's algorithm for evaluation of a continued fraction
It evaluates the partial convergents $C_n$ in a forward direction.
This implementation will evaluate until $C_n=C_{n+1}$.
TODO: describe algorithm.
\begin{titled-frame}{\color{blue}\tt sf\_cf\_steeds}
\begin{code}
sf_cf_steeds :: (Value v) => [v] -> [v] -> v
sf_cf_steeds (a1:as) (b0:b1:bs) =
    let !c0 = b0
        !d1 = 1/b1
        !delc1 = a1*d1
        !c1 = c0 + delc1
    in recur c1 delc1 d1 as bs
    where
      !eps = 5e-16
      recur !cn' !delcn' !dn' !(an:as) !(bn:bs) =
        let !dn = 1/(dn'*an+bn)
            !delcn = (bn*dn - 1)*delcn'
            !cn = cn' + delcn
        in if cn == cn' || (rabs delcn)<eps || is_nan cn
           then cn
           else (recur cn delcn dn as bs)
\end{code}
\end{titled-frame}

\subsection{Runge-Kutta IV}
*/
