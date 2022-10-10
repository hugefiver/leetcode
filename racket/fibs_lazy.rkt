#lang racket/base

(require racket/contract
         racket/promise
         racket/match)


(define/contract (lazy_cons car cdr)
  (-> any/c (promise/c pair?) (promise/c pair?))
  (delay (cons car cdr)))

(define/contract (take_lazy n lst)
  (-> (and/c integer? (>=/c 0)) (promise/c pair?) pair?)
  (match n
    [0 '()]
    [1 (cons (car (force lst)) '())]
    [else (cons (car (force lst)) (take_lazy (- n 1) (cdr (force lst))))]
    ))

;;; (define/contract (map_lazy op lst1 lst2)
;;;   (-> procedure? (promise/c pair?) (promise/c pair?) promise?)
;;;   (let [(l1 (force lst1)) (l2 (force lst2))]
;;;     (cond
;;;       [(or (eq? l1 '()) (eq? l2 '())) (delay '())]
;;;       [else (let [(a1 (car l1)) (a2 (car l2))]
;;;               (delay (cons (op a1 a2)
;;;                            (delay (map_lazy op (cdr l1) (cdr l2))))))])))
(define/contract (map_lazy op lst1 lst2)
  (-> procedure? (promise/c pair?) (promise/c pair?) promise?)
  (cond
    [(or (eq? (force lst1) '()) (eq? (force lst2) '())) (delay '())]
    [else (let [(a1 (car (force lst1))) (a2 (car (force lst2)))]
            (delay (cons (op a1 a2)
                         (delay (map_lazy op (cdr (force lst1)) (cdr (force lst2)))))))]))

(letrec [(fibs (lazy (lazy_cons 1 (lazy_cons 1 (map_lazy + fibs (lazy (cdr (force fibs))))))))]
  (take_lazy 10 fibs))
