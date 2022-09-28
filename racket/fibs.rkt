#lang lazy

(letrec [(fibs (list* 1 1 (map + fibs (cdr fibs))))]
  (!!list (take 10 fibs)))