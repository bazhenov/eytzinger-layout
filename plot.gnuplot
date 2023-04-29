set term png
set output 'plot.png'

set grid

set datafile separator ','
set log x 2
set log y
set ylabel "time (ns.)"
set xlabel "data size (Kb)"

set arrow from 49, graph 0 to 49, graph 1 nohead lc "red" dt 4
set arrow from 512, graph 0 to 512, graph 1 nohead lc "red" dt 4
set arrow from 8192, graph 0 to 8192, graph 1 nohead lc "red" dt 4

plot "data.csv" using 1:2 title "std. binary search" with linespoints lc rgbcolor "#ff9900", \
     "" using 1:3 title "eytzinger layout binary search" with linespoints
