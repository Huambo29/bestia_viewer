log.write('Bestia', log.INFO, 'Bestia Starte')

Bestia = {}

function Bestia.left_pad(n)
	local result = ""
	for i=1, n do
		result = result .. "\t"
	end
	return result
end

function Bestia.table_serialize(tab, max_depth, indent)
	indent = indent or 0
	max_depth = max_depth or 10
	if max_depth == 0 then
		return "MAX_DEPTH"
	end

    if type(tab) == "table" then
		local result = "\n" .. Bestia.left_pad(indent) .. "{\n"
        for k, v in pairs(tab) do
            result =  result .. Bestia.left_pad(indent + 1) .. k .. " = " .. Bestia.table_serialize(v, max_depth - 1, indent + 1) .. ",\n"
        end
        return result .. Bestia.left_pad(indent) .. "}"
    elseif type(tab) == "number" then
        return tostring(tab)
    elseif type(tab) == "string" then
        return "\"" .. tab .. "\""
    elseif type(tab) == "boolean" then
        return (tab and "true" or "false")
    else
        return "UNKNOWN_TYPE: " .. type(tab)
    end
end

function LuaExportStart()
	log.write('Bestia', log.INFO, 'Export Start')
end

function LuaExportBeforeNextFrame()

	
end

function LuaExportAfterNextFrame()
	local status, err = pcall(
		function () 
			log.write('Bestia', log.INFO, "Kinda works")
			log.write('Bestia', log.INFO, "Every unit: " .. Bestia.table_serialize(LoGetWorldObjects("units"), 5))
		end
	)
	log.write('Bestia', log.ERROR, err)
end

function LuaExportStop ()
	log.write('Bestia', log.INFO, 'Export Stop')
end


log.write('Bestia', log.INFO, 'Bestia Ende')