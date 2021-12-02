import           System.IO

main :: IO ()
main = do
  contents <- readFile "../../input/day02"

  print $ "Part 1: " ++ show (part1 contents)
  print $ "Part 2: " ++ show (part2 contents)

part1 :: [Char] -> Int
part1 xs = let (x1, x2) = x in x1 * x2
 where
  x =
    foldl
        (\(x, y) (ins, step) -> case ins of
          "forward" -> (x + step, y)
          "down"    -> (x, y + step)
          "up"      -> (x, y - step)
          _         -> (x, y)
        )
        (0, 0)
      $ map instructions
      . lines
      $ xs

part2 :: [Char] -> Int
part2 xs = let (x1, x2, _) = x in x1 * x2
 where
  x =
    foldl
        (\(x, y, aim) (ins, step) -> case ins of
          "forward" -> (x + step, y + (step * aim), aim)
          "down"    -> (x, y, aim + step)
          "up"      -> (x, y, aim - step)
          _         -> (x, y, aim)
        )
        (0, 0, 0)
      $ map instructions
      . lines
      $ xs

instructions :: String -> (String, Int)
instructions line = (head w, readInt $ w !! 1) where w = words line

readInt :: String -> Int
readInt = read
