reset session
set terminal svg enhanced background rgb '#0D1117' size 780,__INSERT_HEIGHT_HERE__ font "Arial,14"
#set output 'benchmark.svg'

set xlabel __INSERT_LABEL_HERE__  tc rgb "white"  offset 0,graph 0.05
#set ylabel "Libraries" tc rgb "white"
set grid xtics lt 1 lc rgb '#3f3f4f' lw 0.5
unset ytics
set ytics scale 0 out nomirror  textcolor "white"
set xtics scale 0.75 out nomirror offset 0,graph 0.04 textcolor "white"

set border  lw 1 lc "#555555"
set style fill solid 1.0
set lmargin 12

# Define colors
set linetype 1 lc rgb '#92B2CA'  # Blue
set linetype 2 lc rgb '#C2C77B'  # Green
set linetype 3 lc rgb '#F4CF86'  # Yellow
set linetype 4 lc rgb '#E6A472'  # Orange
set linetype 5 lc rgb '#D77C79'  # Red
set linetype 6 lc rgb '#d5a8b7'  # Red
set linetype 7 lc rgb '#c0a7cc'  # Purple

$Data << EOD
__INSERT_DATA_HERE__
EOD

set xrange [0:__INSERT_XMAX_HERE__]
set yrange [0:*] reverse     # start at zero, find max from the data
set style fill solid  # solid color boxes
unset key             # turn off all titles

myBoxWidth = 0.8
set offsets 0,0,1.0-myBoxWidth/2.,1.0

plot $Data using (0.5*$2):0:(0.5*$2):(myBoxWidth/2.):($0+1):ytic(1) with boxxy lc var, \
     $Data using ($2):0:(sprintf(__INSERT_VALUE_FMT_HERE__, $2)) with labels left offset char 0.5,0 tc rgb '#a0a0b0' font ",11"
