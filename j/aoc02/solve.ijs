require 'format/printf'
raw =: freads 'input'

ints =: <;._1',',raw
smoutput '$ints'; $ints

ints =: 1,9,10,3,2,3,11,0,99,30,40,50
smoutput '$ints'; $ints
start_pos =: 0

add =: +
mult =: *
halt =: dyad : '[(_2 Z: 1)'

opcodes =: add`mult`halt
opselect =: (1 2 99) i. ]

get_op =: (add`mult`halt){~(1 2 99 i.])
NB. ints x arg y -> value of ints at index at index y+x
arg=: adverb : '({{])~m&+'

NB. x is the Intcode
NB. y is the instruction pointer
NB. ints run 0
once =: dyad : 0
a=.(>y)}(>x)
smoutput 'a' ; a ; 'halt' ; a=99
op=.get_op a
smoutput 'op' ; op
_2 Z: (a=99)
NB. Op needs to determine arguments!
a=.x 1 arg y
b=.x 2 arg y
at=.((>y)+3)}(>x)
r =. a op`:0 b
smoutput 'r' ; r ; '@' ; at
(y+4);r at} x
)

u=: 3 : 0
smoutput 'u' ; y
<(>1}>y) once (>{.>y)
)

NB. x is Intcode
NB. y is instruction pointer (0)
start =: 4 : 0
init=.y;x
smoutput 'init';init
v=.]
u F:v <init
)

ints start 0
