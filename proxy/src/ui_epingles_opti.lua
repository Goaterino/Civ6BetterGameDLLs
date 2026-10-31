-- ===========================================================================
-- Civ6RolloverProxy (src/ui_epingles_opti.lua) : moins de travail à chaque PlayerInfoChanged (voir
-- map_pins/README.md). Avec `ui_epingles=on`, le proxy l'ajoute à la fin de MapPinManager.lua (et de
-- ses versions DMT ou CQUI) et de MapPinListPanel.lua, précédé de `local EpinglesOpti_Ctx = "<fichier>"`.
-- Chaque PlayerInfoChanged reconstruisait tout, pour n'importe quel joueur et n'importe quelle valeur
-- changée (votes et pause de MPH, renvois identiques) :
-- - les drapeaux et la liste ne sont reconstruits que si une épingle a changé : position, nom,
--   icône ou visibilité pour le joueur local, ou l'ensemble des joueurs ;
-- - pendant une reconstruction des drapeaux, MapPinFlag.GetMapPin lit une seule fois la table
--   GetMapPins() de chaque joueur, au lieu d'environ 7 fois par drapeau ;
-- - la liste, fermée, n'est reconstruite qu'à son ouverture.
-- Le bloc peut s'exécuter plusieurs fois dans un même contexte (fichier de base inclus par un mod,
-- puis le mod) : une fonction déjà enveloppée ne l'est pas deux fois. Tout s'exécute sous pcall ; un
-- échec écrit `EpinglesOpti <fichier> echec <erreur>` et laisse le script hôte intact.
-- ===========================================================================
do
local EpinglesOpti_Ok, EpinglesOpti_Err = pcall(function()
	local CTX = EpinglesOpti_Ctx;
	local S = EpinglesOpti_Etat;
	if S == nil then
		S = { wrapped = {} };
		EpinglesOpti_Etat = S;
	end
	local unpack = unpack or table.unpack;

	local function Pack(...)
		return select("#", ...), { ... };
	end

	-- Ce que les drapeaux et la liste affichent des épingles des joueurs `ids`, en une chaîne.
	local function Signature(ids)
		local localID = Game.GetLocalPlayer();
		local parts = {};
		for _, id in ipairs(ids) do
			local cfg = PlayerConfigurations[id];
			if cfg ~= nil then
				parts[#parts + 1] = "#" .. id;
				for pinID, pin in pairs(cfg:GetMapPins()) do
					parts[#parts + 1] = table.concat({ id, pinID, pin:GetHexX(), pin:GetHexY(), tostring(pin:GetName()),
						tostring(pin:GetIconName()), tostring(pin:IsVisible(localID)) }, "\1");
				end
			end
		end
		table.sort(parts);
		return localID .. "\2" .. table.concat(parts, "\2");
	end

	-- Joueurs dont MapPinManager dessine les épingles (Refresh), et ceux de la liste (BuildMapPinList).
	local function HumanIDs()
		local ids = {};
		for _, p in ipairs(Game.GetPlayers{ Alive = true, Human = true }) do
			ids[#ids + 1] = p:GetID();
		end
		return ids;
	end
	local function MajorIDs()
		local ids = {};
		for id = 0, GameDefines.MAX_MAJOR_CIVS - 1 do
			ids[#ids + 1] = id;
		end
		return ids;
	end

	-- Appelle f(...) entre before() et after(), même si f lève une erreur (qui est relancée).
	local function Around(f, before, after)
		local w = function(...)
			before();
			local n, r = Pack(pcall(f, ...));
			after();
			if not r[1] then
				error(r[2], 0);
			end
			return unpack(r, 2, n);
		end
		S.wrapped[w] = true;
		return w;
	end

	-- Remplace le gestionnaire inscrit à Events.PlayerInfoChanged par w(base).
	local function Rewire(f, make)
		if type(f) ~= "function" or S.wrapped[f] then
			return f;
		end
		local w = make(f);
		S.wrapped[w] = true;
		Events.PlayerInfoChanged.Remove(f);
		Events.PlayerInfoChanged.Add(w);
		return w;
	end

	if CTX == "mappinmanager.lua" or CTX == "mappinmanager_dmt.lua" or CTX == "mappinmanager_cqui.lua" then
		if MapPinFlag ~= nil and type(MapPinFlag.GetMapPin) == "function" and not S.wrapped[MapPinFlag.GetMapPin] then
			local base = MapPinFlag.GetMapPin;
			local w = function(self, ...)
				local cache = S.cache;
				if cache == nil or self.m_Player == nil or self.m_pinID == nil then
					return base(self, ...);
				end
				local id = self.m_Player:GetID();
				local pins = cache[id];
				if pins == nil then
					pins = PlayerConfigurations[id]:GetMapPins();
					cache[id] = pins;
				end
				return pins[self.m_pinID];
			end
			S.wrapped[w] = true;
			MapPinFlag.GetMapPin = w;
		end
		if type(Refresh) == "function" and not S.wrapped[Refresh] then
			Refresh = Around(Refresh, function()
				S.drawn = S.pending or Signature(HumanIDs());
				S.pending = nil;
				S.cache = {};
			end, function()
				S.cache = nil;
			end);
		end
		OnPlayerInfoChanged = Rewire(OnPlayerInfoChanged, function(base)
			return function(playerID, ...)
				local sig = Signature(HumanIDs());
				if sig == S.drawn then
					return;
				end
				S.pending = sig;
				return base(playerID, ...);
			end
		end);
	elseif CTX == "mappinlistpanel.lua" then
		local panel = ContextPtr:LookUpControl("/InGame/MinimapPanel/MapPinListPanel");
		if type(BuildMapPinList) == "function" and not S.wrapped[BuildMapPinList] then
			BuildMapPinList = Around(BuildMapPinList, function()
				S.drawn = S.pending or Signature(MajorIDs());
				S.pending = nil;
				S.dirty = false;
			end, function() end);
		end
		-- Panneau fermé : on note qu'il faut reconstruire, et on le fait à l'ouverture.
		local function OnUpdate()
			if panel ~= nil and not panel:IsHidden() then
				ContextPtr:ClearUpdate();
				if S.dirty then
					BuildMapPinList();
				end
			end
		end
		OnPlayerInfoChanged = Rewire(OnPlayerInfoChanged, function(base)
			return function(playerID, ...)
				local sig = Signature(MajorIDs());
				if sig == S.drawn and not S.dirty then
					return;
				end
				if panel ~= nil and panel:IsHidden() then
					if not S.dirty then
						S.dirty = true;
						ContextPtr:SetUpdate(OnUpdate);
					end
					return;
				end
				S.pending = sig;
				return base(playerID, ...);
			end
		end);
	end
	print("EpinglesOpti\t" .. CTX .. "\tactif");
end);
if not EpinglesOpti_Ok then
	print("EpinglesOpti\t" .. tostring(EpinglesOpti_Ctx) .. "\techec\t" .. tostring(EpinglesOpti_Err));
end
end
