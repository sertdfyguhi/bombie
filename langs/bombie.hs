bombie :: String -> String
bombie str = str ++ "."

main :: IO ()
main = putStrLn (bombie "bombie")