let val id = fn x => x in
  let val compose = fn f => fn g => fn x => f (g x)
  in
    case CONS(1, CONS(2, NIL)) of
      NIL => false
    | CONS(head, tail) => 
        let val result = 
          if head > 0 
          then SOME(head * 2)
          else NONE
        in
          true
        end
  end
end

