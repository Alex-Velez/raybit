==== ==== ====
cont digi num
==== ==== ====

+
[
 -                         cont=0
 >,
 ======SUB10======
 ----------
 
 [                         not 10
  <+>                      cont=1
  =====SUB38======
  ----------
  ----------
  ----------
  --------

  >
  =====MUL10=======
  [>+>+<<-]>>[<<+>>-]<     dup

  >>>+++++++++
  [
   <<<
   [>+>+<<-]>>[<<+>>-]<    dup
   [<<+>>-]
   >>-
  ]
  <<<[-]<
  ======RMOVE1======
  <
  [>+<-]
 ]
 <
]
>>[<<+>>-]<<
#.!123

