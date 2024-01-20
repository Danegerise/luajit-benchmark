# Usage

#### Make sure you have installed Rust on your computer first, and then 
```
cargo bench
```
## Code 1
```lua
-- 91.679 ms on my computer

local get = function(self, idx)
    if type(idx) == "number" then
        idx = idx + 1
    end
    return rawget(self, idx)
end

local set = function(self, idx, val)
    if type(idx) == "number" then
        idx = idx + 1
    end
    rawset(self, idx, val)
end

local utils = { get = get, set = set }

local array = {}

for i = 1, 5000, 1 do
    array[i] = math.random(1000000)
end

len = #array;

for i = 0, len - 2, 1 do
    for j = 0, len - 2 - i, 1 do
        a = utils.get(array, j)
        b = utils.get(array, j + 1)
        if a > b then
            utils.set(array, j, b)
            utils.set(array, j + 1, a)
        end
    end
end

```

## Code 2
```lua
-- 100.08 ms on my computer
local get = function(self, idx)
    if type(idx) == "number" then
        idx = idx + 1
    end
    return rawget(self, idx)
end

local set = function(self, idx, val)
    if type(idx) == "number" then
        idx = idx + 1
    end
    rawset(self, idx, val)
end

local utils = { get = get, set = set }

local array = {}

for i = 1, 5000, 1 do
    array[i] = math.random(1000000)
end

len = #array;

for i = 0, len - 2, 1 do
    for j = 0, len - 2 - i, 1 do
        a = get(array, j)
        b = get(array, j + 1)
        if a > b then
            set(array, j, b)
            set(array, j + 1, a)
        end
    end
end

```