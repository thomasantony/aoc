(ns aoc2021.day02)

(require '[clojure.string :as str])

;; Parse command like "forward 5" into a tuple with 
;; a step like (5 0)
(defn parse_ops_to_steps [{op :direction, step_size :step_size}]
    (case op
      "forward" [step_size 0]
      "back" [(- step_size) 0]
      "down" [0 step_size]
      "up" [0 (- step_size)]))

;; Parses input into a list of maps
;; each map is of format {:direction "<>" :step_size <num>}
(defn parse_input
  [input_str]
  (let [lines (str/split-lines input_str)
        commands (map #(str/split % #" ") lines)]
    (map (fn [[dir step]] {
      :direction dir
      :step_size (Math/abs (Integer. step))
    }) commands)
))

(defn part01
  [input]
  (let [commands (parse_input input)
        input_ops (map parse_ops_to_steps commands)
        final_pos (apply map + input_ops)]
    (reduce * final_pos)
  ))

;; (defn part02
;;   [input]
;;   (let [input_nums (parse_input input)
;;         three_wise (partition 3 1 input_nums)
;;         moving_sum (map #(apply + %1) three_wise)
;;         ]
;;       (do_part01 moving_sum)
;;       ))

;; (def demo_input "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n")
;; (part01 demo_input)
;; (part02 "199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
