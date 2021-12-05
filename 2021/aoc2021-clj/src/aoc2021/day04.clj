(ns aoc2021.day04)

(require '[clojure.string :as str])

(defn index-of-pred [pred coll]
  (first (keep-indexed #(when (pred %2) %1) coll)))


;; Board data struct: {:rows {0: [xx xx xx], 1: [yy yy yy]}, :columns, :numbers {  22: (0, 0), 33: (0, 1) ... } }
;; 
;; With each play, lookup row and column number from ":numbers"
;; If number exists, increment corresponding number in row and col lists
;; Check for win by seeing if any of them hit 5

(defn parse_next_board
  "Pulls in seq of five strings and parses out board"
  [board_lines]
  (merge (apply merge (map-indexed (fn [row line]
                 (let [numbers (->> line
                                    (#(str/split % #"\s"))
                                    (remove str/blank?)
                                    (map read-string))]
                   (apply merge (map-indexed (fn [col num]
                                  {num [row, col]}) numbers))))
               board_lines)) {:rows [0 0 0 0 0] :cols [0 0 0 0 0]}))

(defn parse_input [input_str]
  (let [lines (str/split-lines input_str)
        input_numbers (map #(Integer. %)  (str/split (first lines) #","))
        lines (drop 2 lines)
        board_defs (partition 5 6 lines)]
    
    {:boards (map parse_next_board board_defs)
     :moves input_numbers}))

(defn play_number
  "Looks up given number in board and adds it to the `row` or `col` lists if it exists"
  [board number]
  (if (contains? board number)
    (let [[row col] (get board number)]
      (-> board
          (update-in [:rows row] inc)
          (update-in [:cols col] inc))
    )
    board
    )
  )

(defn is-board-winner? 
  "Returns true if board has a winning row/col"
  [board]
  (or (.contains (:rows board) 5) (.contains (:cols board) 5))
  )

(defn long-str [& strings] (str/join "\n" strings))
(def demo_input (long-str "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"))
""
"22 13 17 11  0"
" 8  2 23  4 24"
"21  9 14 16  7"
" 6 10  3 18  5"
" 1 12 20 15 19"
""
" 3 15  0  2 22"
" 9 18 13 17  5"
"19  8  7 25 23"
"20 11 10 24  4"
"14 21 16 12  6"
""
"14 21 17 24  4"
"10 16 15  9 19"
"18  8 23 26 20"
"22 11 13  6  5"
" 2  0 12  3  7"

(parse_input demo_input)
(defn part01 [input_str]
  (let [{boards :boards
         moves :moves} (parse_input input_str)]
    (loop [boards boards
           moves moves]
      (if )
      )
    ))


(part01 demo_input)
