#lang racket

; val : integer?
; left : (or/c tree-node? #f)
; right : (or/c tree-node? #f)
(struct tree-node
  (val left right) #:mutable #:transparent)

; constructor
(define (make-tree-node [val 0])
  (tree-node val #f #f))

;; start from here

(define (make-node val left right)
  (tree-node val left right))

(define/contract (add-one-row root val depth)
  (-> (or/c tree-node? #f) exact-integer? exact-integer? (or/c tree-node? #f))
  (let [(left (delay (tree-node-left root))) (right (delay (tree-node-right root)))]
    (cond
      [(false? root) #f]
      [(= depth 1) (make-node val root #f)]
      [(= depth 2) (make-node (tree-node-val root) (make-node val (force left) #f) (make-node val #f (force right)))]
      [else (make-node
             (tree-node-val root)
             (add-one-row (force left) val (- depth 1))
             (add-one-row (force right) val (- depth 1)))]))
  )