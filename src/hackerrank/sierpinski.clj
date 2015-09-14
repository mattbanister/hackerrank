(ns hackerrank.sierpinski)

;; for example to print the 3rd iteration
;; > (print-board (sierpinski 3))

;; ui funcs

(defn print-board
  [board]
  (doseq [row board]
    (println (apply str row))))

;; board funcs

(defn initial-point
  [total-width row column]
  (let [center (/ (dec total-width) 2)
        in-range? #(and (>= % (- center row)) (<= % (+ center row)))]
    (if (in-range? column)
      "1"
      "_")))

(defn initial-row
  [total-width row]
  (map #(initial-point total-width row %) (range 0 total-width)))

(defn initial-board
  [num-rows]
  (let [total-width (dec (* num-rows 2))]
    (map #(initial-row total-width %) (range 0 num-rows))))

(defn fold-board
  [board num-folds]
  (let [partition-size (/ (count board) num-folds)
        width (count (first board))]
    (mapcat identity
            (interleave
             (repeat (/ num-folds 2)
                     (repeat partition-size (repeat width "_")))
             (map reverse
                  (take-nth 2
                            (partition partition-size board)))))))

(defn combine-row [board-row mask-row]
  (map (fn [board-point mask-point]
         (if (and (= board-point "1")
                  (= mask-point "1"))
           "_"
           board-point)) board-row mask-row))

(defn sierpinski [total-iterations]
  (let [sierpinski-helper
        (fn [board total-iterations iteration]
          (let [num-folds (int (java.lang.Math/pow 2 (inc iteration)))
                next-board-mask (fold-board board num-folds)
                next-board (map combine-row board next-board-mask)]
            (if (= total-iterations iteration)
              board
              (recur next-board total-iterations (inc iteration)))))]
    (sierpinski-helper (initial-board 32) total-iterations 0)))
