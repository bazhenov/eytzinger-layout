set term svg enhanced size 800,400
set output 'plot.svg'

set grid

set datafile separator ','
set log x 2
#set log y
set ylabel "time (ns.)"
set xlabel "data size (Kb)"

set arrow from 49, graph 0 to 49, graph 1 nohead lc "red" dt 4
set label 1 at 49, graph 0.7 "L1" offset -3,0
set arrow from 512, graph 0 to 512, graph 1 nohead lc "red" dt 4
set label 2 at 512, graph 0.7 "L2" offset -3,0
set arrow from 8192, graph 0 to 8192, graph 1 nohead lc "red" dt 4
set label 3 at 8192, graph 0.7 "L3" offset -3,0

plot "data.csv" using 1:2 title "std. binary search" with linespoints lc rgbcolor "#ff9900", \
     "" using 1:3 title "eytzinger layout" with linespoints, \
     "" using 1:4 title "eytzinger layout w. prefetch" with linespoints
