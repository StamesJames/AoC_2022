module Main (main) where

import Lib

main :: IO ()
main = do
    calcMaxCal "./res/day_01.csv"

calcMaxCal path = do
    filecontent <- readFile path
    let alllines = lines filecontent
    let elfs = getElfList alllines
    let eome =  map (map putStrLn) elfs
    return ()

getElfList :: [String] -> [[String]]
getElfList [] = []
getElfList l = takeWhile (not . null) l : getElfList (dropWhile (not . null) l)