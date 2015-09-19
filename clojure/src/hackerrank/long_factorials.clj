(ns hackerrank.long-factorials)

(let [res (reduce * (range (biginteger 1) (inc (biginteger (read-line)))))]
  (println (apply str (butlast (str res)))))
