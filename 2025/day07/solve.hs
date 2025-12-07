import Basement.Compat.IsList qualified as Set
import Data.Array
import Data.List
import Data.Maybe
import Data.Set
import System.Environment
import System.Exit

main = getArgs >>= parse >>= (print . solve . lines)

findStartingPoint line = fromJust (elemIndex 'S' line)

solve lines = simulate (tail lines) 0 (Data.Set.singleton (findStartingPoint (head lines)))

stringToArray :: [Char] -> Array Int Char
stringToArray s = listArray (0, length s - 1) s

checkHit :: Array Int Char -> Int -> [Int]
checkHit line idx = case line ! idx of
  '^' -> (idx - 1) : [idx + 1]
  _ -> [idx]

simulate :: [[Char]] -> Int -> Set Int -> Int
simulate [] count active = count
simulate lines count active =
  let arr = stringToArray (head lines)
      followUp = Data.Set.fromList $ concatMap (checkHit arr) (Set.toList active)
      hits = length (Data.Set.filter (\i -> arr ! i == '^') active)
      remainder = tail lines
   in simulate remainder (hits + count) followUp

parse ["-h"] = usage >> exitSuccess
parse [] = usage >> die usageStr
parse fs = concat `fmap` mapM readFile fs

usageStr = "Usage: solve [-vh] [file ..]"

usage = putStrLn usageStr
