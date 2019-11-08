set title 'Benchmarking the DLB'
set xlabel "Words"
set ylabel "time"
set datafile separator ","
plot "bench.dat" with line using 0:1
