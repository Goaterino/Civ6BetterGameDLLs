-- ===========================================================================
-- Civ6RolloverProxy (src/ui_rendements.lua), ajouté par le proxy à la fin de TopPanel.lua.
-- Le bandeau ne redessine ses rendements que sur une liste d'événements où ne figure aucun
-- événement de gouverneur. Le proxy de GameCore refait les instantanés de rendements du joueur à
-- la fin de la passe qui a promu, retiré ou établi un gouverneur, ou activé un personnage illustre
-- (voir empire_yields/README.md) : on redessine sur ces événements, puis une seconde fois à la fin
-- de la série d'événements (GameCoreEventPublishComplete), quand les instantanés sont arrivés.
-- Les arguments de GovernorAssigned diffèrent selon les scripts du jeu : on ne les lit pas.
-- ===========================================================================
do
	local m_pendingYields = false;

	local function RequestYields()
		m_pendingYields = true;
		if OnRefreshYields ~= nil then
			OnRefreshYields();
		end
	end

	local function FlushYields()
		if m_pendingYields then
			m_pendingYields = false;
			if OnRefreshYields ~= nil then
				OnRefreshYields();
			end
		end
	end

	Events.GovernorAssigned.Add(RequestYields);
	Events.GovernorChanged.Add(RequestYields);
	Events.GovernorPromoted.Add(RequestYields);
	Events.UnitGreatPersonActivated.Add(RequestYields);
	Events.GameCoreEventPublishComplete.Add(FlushYields);
end
