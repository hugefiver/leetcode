#lang racket/base

(require racket/contract
         racket/promise
         racket/list
         racket/match
         racket/function)


(define/contract (lazy_cons car cdr)
  (-> any/c (promise/c cons?) (promise/c cons?))
  (delay (cons car cdr)))

(define/contract (take_lazy n lst)
  (-> (and/c integer? (>=/c 0)) (promise/c cons?) cons?)
  (match n
    [0 '()]
    [1 (cons (car (force lst)) '())]
    [else (cons (car (force lst)) (take_lazy (- n 1) (cdr (force lst))))]
    ))

(define/contract (map_lazy op lst1 lst2)
  (-> procedure? (promise/c cons?) (promise/c cons?) promise?)
  (let [(l1 (force lst1)) (l2 (force lst2))]
    (cond
      [(or (eq? l1 '()) (eq? l2 '())) (delay '())]
      [else (let [(a1 (car l1)) (a2 (car l2))]
              (cons (op a1 a2)
                    (delay (map_lazy op (cdr l1) (cdr l2)))))])))

(letrec [(fibs (lazy (lazy_cons 1 (lazy_cons 1 (map_lazy + fibs (lazy (cdr (force fibs))))))))]
  (take_lazy 10 fibs))
