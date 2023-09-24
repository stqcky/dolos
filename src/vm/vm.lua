local function read_file(path)
    local file = io.open(path, "rb")

    if file == nil then
        print("could not open file")
        return ""
    end

    local content = file:read("a")

    file:close()

    return content
end


local stack = {}
local sp = 1

local function push(value)
    stack[sp] = value
    sp = sp + 1
end

local function pop()
    sp = sp - 1
    return stack[sp]
end

local ip = 1

local function read_byte(bytecode, idx)
    ip = ip + 1
    return string.byte(bytecode, idx)
end

local function execute(bytecode)
    while true do
        local byte = read_byte(bytecode, ip)

        print(byte)

        if byte == nil then
            return
        end
    end
end

local bytecode = read_file("test.dls")

execute(bytecode)
