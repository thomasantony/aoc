(ns aoc2020.day03
  (:require [clojure.string :as string]))


(defn parse_input [input_str]
  (string/split-lines input_str))


(defn get_map_width [tree_map] (count (first tree_map)))
(defn get_map_height [tree_map] (count tree_map))

(defn wraparound_pos_x
  "Assumes position is given as x"
  [[x y] width]
  [(mod x width) y])


(defn has_tree_at_pos?
  "Checks if there is a tree at given (x, y)position in tree_map"
  [tree_map pos]
  (let [[x y] (wraparound_pos_x pos (get_map_width tree_map))]
    (= (get-in tree_map [y x]) \#)))


(defn get_step_positions
  [step_x step_y tree_map]
  (for [y (range 0 (get_map_height tree_map) step_y)]
    [(int (* y (/ step_x step_y))) y]))

(defn count_trees_on_slope [tree_map step_x step_y]
  (->> tree_map
        (get_step_positions step_x step_y)
        (filter (partial has_tree_at_pos? tree_map))
        count))

;; Part 1
(defn part01 
  [input_str]
  (let [tree_map (parse_input input_str)]
   (count_trees_on_slope tree_map 3 1)))

;; Part 2
(defn part02
  [input_str]
  (let [tree_map (parse_input input_str)
        step_x_list [1 3 5 7 1]
        step_y_list [1 1 1 1 2]]
    
    (apply * (map 
     (partial count_trees_on_slope tree_map) 
     step_x_list 
     step_y_list))
   ))
