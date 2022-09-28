#lang racket

; Definition for singly-linked list:

; val : integer?
; next : (or/c list-node? #f)
(struct list-node
  (val next) #:mutable #:transparent)

; constructor
(define (make-list-node [val 0])
  (list-node val #f))
;; end define list

(define (my-list-ref lst n)
  (cond
    [(null? lst) #f]
    [(= n 0) (car lst)]
    [else (my-list-ref (cdr lst) (- n 1))]))

(define (nth-from-end head n)
  (let loop ([node head]
             [reversed '()])
    (if (not node)
        (my-list-ref reversed n)
	(loop (list-node-next node) (cons node reversed)))))

(define/contract (remove-nth-from-end head n)
  (-> (or/c list-node? #f) exact-integer? (or/c list-node? #f))
  (let ([node (nth-from-end head n)])
    (when node
      (set-list-node-next! node (if (and (list-node-next node)
                                         (list-node-next (list-node-next node)))
                                    (list-node-next (list-node-next node))
                                    #f)))
    (if node head (if head (list-node-next head) #f))))