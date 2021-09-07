(ns aoc2020.day05
  (:require [clojure.string :as string])
  (:require [aoc2020.utils :as utils]))

(defn parse_input [input_str]
  (string/split-lines input_str))

(defn reducer [lo_c hi_c [lo hi] val]
        (let [halfstep  (/ (- hi lo) 2)]
          (cond 
            (= val lo_c) [lo (- hi halfstep)]
            (= val hi_c) [(- hi halfstep) hi])))

(defn bsp [lo hi lo_c hi_c s] 
  (first (reduce (partial reducer lo_c hi_c) [lo hi] s))
  )

(defn get_seat_info [boarding_pass]
  (let [row (bsp 0 128 \F \B (take 7 boarding_pass))
        col (bsp 0 8 \L \R (take-last 3 boarding_pass))]
    [row col (+ (* row 8) col)]))

(defn get_seat_ids [seat_info_vec]
  (map #(nth % 2) seat_info_vec))

;; Part 1
(defn part01 [input_str]
  (let [boarding_passes (parse_input input_str)
        seat_ids (->> boarding_passes
                    (map get_seat_info)
                    get_seat_ids)]
         (apply max seat_ids)
         )
    )

;; Part 2
(defn compute-diffs [seat_ids]
  (map #(apply - %) (partition 2 1 seat_ids)))

(defn part02 [input_str]
  (let [boarding_passes (parse_input input_str)
        sorted_seat_ids (->> boarding_passes
                      (map get_seat_info)
                      get_seat_ids
                      sort)
        seat_index (->> sorted_seat_ids
                        compute-diffs
                        (map-indexed vector)
                        (filter (fn [[_ delta]] (not= delta -1)))
                        first
                        first
                        )]
         (inc (nth sorted_seat_ids seat_index))
         )
    )

(part02 (slurp "inputs/day05.txt"))
