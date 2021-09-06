(ns aoc2020.day02
  (:require [clojure.string :as string]))

;; password req as {:char 'm' :min 1 :max 3}
(def req_keys [:min :max])

(defn parse_part01_password_reqs
  "Parses password requirement string"
  [req_str]
  (let [[min_max_str req_char] (string/split req_str #" ")]
    (assoc (zipmap
        req_keys
        (map #(Integer/parseInt %) (string/split min_max_str #"-"))
    ) :char (first req_char))
    )
  )

(defn parse_input
  [input_str]
  (map (fn [[req_str passwd_str]]
         [(parse_part01_password_reqs req_str) passwd_str])
       (map #(string/split % #": ")
            (string/split-lines input_str)))
)

(defn check_passwd
  [[{min_count :min 
    max_count :max
    req_char :char} passwd]]
  (let [passwd_freq (get (frequencies passwd) req_char)]
    (and (some? passwd_freq) (>= passwd_freq min_count) (<= passwd_freq max_count))
  )
)

(defn part01 
  [input_str]
  (->> input_str
       parse_input
       (filter check_passwd)
       count)
)


;; Part 2

;; (def demo_input "1-3 a: abcde
;; 1-3 a: cbade
;; 1-3 b: cdefg
;; 2-9 c: ccccccccc")
;; (def demo_data (parse_input demo_input))


(defn check_passwd_part2_len?
  "Makes sure that pos_a and pos_b are within bounds"
  [[{pos_a :min
     pos_b :max} passwd]]
  (let [len (count passwd)]
        (and (<= pos_a len) (<= pos_b len)))
  )

(defn validate_passwd_part2
  "Checks if given char exists at pos 'a' and does not exist at pos 'b'"
  [[{pos_a :min
     pos_b :max
     req_char :char} passwd]]

  (if check_passwd_part2_len?
    (let [char_a (nth passwd (dec pos_a))
          char_b (nth passwd (dec pos_b))]

    (or (and (= char_a req_char) (not= char_b req_char))
        (and (= char_b req_char) (not= char_a req_char))))
    false)
)

(defn part02
  [input_str]
  (->> input_str
       parse_input
       (filter validate_passwd_part2)
       count)
)
