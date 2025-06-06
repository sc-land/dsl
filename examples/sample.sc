bug Cat
  gene name String
  gene age Int
  gene color String

  ethics meow
    return "Meow!"
  end

  ethics eat(food: String)
    if food.eq("fish")
      return "Yum!"
    elsif food.eq("vegetable")
      return "Meh..."
    else
      return "Not interested"
    end
  end

  ethics celebrate(times: Int) String
    for i in times
      meow()
    end
    return "Celebration complete!"
  end
end

cat = Cat.happens
cat.setName("Whiskers")
cat.setAge(3)
cat.setColor("Orange")

response = cat.eat("fish")
cat.celebrate(5)

if cat.age.bt(2)
  adult_cat = true
else
  adult_cat = false
end

counter = 0
while counter.lt(10)
  counter = counter.add(1)
end

bug Calculator
  ethics add(a: Int, b: Int) Int
    return a.add(b)
  end

  ethics multiply(a: Int, b: Int) Int
    result = a.mult(b)
    return result
  end
end

calc = Calculator()
sum = calc.add(5, 3)
product = calc.multiply(4, 7)
