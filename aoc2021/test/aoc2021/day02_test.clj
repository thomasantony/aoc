(ns aoc2021.day02-test
  (:require [clojure.test :refer :all]
            [aoc2021.day02 :refer :all]))

(def demo_input "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n")

(deftest part01-test
  (testing "Day 02 Part 1"
    (let [output (part01 demo_input)]
      (is (= output 150)))))

;; (deftest part02-test
;;   (testing "Day 02 Part 2"
;;     (let [output (part02 demo_input)]
;;       (is (= output 5)))))
