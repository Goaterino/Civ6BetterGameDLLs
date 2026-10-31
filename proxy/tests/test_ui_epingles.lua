-- Test hors jeu de src/ui_epingles.lua, dans un environnement qui imite l'interface de Civ6 :
--   lua proxy/tests/test_ui_epingles.lua
-- On charge comme le proxy (`patch_mesure`) : texte du fichier, puis `local MesureEpingles_Ctx = ...`,
-- puis le bloc. Cas couverts : MapPinManager.lua inclus par DMT puis la surcharge de DMT, MapPinListPanel,
-- DiplomacyRibbon (gestionnaire inscrit après coup), tables du moteur en lecture seule (Network l'est
-- dans le jeu), et un bloc qui échoue sans casser le script hôte.

local dir = arg[0]:match("^(.*)[/\\]") or ".";
local f = assert(io.open(dir .. "/../src/ui_epingles.lua", "rb"));
local TAIL = f:read("a");
f:close();

local printed = {};
local function NewEnv()
	local env = setmetatable({}, { __index = _G });
	env.print = function(s) printed[#printed + 1] = s; end
	return env;
end

-- Événements partagés par tous les contextes, comme Events.* du jeu.
local function NewEvent()
	local e = { handlers = {} };
	e.Add = function(h) e.handlers[#e.handlers + 1] = h; end
	e.Remove = function(h)
		for i, x in ipairs(e.handlers) do
			if x == h then table.remove(e.handlers, i); return; end
		end
	end
	e.Fire = function(...) for _, h in ipairs(e.handlers) do h(...); end end
	return e;
end
local Events = { PlayerInfoChanged = NewEvent() };

-- Configurations de joueur : méthodes partagées par une métatable, comme les objets du jeu.
local methods = {};
local pinsOf = { [0] = 3, [1] = 2 };
methods.GetMapPins = function(self)
	local t = {};
	for i = 1, pinsOf[self.id] do t[i] = { id = i }; end
	return t;
end
methods.GetValue = function(self, k)
	if k == "DMT_MapPinSubjects" then return "return {}"; end
end
local PlayerConfigurations = {};
for id = 0, 1 do PlayerConfigurations[id] = setmetatable({ id = id }, { __index = methods }); end
local Game = { GetPlayers = function() return { { GetID = function() return 0; end }, { GetID = function() return 1; end } }; end };
-- Comme dans le jeu : écrire dans Network lève « Attempt to modify read-only table ».
local function ReadOnly(t)
	return setmetatable({}, { __index = t, __newindex = function() error("Attempt to modify read-only table", 2); end });
end
local Network = ReadOnly({ BroadcastPlayerInfo = function() end });

local function Load(env, text, ctx)
	env.Events, env.PlayerConfigurations, env.Game, env.Network = Events, PlayerConfigurations, Game, Network;
	local chunk = text .. "\nlocal MesureEpingles_Ctx = \"" .. ctx .. "\";\n" .. TAIL;
	assert(load(chunk, "=" .. ctx, "t", env))();
end

-- Contexte MapPinManager remplacé par DMT : la base est incluse, puis DMT surcharge et ajoute deserialize.
local mpm = NewEnv();
local refreshes = 0;
Load(mpm, [[
MapPinFlag = {}
function MapPinFlag.Refresh(self) for _, p in pairs(PlayerConfigurations[self.p]:GetMapPins()) do end end
function Refresh()
	for p = 0, 1 do for _ = 1, #PlayerConfigurations[p]:GetMapPins() do MapPinFlag.Refresh({ p = p }) end end
end
function OnPlayerInfoChanged(playerID) Refresh() end
Events.PlayerInfoChanged.Add(OnPlayerInfoChanged)
]], "mappinmanager.lua");
Load(mpm, [[
function deserialize(x) return load(x)() end
local BASE = MapPinFlag.Refresh
function MapPinFlag.Refresh(self) BASE(self); deserialize(PlayerConfigurations[self.p]:GetValue("DMT_MapPinSubjects")) end
]], "mappinmanager_dmt.lua");

-- MapPinListPanel : gestionnaire inscrit dans Initialize, avant le bloc.
local list = NewEnv();
Load(list, [[
function BuildMapPinList() for p = 0, 1 do PlayerConfigurations[p]:GetMapPins() end end
function OnPlayerInfoChanged(playerID) BuildMapPinList() end
Events.PlayerInfoChanged.Add(OnPlayerInfoChanged)
]], "mappinlistpanel.lua");

-- DiplomacyRibbon : inscrit plus tard (OnInit), sous son nom global.
local ribbon = NewEnv();
Load(ribbon, [[
function UpdateLeaders() return "ok", 2 end
function OnInit() Events.PlayerInfoChanged.Add(UpdateLeaders) end
]], "diplomacyribbon.lua");
ribbon.OnInit();

assert(#Events.PlayerInfoChanged.handlers == 3, "un gestionnaire par contexte, sans doublon");
local a, b = ribbon.UpdateLeaders();
assert(a == "ok" and b == 2, "valeurs de retour conservées");
printed = {};
Events.PlayerInfoChanged.Fire(1);

local lines = table.concat(printed, "\n");
print(lines);
local function has(p) assert(lines:find(p, 1, true), "attendu : " .. p); end
-- Le gestionnaire est enveloppé par le bloc du fichier de base, inclus en premier : la ligne porte son nom.
has("mappinmanager.lua\tPlayerInfoChanged\targ=1");
has("MapPinFlag.Refresh=5x");
has("Refresh=1x");
has("deserialize=5x");
has("/45o");           -- 5 x #"return {}"
has("GetMapPins=");
has("epingles=0:3,1:2");
has("dmt_octets=0:9,1:9");
has("mappinlistpanel.lua\tPlayerInfoChanged\targ=1");
has("diplomacyribbon.lua\tUpdateLeaders\targ=1");
-- Le gestionnaire de base, inclus puis enveloppé deux fois, ne doit écrire qu'une ligne.
local n = 0;
for _ in lines:gmatch("mappinmanager[^\t]*\tPlayerInfoChanged") do n = n + 1; end
assert(n == 1, "une seule ligne pour MapPinManager, vu " .. n);
-- Une erreur dans une fonction enveloppée remonte telle quelle.
mpm.Refresh = nil;
local ok, err = pcall(Events.PlayerInfoChanged.handlers[1], 0);
assert(not ok and tostring(err):find("Refresh"), "erreur propagée : " .. tostring(err));

-- Table de méthodes en lecture seule : GetMapPins n'est pas compté, le reste fonctionne.
printed = {};
local roMethods = ReadOnly({ GetMapPins = methods.GetMapPins, GetValue = methods.GetValue });
for id = 0, 1 do PlayerConfigurations[id] = setmetatable({ id = id }, { __index = roMethods }); end
local ro = NewEnv();
Load(ro, [[
function BuildMapPinList() end
function OnPlayerInfoChanged(playerID) BuildMapPinList() end
Events.PlayerInfoChanged.Add(OnPlayerInfoChanged)
loaded = true
]], "mappinlistpanel.lua");
assert(ro.loaded);
lines = table.concat(printed, "\n");
has("GetMapPins non mesurable");
has("mappinlistpanel.lua\tinstrumente");

-- Échec du bloc (Events absent) : une ligne « echec », et la suite du script hôte s'est exécutée.
printed = {};
local broken = NewEnv();
broken.Events, broken.PlayerConfigurations, broken.Game, broken.Network = nil, PlayerConfigurations, Game, Network;
assert(load([[
function OnPlayerInfoChanged() end
hostDone = true
local MesureEpingles_Ctx = "mappinlistpanel.lua";
]] .. TAIL, "=broken", "t", broken))();
assert(broken.hostDone);
lines = table.concat(printed, "\n");
has("mappinlistpanel.lua\techec");
print("OK");
