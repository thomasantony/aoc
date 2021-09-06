(ns aoc2020.day03-test
  (:require [clojure.test :refer :all]
            [aoc2020.day03 :refer :all]))


(def demo_input "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#")

(def demo_data (parse_input demo_input))

(deftest part01-test
  (testing "Day 03 Part 1"
    (let [output (part01 demo_input)]
         (is (= output 7)))))


(deftest part02-test
  (testing "Day 03 Part 2"
    (let [output (part02 demo_input)]
      (is (= output 336)))))
