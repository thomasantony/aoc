(ns aoc2021.core
  (:require [aoc2021.day01 :as day01])
  (:require [aoc2021.day02 :as day02])
  (:require [aoc2021.day03 :as day03])
  ;; (:require [aoc2021.day04 :as day04])
  ;; (:require [aoc2021.day05 :as day05])
  ;; (:require [aoc2021.day06 :as day06])
  (:gen-class))

(defn -main
  [& args]
  ;; (println "Day 01")
  ;; (println "Part 1 -" (day01/part01 (slurp "inputs/day01.txt")))
  ;; (println "Part 2 -" (day01/part02 (slurp "inputs/day01.txt")))

  ;; (println "Day 02")
  ;; (println "Part 1 -" (day02/part01 (slurp "inputs/day02.txt")))
  ;; (println "Part 2 -" (day02/part02 (slurp "inputs/day02.txt")))

  (println "Day 03")
  (println "Part 1 -" (day03/part01 (slurp "inputs/day03.txt")))
  (println "Part 2 -" (day03/part02 (slurp "inputs/day03.txt")))

  ;; (println "Day 04")
  ;; (println "Part 1 -" (day04/part01 (slurp "inputs/day04.txt")))
  ;; (println "Part 2 -" (day04/part02 (slurp "inputs/day04.txt")))

  ;; (println "Day 05")
  ;; (println "Part 1 -" (day05/part01 (slurp "inputs/day05.txt")))
  ;; (println "Part 2 -" (day05/part02 (slurp "inputs/day05.txt")))

  ;; (println "Day 06")
  ;; (println "Part 1 -" (day06/part01 (slurp "inputs/day06.txt")))
  ;; (println "Part 2 -" (day06/part02 (slurp "inputs/day06.txt"))))
)

(comment
  (-main))
