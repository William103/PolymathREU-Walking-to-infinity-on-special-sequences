(define (fold f init seq)
  (if (null? seq)
      init
      (fold f
            (f (car seq) init)
            (cdr seq))))

(define (range n)
  (let loop ((i (- n 1)) (ls '()))
    (if (< i 0) ls
        (loop (- i 1) (cons i ls)))))

(define (compose f g)
  (lambda (x) (f (g x))))

(define (repeated f n)
  (let loop ((i 0) (func (lambda (x) x)))
    (if (= i n)
        func
        (loop (+ i 1) (compose func f)))))
