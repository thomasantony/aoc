(ns aoc2020.day01-test
  (:require [clojure.test :refer :all]
            [aoc2020.day01 :refer :all]))


(def demo_input "1721\n979\n366\n299\n675\n1456\n")

(deftest part01-test
  (testing "Day 01 Part 1"
    (let [output (part01 demo_input)]
         (is (= output 514579)))))

(deftest part02-test
  (testing "Day 01 Part 2"
    (let [output (part02 demo_input)]
         (is (= output 241861950)))))
