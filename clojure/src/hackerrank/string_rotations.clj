(ns hackerrank.string-rotations)

(defn string-rotations [input]
  (let [rotations (take (count input) (partition (count input) 1 (cycle input)))
        rotations-shifted (conj (vec (rest rotations)) (first rotations))]
    (apply str (flatten (interpose " " rotations-shifted)))))
