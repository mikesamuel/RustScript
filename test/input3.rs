let text = input("Enter number: ")
let val = parseVal(text)
println(typeof(val))
if has(val) then println(val * 2) else println("Could not parse input number!")