(ns aoc2020.day06-test
  (:require [clojure.test :refer :all]
            [aoc2020.day06 :refer :all]))


(def demo_input "abc

a
b
c

ab
ac

a
a
a
a

b")

(deftest part01-test
  (testing "Day 06 Part 1"
    (let [inputs [demo_input]
          outputs [11]]
          (is (= outputs (map part01 inputs)))
    )))

(deftest part01-test
  (testing "Day 06 Part 2"
    (let [inputs [demo_input]
          outputs [6]]
          (is (= outputs (map part02 inputs)))
    )))
