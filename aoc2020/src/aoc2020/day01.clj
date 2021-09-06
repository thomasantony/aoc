(ns aoc2020.day01)


(defn parse_input
  [input_str]
  (map #(Integer. %) (clojure.string/split-lines input_str))
)

(defn part01
  [input]
  (let [input_nums (parse_input input)]
    (first (for [n1 input_nums
                 n2 input_nums
                 :when (= 2020 (+ n1 n2))] (* n1 n2))))
  )

(defn part02
  [input]
  (let [input_nums (parse_input input)]
    (first (for [n1 input_nums
                 n2 input_nums
                 n3 input_nums
                 :when (= 2020 (+ n1 n2 n3))] (* n1 n2 n3))))
  )

;; (part01 "1721\n979\n366\n299\n675\n1456\n")
;; (part01 (slurp "inputs/day01.txt"))

;; (part02 "1721\n979\n366\n299\n675\n1456\n")
;; (part02 (slurp "inputs/day01.txt"))
