(ns aoc2020.day02-test
  (:require [clojure.test :refer :all]
            [aoc2020.day02 :refer :all]))


(def demo_input "1-3 a: abcde
1-3 a: cbade
1-3 b: cdefg
2-9 c: ccccccccc")

(def demo_data (parse_input demo_input))

(deftest part01-reqparse-test
  (testing "Day 02 Part 1 Requirement parser"
    (let [output (parse_part01_password_reqs "1-2 c")]
         (is (= output {:min 1, :max 2, :char \c})))))

(deftest part01-test
  (testing "Day 02 Part 1"
    (let [output (part01 demo_input)]
         (is (= output 3)))))


(deftest part02-test
  (testing "Day 02 Part 2 Password length checker"
    (let [output_1 (check_passwd_part2_len? (nth demo_data 1))
          output_2 (check_passwd_part2_len? (nth demo_data 2))]
         (is (and (= output_1 true) (= output_2 false))))))


(deftest part02-test
  (testing "Day 02 Part 2"
    (let [output (part02 demo_input)]
         (is (= output 2)))))
