(ns hackerrank.prefix-compression)

(defn prefix-compression [str1 str2]
  (let [prefix-compression-helper
        (fn [str1 str2 acc]
          (if (or (empty? str1)
                  (empty? str2)
                  (not (= (first str1) (first str2))))
            {:str1 (apply str str1)
             :str2 (apply str str2)
             :acc (apply str (reverse acc))}
            (recur (rest str1)
                   (rest str2)
                   (cons (first str1) acc))))]
    (prefix-compression-helper str1 str2 '())))
