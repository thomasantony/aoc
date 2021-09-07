(ns aoc2020.day05-test
  (:require [clojure.test :refer :all]
            [aoc2020.day05 :refer :all]))

(deftest part01-test-get-seat-info
  (testing "Day 05 Part 1 - Get seat info"
    (let [inputs ["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
          outputs [[44 5 357], [70 7 567], [14 7 119], [102 4 820]]]
          (is (= outputs (map get_seat_info inputs)))
    )))
