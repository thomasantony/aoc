(ns aoc2020.utils)

;; The following split-by builds on top of split-with. Instead of 
;; splitting only the first time pred returns false, it splits (lazily)
;; every time it turns from true to false.
;; Ref: https://clojuredocs.org/clojure.core/split-with#example-57ab7b39e4b0bafd3e2a04dd
(defn split-by [pred coll]
  (lazy-seq
    (when-let [s (seq coll)]
      (let [[xs ys] (split-with pred s)]
        (if (seq xs)
          (cons xs (split-by pred ys))
          (let [!pred (complement pred)
                skip (take-while !pred s)
                others (drop-while !pred s)
                [xs ys] (split-with pred others)]
            (cons (concat skip xs)
                  (split-by pred ys))))))))

(defn numeric? [s] (every? #(Character/isDigit %) s))
