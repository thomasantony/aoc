(ns aoc2021.day03)

(require '[clojure.string :as str])

(defn compute_epsilon_bits [bit_sums num_lines]
  (map (fn [bit_sum] 
    (let [bit_x2 (* 2 bit_sum)]
      (cond
      (< bit_x2 num_lines) 0  ;; zeros more common
      (>= bit_x2 num_lines) 1)  ;; ones more common
      )) bit_sums)
)

(defn bits_to_int [bits]
   (first (reduce (fn [[num, pow] bit]
      [(+ num (* bit pow)) (* 2 pow)])
      [0 1]
      (reverse bits)))
)

(defn parse_bits [lines]
(let [bits (map #(str/split %1 #"") lines)]
  (map (fn [b] (map #(Integer/parseInt %1) b)) bits))
)

(defn part01
  [input_str]
  (let [lines (str/split-lines input_str)
        num_lines (count lines)
        bits (parse_bits lines)
        bit_sums (apply map + bits)
        epsilon_bits (compute_epsilon_bits bit_sums num_lines)
        gamma_bits (map #(cond (= % 1) 0 :else 1) epsilon_bits)
        epsilon (bits_to_int epsilon_bits)
        gamma (bits_to_int gamma_bits)
        ]
        (* gamma epsilon)
))

;; Part 2

;; Returns whether one or zero is the most common in
;; given seq of bits
(defn compute_most_common_bit [bits]
  (let [bit_sum (apply + bits)
        num_bits (count bits)]
    (cond
      (< (* 2 bit_sum) num_bits) 0  ;; zeros more common
      :else 1)))

(defn compute_o2_rating [bits]
  
  (let [n (count (first bits))] (loop [i 0
         bits bits]
    (if (and (< i n) (> (count bits) 1))
      (let [ith_bits (map #(nth %1 i) bits)
            most_commmon_bit (compute_most_common_bit ith_bits)]

        (recur (inc i) (filter #(= most_commmon_bit (nth % i)) bits)))
      (bits_to_int (flatten bits)) ;; return final value at the end
      ))))

(defn compute_co2_rating [bits]
  (let [n (count (first bits))] 
    (loop [i 0
          bits bits]
     (if (and (< i n) (> (count bits) 1))
       (let [ith_bits (map #(nth %1 i) bits)
             most_commmon_bit (compute_most_common_bit ith_bits)
             least_common_bit (if (= most_commmon_bit 0) 1 0)]
         (recur (inc i) (filter #(= least_common_bit (nth % i)) bits)))
       (bits_to_int (flatten bits)) ;; return final value at the end
       ))))

(defn part02
  [input_str]
  (let [lines (str/split-lines input_str)
        bits (parse_bits lines)
        o2_rating (compute_o2_rating bits)
        co2_rating (compute_co2_rating bits)
        ]
    (* o2_rating co2_rating)
   ))

;; (def demo_input "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")
;; (part01 demo_input)
;; (part02 demo_input)
