(ns aoc2021.day02)

(require '[clojure.string :as str])

;; Parse command like "forward 5" into a tuple with 
;; a step like (5 0)
(defn parse_ops_to_steps [{op :direction, step_size :step_size}]
    (case op
      "forward" [step_size 0]
      "back" [(- step_size) 0]
      "down" [0 step_size]
      "up" [0 (- step_size)]))

;; Parses input into a list of maps
;; each map is of format {:direction "<>" :step_size <num>}
(defn parse_input
  [input_str]
  (let [lines (str/split-lines input_str)
        commands (map #(str/split % #" ") lines)]
    (map (fn [[dir step]] {
      :direction dir
      :step_size (Math/abs (Integer. step))
    }) commands)
))

(defn part01
  [input]
  (let [commands (parse_input input)
        input_ops (map parse_ops_to_steps commands)
        final_pos (apply map + input_ops)]
    (reduce * final_pos)
  ))

;; Computes the "aim" value and adds it to each map in the list `commands`
(defn compute_aim [commands]
  (let [aim (drop 1 (reductions  #(case (:direction %2)
                                  "up" (- %1 (:step_size %2))
                                  "down" (+ %1 (:step_size %2))
                                  %1)
                               0 commands))]
      (map (fn [aim cmd] (assoc cmd :aim aim)) aim commands)  
))


(defn compute_step_from_commands [{op :direction, step_size :step_size, aim :aim}]
  (case op
    "forward" [step_size (* aim step_size)]
    "back" [(- step_size) 0]
    "up" [0 0]
    "down" [0 0]))

(defn parse_ops_to_steps_with_aim [commands]
   (let [commands_with_aim (compute_aim commands)
         ]
         (map compute_step_from_commands commands_with_aim)
))

(defn part02
  [input]
  (let [commands (parse_input input)
        steps (parse_ops_to_steps_with_aim commands)
        final_pos (apply map + steps)
        ]
        (reduce * final_pos)
  ))

;; (def demo_input "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n")
;; (part01 demo_input)
;; (part02 demo_input)
