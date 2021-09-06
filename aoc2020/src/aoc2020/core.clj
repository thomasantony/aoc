(ns aoc2020.core
  (:require [aoc2020.day01 :as day01])
  (:gen-class))

(defn -main
  [& args]
  (println "Day 01")
  (println "Part 1 -" (day01/part01 (slurp "inputs/day01.txt")))
  (println "Part 2 -" (day01/part02 (slurp "inputs/day01.txt")))
  ) 

(comment 
  (-main)
  )
