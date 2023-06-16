log.write('Bestia', log.INFO, 'Bestia Starte')

Bestia = {}

function Bestia.table_serialize(tab, max_depth)
	max_depth = max_depth or 10
	if max_depth == 0 then
		return "MAX_DEPTH"
	end

    if type(tab) == "table" then
		local result = "{"
        for k, v in pairs(tab) do
            result =  result .. k .. " = " .. Bestia.table_serialize(v, max_depth - 1) .. ","
        end
        return result .. "}"
    elseif type(tab) == "number" then
        return tostring(tab)
    elseif type(tab) == "string" then
        return tab
    elseif type(tab) == "boolean" then
        return tab and "true" or "false"
    else
        return "UNKNOWN_TYPE: " .. type(tab)
    end
end

function Bestia.LuaExportStart()
	log.write('Bestia', log.INFO, 'Export Start')
end

function Bestia.LuaExportBeforeNextFrame()
	log.write('Bestia', log.INFO, "Every unit: " .. Bestia.table_serialize(LoGetWorldObjects("units")))
end

function Bestia.LuaExportAfterNextFrame()

end

function Bestia.LuaExportStop ()
	log.write('Bestia', log.INFO, 'Export Stop')
end


log.write('Bestia', log.ERROR, 'Bestia Ende')