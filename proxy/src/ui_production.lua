-- ===========================================================================
-- Civ6RolloverProxy (src/ui_production.lua), ajouté par le proxy à la fin de ProductionPanel.lua :
-- les variables locales du fichier restent visibles.
-- La production en cours (CurrentProductionContainer) et le message « aucune production »
-- (NoProductionContainer) ne sont montrés ou cachés qu'au changement d'onglet et, file ouverte, à
-- CityProductionQueueChanged. Un changement de ville passe par Refresh puis View, qui remplit la
-- production en cours sans toucher à sa visibilité : après une ville sans production, celle de la
-- ville suivante restait cachée (et inversement, l'ancienne restait affichée). Après View, on
-- applique la règle des onglets : file (OnTabChangeQueue) ou production (OnTabChangeProduction).
-- ===========================================================================
do
	local View_jeu = View;
	function View(data)
		View_jeu(data);
		if m_isQueueOpen then
			Controls.CurrentProductionContainer:SetHide(not m_hasProductionToShow);
			Controls.NoProductionContainer:SetHide(m_hasProductionToShow);
		elseif m_CurrentListMode == LISTMODE.PRODUCTION then
			Controls.CurrentProductionContainer:SetHide(not m_hasProductionToShow);
		end
	end
end
