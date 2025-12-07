import Data.Array
import Data.List
import Data.Maybe
import System.Environment
import System.Exit

main = getArgs >>= parse >>= run

stringToArray :: [Char] -> Array Int Char
stringToArray s = listArray (0, length s - 1) s

run :: String -> IO ()
run input =
  let gridLines = lines input
      startingPoint = findStartingPoint $ head gridLines
      arrLines = Data.List.map stringToArray $ tail gridLines
   in do
        print $ part1 arrLines startingPoint
        print $ part2 arrLines startingPoint

findStartingPoint :: [Char] -> Int
findStartingPoint line = fromJust $ elemIndex 'S' line

part1 arrLines startingPoint =
  let initial = [startingPoint]
   in simulate arrLines 0 initial

checkHit :: Array Int Char -> Int -> [Int]
checkHit line idx = case line ! idx of
  '^' -> (idx - 1) : [idx + 1]
  _ -> [idx]

simulate [] count active = count
simulate lines count active =
  let arr = head lines
      followUp = nub $ concatMap (checkHit arr) active
      hits = length $ Data.List.filter (\i -> arr ! i == '^') active
      remainder = tail lines
   in simulate remainder (hits + count) followUp

part2 :: [Array Int Char] -> Int -> Int
part2 arrLines startingPosition =
  let n = length arrLines
      arr = listArray (0, n - 1) arrLines
      width = length $ head arrLines
      compute i pos
        | i >= n = 1
        | otherwise =
            let line = arr ! i
             in case line ! pos of
                  '^' -> memo ! (i + 1, pos - 1) + memo ! (i + 1, pos + 1)
                  _ -> memo ! (i + 1, pos)
      memo =
        array
          ((0, 0), (n, width - 1))
          [((i, pos), compute i pos) | i <- [0 .. n], pos <- [0 .. width - 1]]
   in memo ! (0, startingPosition)

parse ["-h"] = usage >> exitSuccess
parse [] = usage >> die usageStr
parse fs = concat `fmap` mapM readFile fs

usageStr = "Usage: solve [-vh] [file ..]"

usage = putStrLn usageStr
