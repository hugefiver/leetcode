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


(define/contract (inorder-traversal root)
  (-> (or/c tree-node? #f) (listof exact-integer?))
  (if (false? root) '()
      (let [(val (tree-node-val root)) (left (tree-node-left root)) (right (tree-node-right root))]
        (append (inorder-traversal left) (list val) (inorder-traversal right) )
        ))
  )