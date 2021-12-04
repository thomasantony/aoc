(ns aoc2021.day03-test
  (:require [clojure.test :refer :all]
            [aoc2021.day03 :refer :all]))

(def demo_input "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")
(deftest part01-test
  (testing "Day 03 Part 1"
    (let [output (part01 demo_input)]
      (is (= output 198)))))

(deftest part02-test
  (testing "Day 02 Part 2"
    (let [output (part02 demo_input)]
      (is (= output 230)))))
