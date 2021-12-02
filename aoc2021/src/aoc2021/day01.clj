(ns aoc2021.day01)

(defn parse_input
  [input_str]
  (map #(Integer. %) (clojure.string/split-lines input_str)))

(defn do_part01 [input_nums]
  (do
    (def data (partition 2 1 input_nums))
      ;; (apply fn args) "splats" the args into the function
      ;; similar to fn(*args) in python
    (def diffs (map #(apply - %1) data))
      ;; retain negative nums as we are doing first-second
      ;; so increasing numbers will create negative values
    (def increasing (filter neg-int? diffs))
      ;; Count how many remain
    (count increasing)))

(defn part01
  [input]
  (let [input_nums (parse_input input)]
    (do_part01 input_nums)))

(defn part02
  [input]
  (let [input_nums (parse_input input)
        three_wise (partition 3 1 input_nums)
        moving_sum (map #(apply + %1) three_wise)
        ]
      (do_part01 moving_sum)
      ))

;; (part01 "199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
;; (part02 "199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
