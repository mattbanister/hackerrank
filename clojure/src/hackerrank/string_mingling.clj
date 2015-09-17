(ns hackerrank.string-mingling)

(defn mingle [str1 str2]
  (apply str (interleave str1 str2)))

(defn transpose-chars [input]
  (apply str (flatten (map reverse (partition 2 input)))))
