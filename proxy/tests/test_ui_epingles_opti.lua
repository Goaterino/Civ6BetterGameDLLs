-- Test hors jeu de src/ui_epingles_opti.lua, dans un environnement qui imite l'interface de Civ6 :
--   lua proxy/tests/test_ui_epingles_opti.lua
-- On charge comme le proxy (`patch_epingles`) : texte du fichier, puis le bloc d'optimisation, puis
-- éventuellement le bloc de mesure, chacun précédé de son contexte.

local dir = arg[0]:match("^(.*)[/\\]") or ".";
local function ReadFile(path)
	local f = assert(io.open(path, "rb"));
	local s = f:read("a");
	f:close();
	return s;
end
local OPTI = ReadFile(dir .. "/../src/ui_epingles_opti.lua");
local MESURE = ReadFile(dir .. "/../src/ui_epingles.lua");

local printed = {};

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

-- Épingles : pins[playerID][pinID] = { x, y, name, icon, visible }.
local pins = { [0] = {}, [1] = {} };
for i = 0, 4 do pins[0][i] = { x = i, y = 1, name = "a" .. i, icon = "ICON_MAP_PIN_SQUARE", visible = true }; end
for i = 0, 2 do pins[1][i] = { x = i, y = 2, name = "b" .. i, icon = "ICON_MAP_PIN_CIRCLE", visible = true }; end

local getMapPins = 0;
local pinMethods = {
	GetHexX = function(self) return self.d.x; end,
	GetHexY = function(self) return self.d.y; end,
	GetName = function(self) return self.d.name; end,
	GetIconName = function(self) return self.d.icon; end,
	IsVisible = function(self) return self.d.visible; end,
};
local cfgMethods = {
	GetMapPins = function(self)
		getMapPins = getMapPins + 1;
		local t = {};
		for id, d in pairs(pins[self.id]) do t[id] = setmetatable({ d = d }, { __index = pinMethods }); end
		return t;
	end,
	GetValue = function() return nil; end,
};
local PlayerConfigurations = {};
for id = 0, 1 do PlayerConfigurations[id] = setmetatable({ id = id }, { __index = cfgMethods }); end
local function Player(id) return { GetID = function() return id; end }; end
local Game = {
	GetLocalPlayer = function() return 0; end,
	GetPlayers = function() return { Player(0), Player(1) }; end,
};

