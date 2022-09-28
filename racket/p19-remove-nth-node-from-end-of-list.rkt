#lang racket/base

; Definition for singly-linked list:

; val : integer?
; next : (or/c list-node? #f)
(struct list-node
  (val next) #:mutable #:transparent)

; constructor
(define (make-list-node [val 0])
  (list-node val #f))
;; end define list


(require racket/list)
(require racket/contract)

(define (linked-list->numbers ll)
  (if (not (list-node? ll))
      '()
      (append (list (list-node-val ll))
              (linked-list->numbers (list-node-next ll)))))

(define (numbers->linked-list numbers)
  (if (empty? numbers) #f (foldr list-node #f numbers)))

(define (drop-nth-right ls n)
  (append (take ls (- (length ls) n))
          (take-right ls (sub1 n))))

(define/contract (remove-nth-from-end head n)
  (-> (or/c list-node? #f) exact-integer? (or/c list-node? #f))
    (numbers->linked-list (drop-nth-right (linked-list->numbers head) n))
  )

