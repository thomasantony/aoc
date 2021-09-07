(ns aoc2020.core
  (:require [aoc2020.day01 :as day01])
  (:require [aoc2020.day02 :as day02])
  (:require [aoc2020.day03 :as day03])
  (:require [aoc2020.day04 :as day04])
  (:gen-class))

(defn -main
  [& args]
  ;; (println "Day 01")
  ;; (println "Part 1 -" (day01/part01 (slurp "inputs/day01.txt")))
  ;; (println "Part 2 -" (day01/part02 (slurp "inputs/day01.txt")))

  ;; (println "Day 02")
  ;; (println "Part 1 -" (day02/part01 (slurp "inputs/day02.txt")))
  ;; (println "Part 2 -" (day02/part02 (slurp "inputs/day02.txt")))

  ;; (println "Day 03")
  ;; (println "Part 1 -" (day03/part01 (slurp "inputs/day03.txt")))
  ;; (println "Part 2 -" (day03/part02 (slurp "inputs/day03.txt")))
  
  (println "Day 04")
  (println "Part 1 -" (day04/part01 (slurp "inputs/day04.txt")))
  (println "Part 2 -" (day04/part02 (slurp "inputs/day04.txt")))
)
(comment
  (-main)
  )
