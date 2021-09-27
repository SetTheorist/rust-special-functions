function b = subsref (a, s)
   if (isempty (s))
     error ("sf_qdouble: missing index");
   endif
   switch (s(1).type)
     case "()"
       error ("sf_qdouble: unsupported indexing");
     case "{}"
       error ("sf_qdouble: unsupported indexing");
     case "."
       fld = s.subs;
       if (strcmp (fld, "hi"))
         b = a.hi_;
       elseif (strcmp (fld, "lo"))
         b = a.lo_;
       else
         error ("@sf_qdouble/subsref: invalid property \"%s\"", fld);
       endif
     otherwise
       error ("@sf_qdouble/subsref: invalid subscript type");
   endswitch
   #if (numel (s) > 1) b = subsref (b, s(2:end)); endif
 endfunction

