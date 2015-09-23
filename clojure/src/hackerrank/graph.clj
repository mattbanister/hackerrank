(ns hackerrank.graph)

(require '[clojure.string :as s])
(require '[clojure.set :as cs])

(defn build-graph [num-nodes edge-list]
  (let [initial-graph
        (reduce (fn [acc idx]
                  (assoc acc (keyword (str idx)) nil))
                {}
                (range 1 (inc num-nodes)))]
    (reduce (fn [acc edge]
              (let [src (first edge)
                    dest (last edge)
                    next-src-vec (into (src acc) (vec (list dest)))
                    next-dest-vec (into (dest acc) (vec (list src)))]
                (assoc (assoc acc dest (vec next-dest-vec)) src (vec next-src-vec))))
            {}
            edge-list)))

(defn find-depth-helper [graph src-set dest visited-set depth]
  (let [next-visited-set (into visited-set src-set)
        next-src-set (cs/difference (set (flatten (map graph src-set))) next-visited-set)]
    (cond
      (src-set dest) depth
      (empty? next-src-set) -1
      :else (recur graph next-src-set dest next-visited-set (inc depth)))))

(defn find-depth [graph src dest]
  (if
    (= src dest) 0
    (find-depth-helper graph (set (src graph)) dest #{} 1)))

(defn next-level [graph src]
  (vec (set (flatten (map graph (src graph))))))

(defn print-result [graph start-node nodes]
  (let [res
        (map
         #(if (> % 0) (* 6 %) %)
         (filter
          #(not (= % 0))
          (map
           #(find-depth graph (keyword start-node) (keyword (str %)))
           (range 1 (inc nodes)))))]
    (doseq [r res] (print (str r " "))) (println "")))

(defn parse-edges [edge-lines]
  (map
   (fn [edge-line]
     (-> edge-line
         (str)
         (#(s/trim %))
         (#(s/split % #" "))
         (#(map keyword %))
         (vec)))
   edge-lines))

(defn -main []
  (let [lines (line-seq (java.io.BufferedReader. *in*))
        num-test-cases (Integer/parseInt (s/trim (first lines)))]
    (loop [x num-test-cases
           remaining (rest lines)]
      (let [edges-and-nodes (s/split (s/trim (str (first remaining))) #" ")
            nodes (Integer/parseInt (first edges-and-nodes))
            edges (Integer/parseInt (last edges-and-nodes))
            edge-list (parse-edges (take edges (rest remaining)))
            start-node (keyword (s/trim (apply str (take 1 (drop (inc edges) remaining)))))
            g (build-graph nodes edge-list)]
        (print-result g start-node nodes)
        (when (> num-test-cases 1)
          (recur (dec num-test-cases) (drop (+ 2 edges) remaining)))))))
