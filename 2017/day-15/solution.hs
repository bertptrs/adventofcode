import Data.Bits

genA n = n : genA ((n * 16807) `rem` 2147483647)

genB n = n : genB ((n * 48271) `rem` 2147483647)

judge :: (Integer, Integer) -> Bool
judge (a, b) = (a .&. 0xffff) == (b .&. 0xffff)

divisible :: Integer -> Integer -> Bool
divisible b = (== 0) . (`rem` b)

genA' n = filter (divisible 4) $ genA n
genB' n = filter (divisible 8) $ genB n

main :: IO ()
main = do
    let aStart = 618
        bStart = 814

    print $ length $ filter judge $ take 40000000 $ drop 1 $ zip (genA aStart) (genB bStart)
    print $ length $ filter judge $ take 5000000 $ zip (genA' aStart) (genB' bStart)
