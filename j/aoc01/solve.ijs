require 'format/printf'
raw =: freads 'input'

NB. Cut at newlines and convert to numbers
data =: __".><;._2 raw

fuel =: ([:-&2[:<.%&3)
part1 =: +/fuel data
'Part 1 solution is: %d' printf <part1

total_fuel =: fuel@]F:([(_2 Z:<&0))
part2 =: +/^:_ total_fuel"0 data
'Part 2 solution is: %d' printf <part2