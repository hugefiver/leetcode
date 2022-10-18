#lang racket

(require racket/set)

(define/contract (check-if-pangram sentence)
  (-> string? boolean?)
  (let [(set (mutable-set))]
    (for [(ch (in-string sentence))]
      (set-add! set ch))
    (eq? (set-count set) 26)))
