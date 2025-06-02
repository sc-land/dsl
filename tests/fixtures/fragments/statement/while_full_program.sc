bug Counter
  gene value Int
  ethics test_while
    while value.bt(10)
      value = value.add(1)
      Counter.log(value)
    end
  end
  gene counter_limit Int
  ethics log_counter
    Counter.log("Final value: ", value)
  end
