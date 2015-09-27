(ns hackerrank.fibonacci-modified)

(require '[clojure.string :as s])

(let [line (read-line)
      line-parsed (map #(Integer/parseInt %) (s/split line #" "))
      a (first line-parsed)
      b (second line-parsed)
      n (last line-parsed)
      memo (atom {})]
  (doseq [i (range n)]
    (cond
      (= i 0) (swap! memo assoc 0 a)
      (= i 1) (swap! memo assoc 1 b)
      :else (swap! memo assoc i
                   (bigint (+
                            (.pow (java.math.BigInteger. (str (@memo (dec i)))) 2)
                            (@memo (dec (dec i))))))))
  (println (str (@memo (dec n)))))
