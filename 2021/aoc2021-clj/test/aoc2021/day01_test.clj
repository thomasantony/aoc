(ns aoc2021.day01-test
  (:require [clojure.test :refer :all]
            [aoc2021.day01 :refer :all]))


(def demo_input "99\n200\n208\n210\n200\n207\n240\n269\n260\n263")

(deftest part01-test
  (testing "Day 01 Part 1"
    (let [output (part01 demo_input)]
      (is (= output 7)))))

(deftest part02-test
  (testing "Day 01 Part 2"
    (let [output (part02 demo_input)]
      (is (= output 5)))))
