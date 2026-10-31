-- ===========================================================================
-- Civ6RolloverProxy (src/ui_epingles.lua) : mesure du lag des épingles (map_pins/README.md). Avec
-- `ui_mesure_epingles=on`, le proxy l'ajoute à la fin de MapPinManager.lua, MapPinListPanel.lua,
-- DiplomacyRibbon*.lua et des fichiers de DMT, précédé de `local MesureEpingles_Ctx = "<fichier>"`. Chaque appel le plus externe d'une fonction « racine »
-- écrit une ligne `MesureEpingles` dans Lua.log : durée, nombre et durée des fonctions mesurées
-- appelées dedans, nombre d'appels à GetMapPins, épingles par joueur humain, taille du cache DMT.
-- Le bloc peut s'exécuter plusieurs fois dans un même contexte (fichier de base inclus par un mod,
-- puis le mod) : une fonction déjà enveloppée ne l'est pas deux fois. Tout s'exécute sous pcall :
-- un échec écrit `MesureEpingles <fichier> echec <erreur>` et laisse le script hôte intact.
-- ===========================================================================
do
local MesureEpingles_Ok, MesureEpingles_Err = pcall(function()
	local CTX = MesureEpingles_Ctx;
	local M = MesureEpingles_Etat;
	if M == nil then
		M = { wrapped = {}, stats = {}, depth = {}, roots = 0 };
		MesureEpingles_Etat = M;
	end
	-- os.clock de HavokScript avance par secondes entières : horloge du proxy si elle est posée.
	local clock = Civ6RolloverProxy_Clock or os.clock;
	local horloge = Civ6RolloverProxy_Clock and "proxy" or "os.clock";
	local unpack = unpack or table.unpack;  -- HavokScript (5.1) : unpack ; Lua 5.4 des tests : table.unpack.

	local function Pack(...)
		return select("#", ...), { ... };
	end

	local function Stat(label)
		local s = M.stats[label];
		if s == nil then
			s = { n = 0, t = 0, b = 0 };
			M.stats[label] = s;
		end
		return s;
	end

	local function Snapshot()
		local snap = {};
		for k, s in pairs(M.stats) do
			snap[k] = { n = s.n, t = s.t, b = s.b };
		end
		snap.__getmappins = M.shared and M.shared.n or 0;
		return snap;
	end

	-- Épingles par joueur humain et taille de la valeur DMT_MapPinSubjects, en "id:n,id:n".
	local function PinCounts()
		local pins, dmt = {}, {};
		local ok, players = pcall(function() return Game.GetPlayers{ Human = true }; end);
		if not ok or players == nil then
			return "?", "?";
		end
		for _, p in ipairs(players) do
			local id = p:GetID();
			local cfg = PlayerConfigurations[id];
			if cfg ~= nil then
				local n = 0;
				for _ in pairs(cfg:GetMapPins()) do
					n = n + 1;
				end
				pins[#pins + 1] = id .. ":" .. n;
				local v = cfg:GetValue("DMT_MapPinSubjects");
				if type(v) == "string" then
					dmt[#dmt + 1] = id .. ":" .. #v;
				end
			end
		end
		return table.concat(pins, ","), table.concat(dmt, ",");
	end

	local function Report(label, arg, dt, before)
		local parts = {};
		for k, s in pairs(M.stats) do
			local b = before[k] or { n = 0, t = 0, b = 0 };
			local dn = s.n - b.n;
			if dn > 0 and k ~= label then
				local item = k .. "=" .. dn .. "x/" .. string.format("%.1f", (s.t - b.t) * 1000) .. "ms";
				if s.b > b.b then
					item = item .. "/" .. (s.b - b.b) .. "o";
				end
				parts[#parts + 1] = item;
			end
		end
		if M.shared ~= nil then
			local dn = M.shared.n - before.__getmappins;
			if dn > 0 then
				parts[#parts + 1] = "GetMapPins=" .. dn .. "x";
			end
		end
		table.sort(parts);
		local pins, dmt = PinCounts();
		print(string.format("MesureEpingles\t%s\t%.3f\t%s\t%s\targ=%s\tms=%.1f\t%s\tepingles=%s\tdmt_octets=%s",
			os.date("%H:%M:%S"), clock(), CTX, label, tostring(arg), dt * 1000, table.concat(parts, " "), pins, dmt));
	end

	-- Enveloppe f. Une racine écrit sa ligne quand elle est l'appel le plus externe de toutes les
	-- racines. sizeArg : indice d'un argument chaîne dont on cumule la longueur (deserialize).
	local function W(f, label, isRoot, sizeArg)
		if type(f) ~= "function" or M.wrapped[f] then
			return f;
		end
		local function w(...)
			if (M.depth[label] or 0) > 0 then
				return f(...);
			end
			M.depth[label] = 1;
			local before = nil;
			if isRoot then
				if M.roots == 0 then
					before = Snapshot();
				end
				M.roots = M.roots + 1;
			end
			local t0 = clock();
			local nr, r = Pack(pcall(f, ...));
			local dt = clock() - t0;
			M.depth[label] = 0;
			if isRoot then
				M.roots = M.roots - 1;
			end
			local s = Stat(label);
			s.n = s.n + 1;
			s.t = s.t + dt;
			if sizeArg ~= nil then
				local a = select(sizeArg, ...);
				if type(a) == "string" then
					s.b = s.b + #a;
				end
			end
			if before ~= nil then
				local ok, err = pcall(Report, label, (select(1, ...)), dt, before);
				if not ok then
					print("MesureEpingles\t" .. CTX .. "\trapport en echec\t" .. tostring(err));
				end
			end
			if not r[1] then
				error(r[2], 0);
			end
			return unpack(r, 2, nr);
		end
		M.wrapped[w] = true;
		return w;
	end

	-- Gestionnaire déjà inscrit à Events.<event> : on le remplace par son enveloppe.
	local function Rewire(event, f, label)
		if type(f) ~= "function" or M.wrapped[f] then
			return f;
		end
		local w = W(f, label, true);
		Events[event].Remove(f);
		Events[event].Add(w);
		return w;
	end

	-- GetMapPins : compteur partagé, posé sur la table de méthodes des configurations de joueur. Les
	-- tables du moteur peuvent être en lecture seule (Network l'est) : l'écriture est protégée.
	if M.shared == nil then
		local ok, c = pcall(function()
			local mt = getmetatable(PlayerConfigurations[0]);
			local idx = mt and mt.__index;
			if type(idx) ~= "table" or type(idx.GetMapPins) ~= "function" then
				error("pas de table de méthodes");
			end
			if idx.MesureEpingles_Compteur == nil then
				local f = idx.GetMapPins;
				local c = { n = 0 };
				idx.GetMapPins = function(...)
					c.n = c.n + 1;
					return f(...);
				end
				idx.MesureEpingles_Compteur = c;
			end
			return idx.MesureEpingles_Compteur;
		end);
		if ok then
			M.shared = c;
		else
			M.shared = false;
			print("MesureEpingles\t" .. CTX .. "\tGetMapPins non mesurable\t" .. tostring(c));
		end
	end

	if CTX == "mappinmanager.lua" or CTX == "mappinmanager_dmt.lua" or CTX == "mappinmanager_cqui.lua" then
		if MapPinFlag ~= nil then
			MapPinFlag.Refresh = W(MapPinFlag.Refresh, "MapPinFlag.Refresh");
		end
		Refresh = W(Refresh, "Refresh", true);
		deserialize = W(deserialize, "deserialize", false, 1);
		OnPlayerInfoChanged = Rewire("PlayerInfoChanged", OnPlayerInfoChanged, "PlayerInfoChanged");
	elseif CTX == "mappinlistpanel.lua" then
		BuildMapPinList = W(BuildMapPinList, "BuildMapPinList", true);
		OnPlayerInfoChanged = Rewire("PlayerInfoChanged", OnPlayerInfoChanged, "PlayerInfoChanged");
	elseif CTX == "diplomacyribbon.lua" or string.sub(CTX, 1, 16) == "diplomacyribbon_" then
		-- Inscrit plus tard, dans OnInit, sous son nom global : il suffit de remplacer le global.
		UpdateLeaders = W(UpdateLeaders, "UpdateLeaders", true);
	elseif CTX == "dmt_yieldcalculator.lua" then
		UpdatePinYields = W(UpdatePinYields, "UpdatePinYields", true);
		BroadcastMapPinSubjects = W(BroadcastMapPinSubjects, "BroadcastMapPinSubjects");
		deserialize = W(deserialize, "deserialize", false, 1);
	end
	print("MesureEpingles\t" .. CTX .. "\tinstrumente\thorloge=" .. horloge);
end);
if not MesureEpingles_Ok then
	print("MesureEpingles\t" .. tostring(MesureEpingles_Ctx) .. "\techec\t" .. tostring(MesureEpingles_Err));
end
end
