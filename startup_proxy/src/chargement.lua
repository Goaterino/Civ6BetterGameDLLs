-- ===========================================================================
-- BetterGameDLLs : correctifs POL des parties locales, ajouté à la fin de LoadScreen.lua par le
-- proxy de démarrage (startup_proxy/src/salon.rs). Fonctions natives posées par le proxy :
--   Civ6Proxy_FixerPol(actif, raison)  -> état lu par le proxy de GameCore (1 ou 0)
--   Civ6Proxy_Journal(texte)           -> écrit dans RolloverProxy.log
-- Une partie qui n'est pas en réseau (solo, hotseat) ne tourne que sur cette machine : rien à
-- désynchroniser, les correctifs POL y sont actifs. En réseau, c'est le salon qui a décidé
-- (salon.lua) : on n'y touche pas.
-- ===========================================================================

if Civ6Proxy_FixerPol ~= nil then
	Events.LoadScreenContentReady.Add(function()
		local ok, err = pcall(function()
			if not GameConfiguration.IsNetworkMultiplayer() then
				Civ6Proxy_FixerPol(1, GameConfiguration.IsHotseat() and "hotseat" or "partie solo");
			end
		end);
		if not ok and Civ6Proxy_Journal ~= nil then
			Civ6Proxy_Journal("chargement : erreur : " .. tostring(err));
		end
	end);
end
