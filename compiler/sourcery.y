%token newline indent type if else channel colon leftarrow rightarrow spell ritual cast interrupt skip question identifier
%token forall mine golem whisper crystal entropic say_print element openP closeP openSB closeSB assign attach
%token add subtract comma and or bigger lesser multiply divide equal not_equal power bequal lequal
%token add_assign sub_assign mul_assign div_assign pow_assign and_assign or_assign
%token pre_increment pre_decrement negate not ref of cleanse post_increment post_decrement as
%token integer_literal char_literal float_literal double_literal single_literal long_literal bool_literal string_literal
%%

file: file spell_case
file: file sigil_declare
file: file ritual_case
file: file say
file: file element_case
file: file cond
file: file attachment
file: %empty
attachment: attach string_literal
type_def: colon type
type_def: %empty
do_comp: do_comp do
do_comp %empty
do_sub: do_comp
do_sub: say
do_sub: return
do_sub: stop
do_sub: cond
do_sub: %empty
do: newline indent do_sub newline
spell_case: spell_decoration spell type_def identifier parameter_choice doOrReturn
doOrReturn: return newline
doOrReturn: colon do
return: rightarrow idOrSay
stop: interrupt
stop: interrupt idOrSay
stop: skip
stop: skip idOrSay
parameter_choice: leftarrow pars
parameter_choice: %empty
sigil_assignment: args rightarrow pars
sigil_assignment: pars assign args
sigil_assignment: pars leftarrow args
pars: pars comma identifier type_def
pars: identifier type_def
args: args comma idOrSay
args: idOrSay
sigil_declare: privacy sigil_assignment
privacy: forall
privacy: mine
privacy: %empty
idOrSay: identifier
idOrSay: say
lambda: cast type_def leftarrow pars return
lambda: cast type_def leftarrow pars colon do
lambda: cast type_def return
lambda: cast type_def colon do
print: say_print string_literal
print: say_print string_literal leftarrow args
mal: crystal
mal: entropic
mal: %empty
target: golem
target: whisper
target: %empty
spell_decoration: spell_decoration decoration_group
spell_decoration: decoration_group
decoration_group: privacy
decoration_group: target
cond: if_case
cond: channeling
if_case: if idOrSay doOrReturn
if_case: if idOrSay doOrReturn else_case
else_case: if
else_case: doOrReturn
channeling: channel idOrSay doOrReturn
channeling: channel say say say doOrReturn
channeling: channel identifier colon idOrSay doOrReturn
say: literals
say: idOrSay openSB idOrSay closeSB
say: sigil_assignment
say: cast rightarrow identifier
say: cast rightarrow lambda
say: cast args rightarrow identifier
say: cast args rightarrow lambda
say: lambda
say: binary_exps
say: unary_exps
say: ternary_exps
say: ritual_const
say: print
ternary_exps: idOrSay question idOrSay colon idOrSay
ritual_const: openP args closeP
ritual_const: identifier openP args closeP
ritual_const: ritual openP args closeP
literals: integer_literal
literals: char_literal
literals: float_literal
literals: double_literal
literals: single_literal
literals: long_literal
literals: bool_literal
literals: string_literal
binary_exps: idOrSay add idOrSay
binary_exps: idOrSay subtract idOrSay
//binary_exps: idOrSay comma idOrSay
binary_exps: idOrSay and idOrSay
binary_exps: idOrSay or idOrSay
binary_exps: idOrSay bigger idOrSay
binary_exps: idOrSay lesser idOrSay
binary_exps: idOrSay multiply idOrSay
binary_exps: idOrSay divide idOrSay
binary_exps: idOrSay equal idOrSay
binary_exps: idOrSay not_equal idOrSay
binary_exps: idOrSay power idOrSay
binary_exps: idOrSay bequal idOrSay
binary_exps: idOrSay lequal idOrSay
binary_exps: identifier add_assign idOrSay
binary_exps: identifier sub_assign idOrSay
binary_exps: identifier mul_assign idOrSay
binary_exps: identifier div_assign idOrSay
binary_exps: identifier pow_assign idOrSay
binary_exps: identifier and_assign idOrSay
binary_exps: identifier or_assign idOrSay
unary_exps: pre_increment identifier
unary_exps: pre_decrement identifier
unary_exps: negate idOrSay
unary_exps: not idOrSay
unary_exps: ref idOrSay
unary_exps: of idOrSay
unary_exps: cleanse identifier
unary_exps: identifier post_increment
unary_exps: identifier post_decrement
unary_exps: idOrSay as type
type_list: type_list comma type
type_list: type
element_sub1: identifier leftarrow type_list
element_sub1: identifier
element_sub_comp: element_sub_comp element_sub 
element_sub_comp: %empty
element_sub: element_sub_comp
element_sub: element_sub1
element_sub: %empty
element_case: element identifier colon newline indent element_sub newline
