log.write('Bestia', log.INFO, 'Export.lua')
local bestia_lfs=require('lfs')
dofile(bestia_lfs.writedir()..[[Mods\Services\bestia_viewer\Scripts\bestia_export.lua]])