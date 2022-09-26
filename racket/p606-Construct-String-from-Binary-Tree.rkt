#lang racket

; Definition for a binary tree node.

; val : integer?
; left : (or/c tree-node? #f)
; right : (or/c tree-node? #f)
(struct tree-node
  (val left right) #:mutable #:transparent)

; constructor
(define (make-tree-node [val 0])
  (tree-node val #f #f))

(define/contract (tree2str root)
  (-> (or/c tree-node? #f) string?)
  (let [(val (tree-node-val root)) (left (tree-node-left root)) (right (tree-node-right root))]
    (match (list left right)
      [(list #f #f) (~a val)]
      [(list #f (? tree-node? r)) (format "~a()(~a)" val (tree2str right))]
      [(list (? tree-node? l) r) (format "~a(~a)~a" val (tree2str l)
                                         (if r (~a "(" (tree2str r) ")") "")
                                         )]
      )
    ))
