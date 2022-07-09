#!/usr/bin/env lua 
local argparse = require("argparse")
local lfs = require("lfs")
local json = require("lunajson")

local parser = argparse("mktpl", "A file template maker.")
parser:mutex(
    parser:flag("-l --list", "List all the availiable templates"),
    parser:option("-t --template", "The name of the template to be created")
)

local args = parser:parse()


function read_file (dir) 
    local f = assert(io.open(dir.. '/' .. "templates.json", "rb"))
    local contents = f:read("*all")
    f:close()
    return contents
end


function list_templates ()
    print("Templates: ")
    local template_dir = os.getenv("HOME") .. "/.templates"
    
    -- Iterate through files in the template dir ignoring self, back and the templates.json file and print the file name and help message if its in templates.json
    for file in lfs.dir(template_dir) do 
        if file ~= "." and file ~= ".." and file ~= "templates.json" then
            local f = template_dir .. '/' .. file
            local attributes = lfs.attributes(f)
            -- Ignore directories
            if attributes.mode ~= "directory" then
                local info = json.decode(read_file(template_dir))["templates"]
                if info ~= nil then
                    if info[file] ~= nil then
                        print("  "..file, info[file]["help"])
                    else 
                        print("  "..file)
                    end
                else 
                    print("Reading template json, is is formatted correctly?")
                end
            end
        end 
    end
end

function make_template (template_name)
    local cwd = lfs.currentdir()
    local template_full_path = "~/.templates/" .. template_name
    local current_full_path = cwd .. "/" .. template_name 
    if os.execute(string.format("cp %s %s", template_full_path, current_full_path)) then
        print("Successfully create template with name " .. template_name)
    else
        print("Error creating template with name " .. template_name)
    end
end

for key, value in pairs(args) do 

    if key == "list" then
        list_templates() 
    elseif key == "template" then
        make_template(value)
    end
end

