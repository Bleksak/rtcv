local client = require('rtcv.client')

M = {}

M.setup = function (opts)
    client.connect(opts.socket_path)
end

return M
