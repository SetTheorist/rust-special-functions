function p = sf_qdouble_gamma(x)
  % don't really need so many...
  persistent n = 43;
  persistent c = [ % taylor coefficients for 1/gamma(1+x)-x...
    sf_qdouble("+0.5772156649015328606065120900824024310421593359"),
    sf_qdouble("-0.6558780715202538810770195151453904812797663805"),
    sf_qdouble("-0.0420026350340952355290039348754298187113945004"),
    sf_qdouble("+0.1665386113822914895017007951021052357177815022"),
    sf_qdouble("-0.0421977345555443367482083012891873913016526841"),
    sf_qdouble("-0.0096219715278769735621149216723481989753629422"),
    sf_qdouble("+0.0072189432466630995423950103404465727099048009"),
    sf_qdouble("-0.0011651675918590651121139710840183886668093337"),
    sf_qdouble("-0.0002152416741149509728157299630536478064782419"),
    sf_qdouble("+0.0001280502823881161861531986263281643233948920"),
    sf_qdouble("-0.0000201348547807882386556893914210218183822948"),
    sf_qdouble("-0.0000012504934821426706573453594738330922423226"),
    sf_qdouble("+0.0000011330272319816958823741296203307449433240"),
    sf_qdouble("-0.0000002056338416977607103450154130020572836512"),
    sf_qdouble("+0.0000000061160951044814158178624986828553428672"),
    sf_qdouble("+0.0000000050020076444692229300556650480599913030"),
    sf_qdouble("-0.0000000011812745704870201445881265654365055777"),
    sf_qdouble("+1.0434267116911005104915403323122501914007098231E-10"),
    sf_qdouble("+7.7822634399050712540499373113607772260680861813E-12"),
    sf_qdouble("-3.6968056186422057081878158780857662365709634513E-12"),
    sf_qdouble("+5.1003702874544759790154813228632318027268860697E-13"),
    sf_qdouble("-2.0583260535665067832224295448552374197460910808E-14"),
    sf_qdouble("-5.3481225394230179823700173187279399489897154781E-15"),
    sf_qdouble("+1.2267786282382607901588938466224224281654557504E-15"),
    sf_qdouble("-1.1812593016974587695137645868422978312115572918E-16"),
    sf_qdouble("+1.1866922547516003325797772429286740710884940796E-18"),
    sf_qdouble("+1.4123806553180317815558039475667090370863507503E-18"),
    sf_qdouble("-2.2987456844353702065924785806336992602845059314E-19"),
    sf_qdouble("+1.7144063219273374333839633702672570668126560625E-20"),
    sf_qdouble("+1.3373517304936931148647813951222680228750594717E-22"),
    sf_qdouble("-2.0542335517666727893250253513557337966820379352E-22"),
    sf_qdouble("+2.7360300486079998448315099043309820148653116958E-23"),
    sf_qdouble("-1.7323564459105166390574284515647797990697491087E-24"),
    sf_qdouble("-2.3606190244992872873434507354275310079264135521E-26"),
    sf_qdouble("+1.8649829417172944307184131618786668989458684290E-26"),
    sf_qdouble("+2.2180956242071972043997169136268603797317795006E-27"),
    sf_qdouble("+1.2977819749479936688244144863305941656194998646E-28"),
    sf_qdouble("+1.1806974749665284062227454155099715185596846378E-30"),
    sf_qdouble("-1.1245843492770880902936546742614395121194117955E-30"),
    sf_qdouble("+1.2770851751408662039902066777511246477487720656E-31"),
    sf_qdouble("-7.3914511696151408234612893301085528237105689924E-33"),
    sf_qdouble("+1.1347502575542157609541652594693063930086121959E-35"),
    sf_qdouble("+4.6391346410587220299448049079522284630579686797E-35")
  ];
  ss = sf_qdouble(x);
  f = sf_qdouble(1.0);
  sm = sf_qdouble(0.0);
  while (ss > 1.0)
    ss -= 1.0;
    f *= ss;
  endwhile
  while (ss < 1.0)
    f /= ss;
    ss += 1.0;
  endwhile
  if (ss == 1.0) p=f; return; endif
  ss -= 1.0;
  for i = n:(-1):1
    sm = c(i) + ss*sm;
  endfor
  p = f / (ss*sm + 1.0);
endfunction
