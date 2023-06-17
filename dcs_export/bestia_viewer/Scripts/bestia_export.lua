log.write('Bestia', log.INFO, 'Bestia Starte')

Bestia = {
	alive=false,
	world_screenshot_requested = true
}

function Bestia.SplitString(text, separator)
	local result = {}
	for line in string.gmatch(text, "([^" .. separator .. "]+)") do
		table.insert(result, line)
	end

	return result
end

function Bestia.LeftPad(n)
	local result = ""
	for i=1, n do
		result = result .. "\t"
	end
	return result
end

function Bestia.TableSerialize(tab, max_depth, indent)
	indent = indent or 0
	max_depth = max_depth or 10
	if max_depth == 0 then
		return "MAX_DEPTH"
	end

    if type(tab) == "table" then
		local result = "\n" .. Bestia.LeftPad(indent) .. "{\n"
        for k, v in pairs(tab) do
            result =  result .. Bestia.LeftPad(indent + 1) .. k .. " = " .. Bestia.TableSerialize(v, max_depth - 1, indent + 1) .. ",\n"
        end
        return result .. Bestia.LeftPad(indent) .. "}"
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

function Bestia.UnitsTableToCSV(tab)
	local header = "id, GroupName, UnitName, UnitType, Coalition, CoalitionID, Country, PositionX, PositionY, PositionZ, Longitude, Latitude, Altitude, Heading, Pitch, Bank, IsHuman, IsInvisible, IsRadarActive, IsJamming, IsIRJamming, IsBorn, IsStatic, IsAI_ON"
	local separator = ", "
	local result = header

	for k, v in pairs(tab) do
		result = result .. "\n"
		result = result .. k .. separator --id 
		result = result .. "\"" .. v.GroupName .. "\"" .. separator
		result = result .. "\"" .. v.UnitName .. "\"" .. separator
		result = result .. "\"" .. v.Name .. "\"" .. separator --UnitType
		result = result .. "\"" .. v.Coalition .. "\"" .. separator
		result = result .. v.CoalitionID .. separator
		result = result .. v.Country .. separator
		result = result .. v.Position.x .. separator
		result = result .. v.Position.y .. separator
		result = result .. v.Position.z .. separator
		result = result .. v.LatLongAlt.Long .. separator
		result = result .. v.LatLongAlt.Lat .. separator
		result = result .. v.LatLongAlt.Alt .. separator
		result = result .. v.Heading .. separator
		result = result .. v.Pitch .. separator
		result = result .. v.Bank .. separator
		result = result .. (v.Flags.Human and "true" or "false") .. separator
		result = result .. (v.Flags.Invisible and "true" or "false") .. separator
		result = result .. (v.Flags.RadarActive and "true" or "false") .. separator
		result = result .. (v.Flags.Jamming and "true" or "false") .. separator
		result = result .. (v.Flags.IRJamming and "true" or "false") .. separator
		result = result .. (v.Flags.Born and "true" or "false") .. separator
		result = result .. (v.Flags.Static and "true" or "false") .. separator
		result = result .. (v.Flags.AI_ON and "true" or "false")
	end

	return result
end

function Bestia.TakeWorldScreenshot()
	log.write('Bestia', log.INFO, "World Screenshot")
	return LoGetWorldObjects("units")
end

function Bestia.StartApi()
	Bestia.server = socket.tcp()
	Bestia.server:bind('*', 2137)
	Bestia.server:listen()
	Bestia.server:settimeout(0)
end

function Bestia.GetResponse(request)
	if request.method == "GET" and request.path == "/ping" then
		return "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\npong"
	elseif request.method == "GET" and request.path == "/units" then
		return "HTTP/1.1 200 OK\r\nContent-Type: text/csv\r\n\r\n" .. Bestia.UnitsTableToCSV(Bestia.TakeWorldScreenshot())
	end

	return nil
end

function Bestia.DeserializeRequest(request_text)
	local data_splitted = Bestia.SplitString(request_text, " ")
	local path_parameter_splitted = Bestia.SplitString(data_splitted[2], "?")

	local request = {
		method = data_splitted[1],
		path = path_parameter_splitted[1],
		parameter_text = path_parameter_splitted[2],
		http_protocol = data_splitted[3]
	}
	log.write('Bestia', log.INFO, 'Request formated: ' .. Bestia.TableSerialize(request))
	return request
end

function LuaExportStart()
	local status, err = pcall(
		function () 
			log.write('Bestia', log.INFO, 'Export Start')
			Bestia.alive = true
			Bestia.StartApi()
		end
	)
	if err then
		log.write('Bestia', log.ERROR, err)
	end
end

function LuaExportBeforeNextFrame()

end

function LuaExportAfterNextFrame()
	local status, err = pcall(
		function () 
			if Bestia.alive then
				local client = Bestia.server:accept()
				if client then
					local request_text = client:receive()
					log.write('Bestia', log.INFO, "Got request: " .. request_text)
					local request = Bestia.DeserializeRequest(request_text)
					local response = Bestia.GetResponse(request)
					if response then
						client:send(response)
					end
				end
			end
		end
	)
	if err then
		log.write('Bestia', log.ERROR, err)
	end
end

function LuaExportStop ()
	log.write('Bestia', log.INFO, 'Export Stop')
	Bestia.alive = false
	Bestia.server:close()
end


log.write('Bestia', log.INFO, 'Bestia Ende')