local _ = 1
local _ = "str"
local foo = function(foo: string, bar)
end
local _, bar = "str", @native function(): { number }
end
local _, _ = 1, "str"
local _ = 1 + 1
local _ = "str" .. "str" .. "str"
local _ = foo(_, { 1 })
local _: { number } = bar() :: { number }
local _ = {
    "foo",
    [(function() return 2 end)()] = "bar",
    _ = "qux",
}
