require 'format/printf'
raw =: freads 'input'

ints =: <;._1',',raw
ints =: 1,9,10,3,2,3,11,0,99,30,40,50
start_pos =: 0

add =: +
mult =: *
halt =: % NB. [(_2 Z: 1)

opcodes =: add`mult`halt
opselect =: (1 2 99) i. ]

get_op =: (add`mult`halt){~(1 2 99 i.])
NB. ints x arg y -> value of ints at index at index y+x
arg=: adverb : '({{])~m&+'

NB. x is the Intcode
NB. y is the instruction pointer
NB. ints run 0

run =: dyad : 0
op=.get_op y}x
a=.x 1 arg y
b=.x 2 arg y
at=.x 3 arg y
r =. a op`:0 b
r at} x
)

ints run 0

