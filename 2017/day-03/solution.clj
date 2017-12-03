(defn lower_root [n] (int (Math/floor (Math/sqrt n))))

(defn box_size [n] (if (even? (lower_root n)) (- (lower_root n) 1) (lower_root n)))

(defn half_box [n] (int (/ (box_size n) 2)))

(defn sq [n] (* n n))

(defn side_num [n]
  (mod
    (- n (sq (box_size n)))
    (+ (box_size n) 1))
  )

(defn part1 [n]
  (if (= n 1)
    0
    (+ 1 (half_box n) (Math/abs (- (side_num n) (half_box n) 1)))
    )
  )

(println "Part 1 including samples")
(println (part1 1))
(println (part1 12))
(println (part1 23))
(println (part1 1024))
(println (part1 361527))
