(ns hackerrank.string-reductions)

(defn string-reductions [input]
  (let [string-reductions-helper
        (fn [input acc]
          (if (empty? input)
            (apply str (reverse acc))
            (recur (rest input)
                   (if ((set acc) (first input))
                     acc
                     (cons (first input) acc)))))]
    (string-reductions-helper input [])))
