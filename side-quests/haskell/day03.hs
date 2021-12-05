import           System.IO

main :: IO ()
main = do
  contents <- readFile "../../input/day03"

  print $ "Part 1: " ++ show (part1 contents)
  print $ "Part 2: " ++ show (part2 contents)

part1 :: [Char] -> Int
part1 = mostTimesLeastCommon . mostCommon . map bits . lines

part2 :: [Char] -> Int
part2 _ = 1

mostTimesLeastCommon :: [Int] -> Int
mostTimesLeastCommon xs = binaryToDecimal xs * binaryToDecimal lc
  where lc = map (\v -> if v == 1 then 0 else 1) xs

bits :: String -> [Int]
bits = map readInt

mostCommon :: [[Int]] -> [Int]
mostCommon xs = mostCommonBit (length xs)
  $ foldl incrementList (replicate n 0) xs
  where n = length $ head xs

mostCommonBit :: Int -> [Int] -> [Int]
mostCommonBit n = map (\v -> if v > n `div` 2 then 1 else 0)

incrementList :: [Int] -> [Int] -> [Int]
incrementList xs xxs =
  foldl (\acc (i, _) -> incrementAt i acc xxs) xs $ enumerate xxs

incrementAt :: Int -> [Int] -> [Int] -> [Int]
incrementAt n xs xxs = front ++ [current + v] ++ end
 where
  (front, current : end) = splitAt n xs
  (_    , v : _        ) = splitAt n xxs

readInt :: Char -> Int
readInt = read . pure

binaryToDecimal :: [Int] -> Int
binaryToDecimal = foldr ((\x y -> fromEnum x + 2 * y) . (== 1)) 0

enumerate :: [Int] -> [(Int, Int)]
enumerate x = zip [0 .. length x - 1] x
