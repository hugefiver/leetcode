;; d老师写的

(define/contract (first-missing-positive nums)
  (-> (listof exact-integer?) exact-integer?)
  (let* ([n (length nums)]
         [vec (list->vector nums)])
    (for ([i (in-range n)])
      (let loop ()
        (let ([current (vector-ref vec i)])
          (when (and (<= 1 current) 
                     (<= current n)
                     (not (= current (vector-ref vec (- current 1)))))
            (let ([target-index (- current 1)])
              (let ([temp (vector-ref vec i)])
                (vector-set! vec i (vector-ref vec target-index))
                (vector-set! vec target-index temp)))
            (loop)))))
    (or (for/first ([i (in-range n)]
                    #:unless (= (vector-ref vec i) (+ i 1)))
          (+ i 1))
        (+ n 1))))
