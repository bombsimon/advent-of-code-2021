import           System.IO

main :: IO ()
main = do
  contents <- readFile "../../input/day01"

  print $ "Part 1: " ++ show (part1 contents)
  print $ "Part 2: " ++ show (part2 contents)

part1 :: [Char] -> Int
part1 = increases . map readInt . lines

part2 :: [Char] -> Int
part2 = increases . sum3 . map readInt . lines

sum3 :: [Int] -> [Int]
sum3 xs | length xs < 3 = []
        | otherwise     = n : sum3 (tail xs)
  where n = head xs + (xs !! 1) + (xs !! 2)

increases :: [Int] -> Int
increases =
  snd . foldl (\(p, s) v -> if v > p then (v, s + 1) else (v, s)) (maxBound, 0)

readInt :: String -> Int
readInt = read
