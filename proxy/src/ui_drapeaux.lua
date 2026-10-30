-- ===========================================================================
-- Civ6RolloverProxy (src/ui_drapeaux.lua), inséré par le proxy avant OnUnitFlagClick : clic gauche
-- sur un drapeau d'unité. En visée d'une ville ou d'un quartier, un clic sur le drapeau d'une unité
-- étrangère tire sur sa case, comme un clic sur la case elle-même (WorldInput, CityRangeAttack et
-- DistrictRangeAttack) et comme un clic sur ce drapeau quand c'est une unité qui attaque. Sinon, ou
-- si la cible n'est pas valide, OnUnitFlagClick fait ce qu'il faisait.
-- ===========================================================================
function Civ6RolloverProxy_OnUnitFlagLClick( playerID, unitID )
	local mode = UI.GetInterfaceMode();
	local isCity = (mode == InterfaceModeTypes.CITY_RANGE_ATTACK);
	if playerID ~= Game.GetLocalPlayer() and (isCity or mode == InterfaceModeTypes.DISTRICT_RANGE_ATTACK) then
		local pPlayer = Players[playerID];
		local pUnit = pPlayer and pPlayer:GetUnits():FindID(unitID);
		local pAttacker = isCity and UI.GetHeadSelectedCity() or UI.GetHeadSelectedDistrict();
		if pUnit ~= nil and pAttacker ~= nil then
			local tParameters = {};
			tParameters[UnitOperationTypes.PARAM_X] = pUnit:GetX();
			tParameters[UnitOperationTypes.PARAM_Y] = pUnit:GetY();
			if CityManager.CanStartCommand( pAttacker, CityCommandTypes.RANGE_ATTACK, tParameters ) then
				CityManager.RequestCommand( pAttacker, CityCommandTypes.RANGE_ATTACK, tParameters );
				UI.SetInterfaceMode( InterfaceModeTypes.SELECTION );
				return;
			end
		end
	end
	OnUnitFlagClick( playerID, unitID );
end

