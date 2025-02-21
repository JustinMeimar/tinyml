val f : int->bool = fn x=>x>0
val x : int = 42
val b : bool = true
val id : 'a -> 'a = fn x=>x

val increment = fn x => x + 1

val five = increment 4

let 
  val y = 10
in
  y * 2
end

  val maybeValue = some 42

val extractedValue = 
  case maybeValue of
    none => 0
  | some x => x

val myList = 1 :: 2 :: 3 :: nil

val headOrDefault = 
  case myList of
    nil => 0
  | x :: xs => x

val nestedOption = some (some 10)

val deepValue =
  case nestedOption of
    none => 0
  | some none => 5
  | some (some v) => v

