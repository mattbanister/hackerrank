(ns hackerrank.string-compression)

(defn compress-string [input]
  (let [compress-string-helper
        (fn [input memo acc]
          (if (empty? input)
            acc
            (recur (rest input)
                   (first input)
                   (if (= (first input) memo)
                     (cons [(first (first acc)) (inc (second (first acc)))] (rest acc))
                     (cons [(first input) 1] acc)))))]
    (apply str (filter #(not (= % 1)) (flatten (reverse (compress-string-helper input nil '())))))))
