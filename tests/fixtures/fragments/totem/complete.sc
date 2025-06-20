totem Stage
  egg
  larvie
  bee
end

totem Status
  alive(Stage)
  dead
end

totem Geo
    circle(r: Int)
    retangle(x: Int, y: Int)
    triangle(a: Int, b: Int, c: Int)
end

totem Severity
  low
  high
end

totem Alert
  ping(Str)
  alarm(Str, Severity)
end

totem SystemEvent
  idle
  error(Alert)
end

totem Lang
  rust
  sc
  other(Str)
end

totem File
  source(Str, Lang)
  binary(Str)
  none
end