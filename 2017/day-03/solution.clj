(defn lower_root [n] (int (Math/floor (Math/sqrt n))))

(defn box_size [n] (
                    let
                    [box (lower_root n)]
                         (if (even? box) (dec box) box)))

(defn half_box [n] (int (/ (box_size n) 2)))

(defn sq [n] (* n n))

(defn side_num [n]
  (mod
    (- n (sq (box_size n)))
    (inc (box_size n))))

(defn part1 [n]
  (if (= n 1)
    0
    (+ 1 (half_box n) (Math/abs (- (side_num n) (half_box n) 1)))
    ))

(println "Part 1 including samples")
(println (part1 1))
(println (part1 12))
(println (part1 23))
(println (part1 1024))
(println (part1 361527))

(def directions
  "Infite seq of directions through
  spiral: :right :up :left :left :down, etc."
  (let [dirs (cycle [[:right :up] [:left :down]])
        amount (map inc (range))]
    (mapcat (fn [[d1 d2] amount]
              (concat (repeat amount d1)
                      (repeat amount d2)))
            dirs
            amount)))

(defn next-tile
  "Calculates (n+1)th tile from nth tile"
  [tile direction]
  (let [[axis delta]
        (case direction
          :right [:x 1]
          :left  [:x -1]
          :down  [:y 1]
          :up    [:y -1])]
    (update tile axis + delta)))

(defn sum-of-neighbours
  "Sum of neighbouring tiles"
  [{:keys [x y]}
   tiles]
  (let [neighbour-positions
        (for [dx [-1 0 1]
              dy [-1 0 1]
              :when (not= 0 dx dy)]
          [(+ x dx)
           (+ y dy)])
        neighbours (keep #(get tiles %)
                         neighbour-positions)]
    (reduce +' (map :v neighbours))))

(defn tile-with-bigger-sum
  "Returns first tile with sum > n"
  [n]
  (let [init-tile {:x 0 :y 0 :v 1}]
    (loop [tile init-tile
           tiles {[0 0] init-tile}
           directions directions]
      (let [next-direction (first directions)
            new-tile (next-tile tile next-direction)
            sum (sum-of-neighbours
                  new-tile
                  tiles)
            new-tile (assoc new-tile :v sum)]
        (if (> sum n)
          new-tile
          (recur new-tile
                 (assoc tiles [(:x new-tile)
                               (:y new-tile)]
                        new-tile)
                 (rest directions)))))))

(defn part2 [n]
  (:v (tile-with-bigger-sum n)))

(println "Part 2 including samples")
(println (part2 1))
(println (part2 12))
(println (part2 23))
(println (part2 1024))
(println (part2 361527))
