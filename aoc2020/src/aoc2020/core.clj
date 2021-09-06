(ns aoc2020.core
  (:require [aoc2020.day01 :as day01])
  (:require [aoc2020.day02 :as day02])
  (:gen-class))

(defn -main
  [& args]
  ;; (println "Day 01")
  ;; (println "Part 1 -" (day01/part01 (slurp "inputs/day01.txt")))
  ;; (println "Part 2 -" (day01/part02 (slurp "inputs/day01.txt")))
  (println "Day 02")
  (println "Part 1 -" (day02/part01 (slurp "inputs/day02.txt")))
  (println "Part 2 -" (day02/part02 (slurp "inputs/day02.txt")))
  ) 

(comment 
  (-main)
  )
