option randseed '';

set assets;			# Set of assets
set realizations := 1..5;


param random1 {assets} := Normal(0, 10);

param copy{assets, realizations};
param toread {assets, realizations};


table assets_table_in IN "assets.tab":
assets <- [ASSETS];

table random_table_out{r in realizations} OUT ("random" & r & ".tab"): 
[assets], random1;

table random_table_in{r in realizations} IN ("random" & r & ".tab"): 
[a ~ assets], toread[a, r] ~ random1;



## to be read
#read table assets_table_in;


#for {i in realizations} 
#{
#write table random_table_out[i];
#let {a in assets} copy[a,i] := random1[a];
#reset data random1;
#}