%token i8 i16 i32 i64 f8 f16 f32 f64 string bool as
%token ui8 ui16 ui32 ui64 uf8 uf16 uf32 uf64
%token newline indent spell ritual cast colon rightarrow leftarrow
%token channel if else interrupt skip question identifier sigil await sizeof len
%token forall mine golem whisper crystal entropic say element openP closeP openSB closeSB attach 
%token assign add_assign sub_assign mul_assign div_assign pow_assign and_assign or_assign sright_assign sleft_assign mod_assign
%token dot comma add subtract multiply divide power mod not
%token equal not_equal or and bitwise_xor bitwise_and bitwise_or bitwise_not
%token bitwise_right bitwise_left bigger lesser bigger_equal lesser_equal increment decrement infuse cleanse if_as_if ref of
%token char_literal short_literal int_literal long_literal bif_literal half_literal float_literal double_literal string_literal bool_literal

%left pre15
%right pre14
%left pre14l
%left pre13
%left pre12
%left pre11
%left pre10
%left pre9
%left pre8
%left pre7
%left pre6
%left pre5 
%left pre4 
%left pre3 
%left pre2l
%right pre2 
%left pre1 

%%

file: file attachment newline
    | file say_exp newline
    | file spell_case
    | file sigil_declare
    | file ritual_case
    | file element_case
    | file cond newline
    | %empty
attachment: attach string_literal
par_choice: leftarrow pars
          | %empty
arg_choice: leftarrow args
          | %empty
print: say string_literal arg_choice
type: i8
    | ui8
    | i16
    | ui16
    | i32
    | ui32
    | i64
    | ui64
    | f8
    | uf8
    | f16
    | uf16
    | f32
    | uf32
    | f64
    | uf64
    | string
    | bool 
    | identifier
spell_decorator: privacy target
               | target privacy
               | privacy
               | target
spell_case: spell_decorator spell type_def identifier par_choice do_or_return
          | spell type_def identifier par_choice do_or_return 
type_def: colon type
        | %empty 
do_sub: do_sub do 
      | say_exp newline
      | return newline 
      | stop newline
      | cond newline 
      | newline
do: newline indent do_sub
return: rightarrow say_exp
do_or_return: return
          | colon do
pars: pars comma identifier type_def
    | identifier type_def
args: args comma say_exp
    | say_exp
mal: crystal
   | entropic
target: golem
      | whisper
privacy: forall
        | mine
sigil_declare: privacy sigil_assignment newline
sigil_assignment: args rightarrow sigil mal pars %prec pre14l
                | sigil mal pars assign args %prec pre14
                | sigil mal pars leftarrow args %prec pre14
                | args rightarrow sigil pars %prec pre14 
                | sigil pars assign args %prec pre14
                | sigil pars leftarrow args %prec pre14
lambda: cast type_def leftarrow pars do_or_return
      | cast type_def leftarrow openP closeP do_or_return
cond: if_case
    | channeling
if_case: if say_exp do_or_return
       | if say_exp do_or_return else_case
else_case: else if_case
         | do_or_return
channeling: channel say_exp do_or_return
          | channel sigil_assignment say_exp say_exp do_or_return
          | channel identifier colon say_exp do_or_return
stop: interrupt
    | interrupt say_exp
    | skip
    | skip say_exp
say_exp: literals
       | say_exp openSB say_exp closeSB %prec pre1
       | sigil_assignment
       | spell_cast
       | lambda
       | binary_exps
       | unary_exps
       | say_exp question say_exp colon say_exp %prec pre14
       | ritual_cast
       | print
ritual_cast: identifier leftarrow args %prec pre1
           | identifier leftarrow openP closeP
spell_cast: cast args rightarrow say_exp %prec pre1  
          | cast rightarrow say_exp %prec pre1 
literals: char_literal
        | short_literal
        | int_literal
        | long_literal
        | bif_literal
        | half_literal
        | float_literal
        | double_literal
        | string_literal
        | bool_literal
        | identifier
        | openP say_exp closeP %prec pre1 
binary_exps: say_exp assign say_exp %prec pre14
           | say_exp comma say_exp %prec pre15
           | say_exp dot say_exp %prec pre1 
           | say_exp add say_exp %prec pre4
           | say_exp subtract say_exp %prec pre4
           | say_exp multiply say_exp %prec pre3 
           | say_exp divide say_exp %prec pre3 
           | say_exp mod say_exp %prec pre3
           | say_exp power say_exp %prec pre2l
           | say_exp and say_exp %prec pre12
           | say_exp or say_exp %prec pre13
           | say_exp equal say_exp %prec pre8
           | say_exp not_equal say_exp %prec pre8
           | say_exp bigger say_exp %prec pre7
           | say_exp lesser say_exp %prec pre7 
           | say_exp bigger_equal say_exp %prec pre7 
           | say_exp lesser_equal say_exp %prec pre7 
           | say_exp if_as_if say_exp %prec pre6 
           | say_exp bitwise_or say_exp %prec pre11
           | say_exp bitwise_xor say_exp %prec pre10 
           | say_exp bitwise_and say_exp %prec pre9
           | say_exp bitwise_left say_exp %prec pre5
           | say_exp bitwise_right say_exp %prec pre5
           | say_exp add_assign say_exp %prec pre14
           | say_exp sub_assign say_exp %prec pre14
           | say_exp pow_assign say_exp %prec pre14
           | say_exp mul_assign say_exp %prec pre14
           | say_exp div_assign say_exp %prec pre14
           | say_exp and_assign say_exp %prec pre14
           | say_exp or_assign say_exp %prec pre14
           | say_exp sright_assign say_exp %prec pre14
           | say_exp sleft_assign say_exp %prec pre14
           | say_exp mod_assign say_exp %prec pre14
unary_exps: increment say_exp %prec pre2
          | decrement say_exp %prec pre2 
          | infuse say_exp %prec pre2 
          | subtract say_exp %prec pre2 
          | not say_exp %prec pre2 
          | bitwise_not say_exp %prec pre2 
          | cleanse say_exp %prec pre2 
          | await say_exp %prec pre2 
          | sizeof say_exp %prec pre2
          | len say_exp %prec pre2 
          | ref say_exp %prec pre2
          | of say_exp %prec pre2 
          | say_exp increment %prec pre1 
          | say_exp decrement %prec pre1 
          | say_exp as type %prec pre1 
element_case: element identifier colon element_sub 
element_sub: newline indent element_sub_comp
element_sub_comp: element_sub_comp element_sub
                | identifier type_def newline
                | newline
ritual_case: privacy ritual identifier colon ritual_sub
           | privacy ritual identifier leftarrow pars newline
           | ritual identifier colon ritual_sub_comp
           | ritual identifier leftarrow pars newline
ritual_sub: newline indent ritual_sub_comp
ident_decorator: privacy mal
               | mal privacy 
               | privacy
               | mal
               | %empty
assign_or_not: assign say_exp %prec pre14
             | leftarrow say_exp %prec pre14 
             | %empty
ritual_sub_comp: ritual_sub_comp ritual_sub
               | ident_decorator identifier type_def assign_or_not newline
               | newline
%%






