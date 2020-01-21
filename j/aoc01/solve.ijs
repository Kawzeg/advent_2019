require 'format/printf'
raw =: freads 'input'

NB. Cut at newlines and convert to numbers
data =: __".><;._2 raw

fuel =: ([:-&2[:<.%&3)
part1 =: +/fuel data
'Part 1 solution is: %d' printf <part1