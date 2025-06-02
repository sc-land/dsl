bug TestBug
  gene x Int
  ethics test_method
      x = 42
  end
  ethics test_method2()
    x = 42
  end
  ethics test_method3(a: Int) Int
    x = 42
    return x
  end
end
