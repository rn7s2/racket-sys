#lang racket/base

(provide fact)

(define (fact x)
  (if (= x 0)
      1
      (* x (fact (- x 1)))))
