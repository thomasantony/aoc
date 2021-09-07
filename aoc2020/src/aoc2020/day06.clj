(ns aoc2020.day06
  (:require [clojure.string :as string])
  (:require clojure.set)
  (:require [aoc2020.utils :as utils]))

(defn parse_input [input_str]
  (->> input_str
       (string/split-lines)        ; Split lines
       (utils/split-by #(= "" %))  ; Split by blank line to separate groups
       (map #(remove string/blank? %))
       )
) 

;; Part 1
(defn part01 [input_str]
  (->> input_str
       parse_input
       (map #(string/join "" %))  ; Join all pieces in each row
       (map #(string/trim %))     ; Trim blank space)
       (map set)
       (map count)
       (apply +)))
       
;; Part 2
(defn part02 [input_str]
  (->> input_str
     parse_input
     ; Convert each person into a set
     (map #(map set %)) 
     ; Find common questions in each group
     (map #(apply clojure.set/intersection %)) 
     (map count)
     (apply +))
  )
