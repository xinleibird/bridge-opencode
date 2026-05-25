pub fn refresh_buffer_lua(buf_number: i64) -> String {
    format!(
        r#"
        local buf = {}
        local cursor_positions = {{}}
        local is_current_buf = vim.api.nvim_get_current_buf() == buf

        for _, win in ipairs(vim.api.nvim_list_wins()) do
            if vim.api.nvim_win_get_buf(win) == buf then
                cursor_positions[win] = vim.api.nvim_win_get_cursor(win)
            end
        end

        vim.api.nvim_buf_call(buf, function()
            vim.cmd('checktime')
            vim.cmd('edit')
        end)

        for win, pos in pairs(cursor_positions) do
            if vim.api.nvim_win_is_valid(win) then
                pcall(vim.api.nvim_win_set_cursor, win, pos)
            end
        end

        if is_current_buf then
            vim.cmd('redraw')
        end
        "#,
        buf_number
    )
}

pub fn send_notification_lua(message: &str) -> String {
    let escaped = message
        .replace('\\', "\\\\")
        .replace('"', r#"\""#)
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!(r#"vim.notify("{}", vim.log.levels.ERROR)"#, escaped)
}

pub fn get_visual_selection_lua() -> &'static str {
    r#"
    local mode = vim.fn.mode()
    if not mode:match('[vV\22]') then
        return nil
    end

    local start_pos = vim.fn.getpos("v")
    local end_pos = vim.fn.getpos(".")
    local sel_type = mode:sub(1, 1)

    if start_pos[2] == 0 or end_pos[2] == 0 then
        return nil
    end

    local file_path = vim.api.nvim_buf_get_name(0)
    if file_path == "" then
        return nil
    end

    local lines = vim.fn.getregion(start_pos, end_pos, { type = sel_type })
    local content = table.concat(lines, "\n")

    local start_line = math.min(start_pos[2], end_pos[2])
    local end_line = math.max(start_pos[2], end_pos[2])

    return vim.fn.json_encode({
        file_path = file_path,
        start_line = start_line,
        end_line = end_line,
        content = content,
        cwd = vim.fn.getcwd()
    })
    "#
}
