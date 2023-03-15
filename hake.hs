{-# LANGUAGE UnicodeSyntax #-}

import           Hake

main ∷ IO ()
main = hake $ do

  "clean | clean the project" ∫
    cargo ["clean"] >> removeDirIfExists targetPath

  "update | update dependencies" ∫ cargo ["update"]

  kathoeyExecutable ♯
      cargo <| "build" : buildFlags

  "test | build and test" ◉ [kathoeyExecutable] ∰ do
    cargo ["test"]
    cargo ["clippy"]

 where
  targetPath ∷ FilePath
  targetPath = "target"

  buildPath ∷ FilePath
  buildPath = targetPath </> "release"

  buildFlags ∷ [String]
  buildFlags = [ "--release" ]

  kathoeyExecutable ∷ FilePath
  kathoeyExecutable = buildPath </> "libkathoey.rlib"
