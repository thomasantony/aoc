(ns aoc2020.day04-test
  (:require [clojure.test :refer :all]
            [aoc2020.day04 :refer :all]))



(def demo_input "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in")

(def demo_invalid_input "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007")

(def demo_valid_input "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")

(def demo_data (parse_input demo_input))
(def demo_valid_passports (parse_input demo_valid_input))
(def demo_invalid_passports (parse_input demo_invalid_input))

(deftest part01-test
  (testing "Day 04 Part 1"
    (let [output (part01 demo_input)]
         (is (= output 2)))))


(deftest part02-test-byr
  (testing "Day 04 Part 2 - byr validator"
    (let [inputs ["1920" "2002" "1919" "2003" "abcd"]
          outputs [true true false false false]]
          (is (= outputs (map byr_validator inputs)))
      
    )))

(deftest part02-test-iyr
  (testing "Day 04 Part 2 - iyr validator"
    (let [inputs ["2010" "2020" "2009" "2021" "abcd"]
          outputs [true true false false false]]
          (is (= outputs (map iyr_validator inputs)))
      
    )))

(deftest part02-test-eyr
  (testing "Day 04 Part 2 - eyr validator"
    (let [inputs ["2020" "2030" "2019" "2031" "abcd"]
          outputs [true true false false false]]
          (is (= outputs (map eyr_validator inputs)))
      
    )))

(deftest part02-test-hgt
  (testing "Day 04 Part 2 - hgt validator"
    (let [inputs ["60in", "190cm", "190in", "190"]
          outputs [true true false false]]
          (is (= outputs (map hgt_validator inputs)))
      
    )))

(deftest part02-test-hcl
  (testing "Day 04 Part 2 - hcl validator"
    (let [inputs ["#123abc", "#123abz", "123abc", "#123"]
          outputs [true false false false]]
          (is (= outputs (map hcl_validator inputs)))
      
    )))

(deftest part02-test-ecl
  (testing "Day 04 Part 2 - ecl validator"
    (let [inputs ["brn", "wat"]
          outputs [true false]]
          (is (= outputs (map ecl_validator inputs)))
      
    )))

(deftest part02-test-pid
  (testing "Day 04 Part 2 - pid validator"
    (let [inputs ["000000001", "0123456789", "123"]
          outputs [true false false]]
          (is (= outputs (map pid_validator inputs)))
      
    )))

(deftest part02-test
  (testing "Day 04 Part 2"
    (let [inputs [demo_input demo_valid_input demo_invalid_input]
          outputs [2 4 0]]
      (is (= outputs (map part02 inputs)))
    )))
