(define (square? n)
  (let ((s (sqrt n)))
    (= s (floor s))))

(define (prime? n)
  (if (< n 2) #f
      (let loop ((i 2))
        (cond
          ((> (* i i) n) #t)
          ((zero? (remainder n i)) #f)
          (else (loop (+ i 1)))))))

(define (square-free? n)
  (if (< n 1) #f
      (let loop ((i 2))
        (cond
          ((> (* i i) n) #t)
          ((zero? (remainder n (* i i))) #f)
          (else (loop (+ i 1)))))))

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

(define (digits n)
  (length (string->list (number->string n))))
