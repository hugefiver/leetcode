#lang racket

; val : integer?
; next : (or/c list-node? #f)
(struct list-node
  (val next) #:mutable #:transparent)

; constructor
(define (make-list-node [val 0])
  (list-node val #f))


;; need pass argument as reference
(define/contract (travel! root n)
  (-> (or/c list-node? #f) integer? integer?)
  (match root
    [#f (quotient (n . + . 1) 2)]
    [node (let [(r (travel! (list-node-next node) (+ n 1)))]
            (case r
              ['(0) ((set-list-node-next!
                      node
                      (list-node-next (list-node-next node)))
                     -1)]
              [else (- r 1)]
              ))]
    ))

(define/contract (delete-middle head)
  (-> (or/c list-node? #f) (or/c list-node? #f))
  (if (false? (list-node-next head))
      #f
      (let ()
        (travel! head 0)
        head)
      ))
