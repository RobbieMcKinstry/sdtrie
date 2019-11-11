set title "Benchmarking the DLB"
set xlabel "Words"
set ylabel "time"
set key left top
set term png
set datafile separator ","
set output "words-vs-time.png"
plot "bench.dat" index 0:0 with line, "bench.dat" index 1:1 with line
set style line 1 linecolor rgb "violet"
set style line 2 linecolor rgb "chartreuse"
