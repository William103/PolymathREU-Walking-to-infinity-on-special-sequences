(load "util.scm")

(define (val tree) (car tree))
(define (children tree) (cdr tree))
(define (new-tree val children) 
  (let loop ((ls '()) (children children))
    (if (null? children) (cons val ls)
        (loop (append ls (new-tree (car children) '())) (cdr children)))))
(define (tree val children)
  (new-tree val (map (lambda (x) (new-tree x '())) children)))

(define (tree-step tree)
  (if (null? (children tree))
      (new-tree (val tree) (map (lambda (x) (new-tree x '())) (next (val tree))))
      (new-tree (val tree) (map tree-step (children tree)))))

(define (longest-path tree)
  (cdr (let longest-path ((tree tree))
  (if (null? (children tree))
      (list (val tree))
      (cons (val tree)
            (fold (lambda (curr prev) (if (< (length prev) (length curr)) curr prev))
                  '()
                  (map longest-path (children tree))))))))

; --- change this function to control behavior of tree ---
(define (next val)
  (filter prime? (map (lambda (d) (+ (* val 10) d)) (range 10))))

; (define (next val)
;   (filter prime? (map (lambda (d) (+ (* d (expt 10 (digits val))) val)) (cdr (range 10)))))
