(ns aoc2020.day06
  (:require [clojure.string :as string])
  (:require [clojure.edn :as edn])
  (:require [aoc2020.utils :as utils]))

(def test_input "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.")


(defn parse_bag_color_and_count [bag_str]
  (take-last 2 (re-matches #"(no|\d+) (\w+ \w+) bags?\.?" bag_str))
  )

(defn parse_bag_color [bag_str]
  (second (re-matches #"(\w+ \w+) bags?" bag_str))
  )

(defn parse_input [input_str]
  (let [preprocessed (->> input_str
                          (string/split-lines)
                          (map #(string/split % #" contain ")))
        containers (map first preprocessed)
        contents (map #(string/split % #", ") (map second preprocessed))]
          (zipmap (map parse_bag_color containers)
            (map (fn [line]
                   (->> line
                        (map parse_bag_color_and_count)
                        (map #(zipmap [:num :color] %))
                        (remove empty?)
                        (map #(update-in % [:num] edn/read-string)))) contents)))
)


;; Assume no cycles in graph and remove 
;; all colors "below" shiny-gold
(def any? (comp not not-any?))

(defn dig_for_color [color_map query_color container]
 (let [colors_to_remove (cons query_color (map :color (color_map query_color)))
       color_map (dissoc color_map colors_to_remove)
       bags (get color_map container)]
    (if (empty? bags)
      false
      (let [bag_colors (map :color bags)]
       (if (.contains bag_colors query_color)
         true
        (any? identity (map #(dig_for_color color_map query_color %) bag_colors))
         )
      )
    )
  )
)

(dig_for_color color_map query_color "dark olive")

(defn part01 [input_str]
  (let [color_map (parse_input input_str)
        query_color "shiny gold"]
   (->>  color_map
    (map (fn [[container _]] (dig_for_color color_map query_color container)))
    (filter identity)
     count
    ))
)

(part01 test_input)

(part01 (slurp "inputs/day07.txt"))
