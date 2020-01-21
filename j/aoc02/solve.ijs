require 'format/printf'
raw =: freads 'input'

ints =: }:__".><;._1',',raw
smoutput '$ints'; $ints

exints =: 1,9,10,3,2,3,11,0,99,30,40,50
smoutput '$exints'; $exints
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
op=.get_op a
_2 Z: (a=99)
NB. Op needs to determine arguments!
a=.x 1 arg y
b=.x 2 arg y
at=.((>y)+3)}(>x)
r =. a op`:0 b
smoutput ((0 1 2 3)+(>y)){(>x)
smoutput a; op; b; '=' ; r ; '@' ; at
(y+4);r at} x
)

u=: 3 : 0
<(>1}>y) once (>{.>y)
)

NB. x is Intcode
NB. y is instruction pointer (0)
start =: 4 : 0
init=.y;x
v=.]
u F:v <init
)

NB. Part 1
ints=:12 2 (1 2)} ints
pps =. 9!:11	NB. print precision set
pps 9
>ints start 0
