local player = {
	extends = Player,
}

function player:_ready()
	print("ready")
end

function player:_process()
	self:move_local_x(1)
end

return player
