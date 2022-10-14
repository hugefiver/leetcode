#lang racket

; val : integer?
; next : (or/c list-node? #f)
(struct list-node
  (val next) #:mutable #:transparent)

; constructor
(define (make-list-node [val 0])
  (list-node val #f))


(define/contract (travel root n)
  (-> (or/c list-node? #f) integer? (or/c integer? list-node?))
  (match root
    [#f (quotient (n . + . 1) 2)]
    [node (let [(r (travel (list-node-next node) (+ n 1)))]
            (match r
              [0 (let ()
                      (set-list-node-next!
                       node
                       (list-node-next (list-node-next node)))
                      node)]
              [(? integer? x) (- r 1)]
              [else (let ()
                      (set-list-node-next! node r)
                      node)]
              ))]
    ))

(define/contract (delete-middle head)
  (-> (or/c list-node? #f) (or/c list-node? #f))
  (if (false? (list-node-next head))
      #f
      (match (travel head 0)
        [(? list-node? x) x]
        [else #f])
      ))