local function NewEnv(events, contextPtr)
	local env = setmetatable({}, { __index = _G });
	env.print = function(s) printed[#printed + 1] = s; end
	env.Events, env.PlayerConfigurations, env.Game, env.ContextPtr = events, PlayerConfigurations, Game, contextPtr;
	env.Players = { [0] = Player(0), [1] = Player(1) };
	env.GameDefines = { MAX_MAJOR_CIVS = 2 };
	return env;
end

local function Load(env, text, ctx, mesure)
	local chunk = text .. "\nlocal EpinglesOpti_Ctx = \"" .. ctx .. "\";\n" .. OPTI;
	if mesure then
		chunk = chunk .. "\nlocal MesureEpingles_Ctx = \"" .. ctx .. "\";\n" .. MESURE;
	end
	assert(load(chunk, "=" .. ctx, "t", env))();
end

-- MapPinManager simplifié : comme le vrai, chaque drapeau relit GetMapPins() plusieurs fois.
local MANAGER = [[
MapPinFlag = {}
MapPinFlag.__index = MapPinFlag
function MapPinFlag.GetMapPin(self)
	return PlayerConfigurations[self.m_Player:GetID()]:GetMapPins()[self.m_pinID]
end
function MapPinFlag.Refresh(self)
	for _ = 1, 7 do assert(self:GetMapPin() ~= nil) end
end
refreshes = 0
function Refresh()
	refreshes = refreshes + 1
	for _, p in ipairs(Game.GetPlayers{ Alive = true, Human = true }) do
		for pinID in pairs(PlayerConfigurations[p:GetID()]:GetMapPins()) do
			local flag = setmetatable({ m_Player = Players[p:GetID()], m_pinID = pinID }, MapPinFlag)
			flag:Refresh()
		end
	end
	if failNext then failNext = false; error("Refresh en échec") end
end
function OnPlayerInfoChanged(playerID) Refresh() end
Events.PlayerInfoChanged.Add(OnPlayerInfoChanged)
]];

local function check(cond, msg) assert(cond, msg); end

do
	local events = { PlayerInfoChanged = NewEvent() };
	local mpm = NewEnv(events, {});
	Load(mpm, MANAGER, "mappinmanager.lua");
	check(#events.PlayerInfoChanged.handlers == 1, "un seul gestionnaire");

	getMapPins = 0;
	events.PlayerInfoChanged.Fire(1);
	check(mpm.refreshes == 1, "première diffusion : reconstruction");
	-- Sans cache : 2 (Refresh) + 8 drapeaux x 7. Avec : 2 (signature) + 2 (Refresh) + 2 (cache).
	check(getMapPins <= 6, "GetMapPins lu une fois par joueur, vu " .. getMapPins);

	events.PlayerInfoChanged.Fire(1);
	check(mpm.refreshes == 1, "rien n'a changé : pas de reconstruction");

	pins[1][1].name = "renommée";
	events.PlayerInfoChanged.Fire(1);
	check(mpm.refreshes == 2, "nom changé : reconstruction");
	pins[1][3] = { x = 9, y = 9, name = "nouvelle", icon = "ICON_MAP_PIN_CIRCLE", visible = true };
	events.PlayerInfoChanged.Fire(1);
	check(mpm.refreshes == 3, "épingle ajoutée : reconstruction");
	pins[0][2].visible = false;
	events.PlayerInfoChanged.Fire(0);
	check(mpm.refreshes == 4, "visibilité changée : reconstruction");

	-- Hors reconstruction, GetMapPin lit la configuration comme avant.
	getMapPins = 0;
	local flag = setmetatable({ m_Player = mpm.Players[0], m_pinID = 1 }, mpm.MapPinFlag);
	flag:GetMapPin();
	check(getMapPins == 1, "GetMapPin hors reconstruction inchangé");

	-- Une erreur dans Refresh remonte, et le cache ne reste pas actif.
	mpm.failNext = true;
	pins[0][4].name = "x";
	local ok = pcall(events.PlayerInfoChanged.handlers[1], 0);
	check(not ok, "erreur propagée");
	check(mpm.EpinglesOpti_Etat.cache == nil, "cache vidé après une erreur");
end

-- Fichier de base inclus par un mod, puis le mod : le bloc s'exécute deux fois sans doublon.
do
	local events = { PlayerInfoChanged = NewEvent() };
	local mpm = NewEnv(events, {});
	Load(mpm, MANAGER, "mappinmanager.lua");
	Load(mpm, "local BASE = MapPinFlag.Refresh\nfunction MapPinFlag.Refresh(self) BASE(self) end", "mappinmanager_dmt.lua");
	check(#events.PlayerInfoChanged.handlers == 1, "pas de gestionnaire en double");
	pins[0][0].name = "z";
	events.PlayerInfoChanged.Fire(0);
	events.PlayerInfoChanged.Fire(0);
	check(mpm.refreshes == 1, "une reconstruction, puis rien");
end

-- Liste : reconstruite seulement à l'ouverture si elle est fermée.
do
	local events = { PlayerInfoChanged = NewEvent() };
	local panelHidden = true;
	local update = nil;
	local ctxPtr = {
		LookUpControl = function(_, path)
			assert(path == "/InGame/MinimapPanel/MapPinListPanel");
			return { IsHidden = function() return panelHidden; end };
		end,
		SetUpdate = function(_, f) update = f; end,
		ClearUpdate = function() update = nil; end,
	};
	local list = NewEnv(events, ctxPtr);
	Load(list, [[
builds = 0
function BuildMapPinList() builds = builds + 1 end
function OnPlayerInfoChanged(playerID) BuildMapPinList() end
Events.PlayerInfoChanged.Add(OnPlayerInfoChanged)
BuildMapPinList()
]], "mappinlistpanel.lua", true);
	check(list.builds == 1, "construction initiale");

	pins[1][0].name = "fermée";
	events.PlayerInfoChanged.Fire(1);
	check(list.builds == 1, "liste fermée : rien");
	check(update ~= nil, "reconstruction en attente");
	update();
	check(list.builds == 1 and update ~= nil, "toujours fermée : on attend");
	panelHidden = false;
	update();
	check(list.builds == 2 and update == nil, "ouverture : reconstruction, puis plus d'attente");

	events.PlayerInfoChanged.Fire(1);
	check(list.builds == 2, "ouverte, rien n'a changé : rien");
	pins[1][0].name = "ouverte";
	events.PlayerInfoChanged.Fire(1);
	check(list.builds == 3, "ouverte, nom changé : reconstruction");

	-- Fermée et modifiée, puis rouverte par une diffusion avant la mise à jour de l'image.
	panelHidden = true;
	pins[1][0].name = "encore";
	events.PlayerInfoChanged.Fire(1);
	panelHidden = false;
	events.PlayerInfoChanged.Fire(1);
	check(list.builds == 4, "reconstruite par la diffusion");
	update();
	check(list.builds == 4 and update == nil, "pas de seconde reconstruction");
end

local lines = table.concat(printed, "\n");
print(lines);
assert(lines:find("EpinglesOpti\tmappinmanager.lua\tactif", 1, true));
assert(lines:find("EpinglesOpti\tmappinlistpanel.lua\tactif", 1, true));
assert(lines:find("mappinlistpanel.lua\tPlayerInfoChanged\targ=1", 1, true), "mesure de la liste allégée");
assert(not lines:find("echec", 1, true));
print("OK");
