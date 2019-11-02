function split(s, c)
  if string.find(s, c) == nil then
    return { s }
  end

  local cs = {}
  
  local lastPos = 0
  for part, p in string.gmatch(s, "(.-)" .. c .. "()") do
    table.insert(cs, part)
    lastPos = p
  end

  table.insert(cs, string.sub(s, lastPos))

  return cs
end
