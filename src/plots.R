library(ggplot2)
setwd('~/Dropbox/work/evol/MA/coalescence/ma_brothers')
d <- read.table('20gens_1000times_three_d07_bis.tsv', sep="\t", header=FALSE)

alive <- d[d$V1=="alive",'V2']
alivedf=data.frame(alive=alive)
ggplot(data=alivedf, aes(x=alive))+geom_histogram()+theme_bw() 

mrca_3 <- d[d$V1!="alive",]
colnames(mrca_3) <- c('index', 'gen')
mrca_3 <- mrca_3[mrca_3$index != -1,]
bi <- length(unique(mrca_3$gen))
ggplot(data=mrca_3, aes(x=gen))+geom_bar()+theme_bw()+scale_x_continuous(labels=seq(0, bi), breaks=seq(0, bi))



nrow(d[d$V1==-1,])
