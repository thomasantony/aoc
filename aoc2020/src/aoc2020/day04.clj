(ns aoc2020.day04
  (:require [clojure.string :as string])
  (:require [aoc2020.utils :as utils]))

(defn parse_input [input_str]
  (->> input_str
       (string/split-lines)        ; Split lines
       (utils/split-by #(= "" %))  ; Split by blank line to separate rows
       (map #(string/join " " %))  ; Join all pieces in each row
       (map string/trim)           ; Trim blank space
       (map #(string/split % #" ")); Split each row into data segments
       ; Convert each row into a map
       (map  #(reduce (fn [data-map data-str]
                        (let [[key val] (string/split data-str #":")]
                          (assoc data-map (keyword key) val)))
                      {} %)))
)

;; Part 01
(def required_keys [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid])
(defn passport_validator_part01
  [passport]
  (every? #(contains? passport %) required_keys)
  )

(defn part01 [input_str]
  (let [passports (parse_input input_str)]
    (->> passports
         (filter passport_validator_part01)
         count)
    ))


;; Part 02

(defn num_validator [min max value]
  (let [num (Integer/parseInt value)]
    (and (>= num min) (<= num max))
    )
  )
  
(defn byr_validator 
  "Validates birth year"
  [value]
  (and (utils/numeric? value) 
       (num_validator 1920 2002 value)))

(defn iyr_validator 
  "Validates issue year"
  [value]
  (and (utils/numeric? value) 
       (num_validator 2010 2020 value)))

(defn eyr_validator 
  "Validates expiry year"
  [value]
  (and (utils/numeric? value) 
       (num_validator 2020 2030 value)))


(defn hgt_in_validator
  "Validates height in inches"
  [value]
  (and (utils/numeric? value) 
       (num_validator 59 76 value)))

(defn hgt_cm_validator
  "Validates height in centimeters"
  [value]
  (and (utils/numeric? value) 
       (num_validator 150 193 value)))

(defn hgt_validator "Validates height"
  [value]
  (let [matches (re-find #"(\d+)(in|cm)" value)]
    (if (some? matches)
      (let [hgt (second matches)
            unit (nth matches 2)]
       (case unit
        "cm" (hgt_cm_validator hgt)
        "in" (hgt_in_validator hgt)))
      false
    )
  ))

(def hcl_validator #(some? (re-matches #"\#[0-9a-f]{6}" %)))

(def valid_ecl ["amb" "blu" "brn" "gry" "grn" "hzl" "oth"])
(def ecl_validator #(.contains valid_ecl %))

(defn pid_validator [value]
  (and (utils/numeric? value) (= (count value) 9)))

(def validators {:byr byr_validator
                 :iyr iyr_validator
                 :eyr eyr_validator
                 :hgt hgt_validator
                 :hcl hcl_validator
                 :ecl ecl_validator
                 :pid pid_validator
                 :cid (fn [_] true)})


(defn validate_passport_data
  "Validates a single key-value pair from a passport"
  [[key value]]
  ((validators key) value))
  
(defn passport_validator_part02 [passport]
  (every? validate_passport_data passport)
  )

(defn part02 [input_str] 
  (let [passports (parse_input input_str)]
    (->> passports
        (filter passport_validator_part01)
        (filter passport_validator_part02)
        count)
    )
  )
