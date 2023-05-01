set term svg enhanced size 800,400 lw 1.5
set output 'plot.svg'

set grid
set key left top

set datafile separator ','
set log x 2
set ylabel "time (ns.)"
set xlabel "data size (Kb)"

set arrow from 49, graph 0 to 49, graph 1 nohead lc "red" dt 4
set label 1 at 49, graph 0.6 "L1" offset -3,0
set arrow from 512, graph 0 to 512, graph 1 nohead lc "red" dt 4
set label 2 at 512, graph 0.6 "L2" offset -3,0
set arrow from 8192, graph 0 to 8192, graph 1 nohead lc "red" dt 4
set label 3 at 8192, graph 0.6 "L3" offset -3,0

plot "std.csv" using 1:2 title "std. binary search" with linespoints lc rgbcolor "#A60400", \
     "eytzinger.csv" using 1:2 title "eytzinger" with linespoints pt 4 lc rgbcolor "#052C6E", \
     "eytzinger-prefetch.csv" using 1:2 title "eytzinger w. prefetch" with linespoints pt 5 lc rgbcolor "#052C6E", \
     "eytzinger-branchless.csv" using 1:2 title "branchless eytzinger" with linespoints pt 4 lc rgbcolor "#97C300", \
     "eytzinger-branchless-prefetch.csv" using 1:2 title "branchless eytzinger w. prefetch" with linespoints pt 5 lc rgbcolor "#97C300"
