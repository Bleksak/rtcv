M = {}

local uv = vim.uv

M.connect = function(path)
    local socket = uv.new_pipe()
    local err = socket:connect(vim.fn.expand(path))

    if err then
        error(err)
    end

    socket:close()
end

return M
