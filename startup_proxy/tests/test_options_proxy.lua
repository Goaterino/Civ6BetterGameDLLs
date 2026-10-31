-- Test hors jeu de src/options_proxy.lua :
--   lua startup_proxy/tests/test_options_proxy.lua [Options.lua]
-- On charge comme le proxy (`patch_options_lua`) : Options.lua du jeu, puis le bloc, dans un
-- environnement qui imite l'interface. Les deux fonctions natives écrivent dans une table au lieu
-- du fichier. On vérifie l'onglet, les listes, l'enregistrement à la confirmation et le message de
-- redémarrage du jeu.

local dir = arg[0]:match("^(.*)[/\\]") or ".";
local function ReadFile(path)
	local f = assert(io.open(path, "rb"));
	local s = f:read("a");
	f:close();
	return s;
end
local BLOCK = ReadFile(dir .. "/../src/options_proxy.lua");
local OPTIONS_PATH = arg[1]
	or [[C:\Program Files (x86)\Steam\steamapps\common\Sid Meier's Civilization VI\Base\Assets\UI\Options.lua]];
-- HavokScript accepte des annotations de type (`local x:number`) que Lua 5.4 refuse : on les retire.
local OPTIONS = ReadFile(OPTIONS_PATH);
for _, t in ipairs({ "table", "number", "string", "boolean", "object" }) do
	OPTIONS = OPTIONS:gsub("%s*:%s*" .. t .. "%f[^%w_(]", "");
end

-- Tout ce que l'écran appelle sans qu'on s'y intéresse : chaque champ et chaque appel rendent le
-- même objet, sauf les index numériques (ipairs s'arrête) et les comparaisons numériques.
local Any;
Any = setmetatable({}, {
	__index = function(_, k) if type(k) == "number" then return nil; end return Any; end,
	__call = function() return Any; end,
	__lt = function() return false; end,
	__le = function() return false; end,
	__mul = function() return 0; end, __div = function() return 0; end,
	__add = function() return 0; end, __sub = function() return 0; end,
	__concat = function() return ""; end,
});

-- Un contrôle : garde sa visibilité, son texte, ses rappels et, pour une liste, ses entrées.
local function NewControl()
	local c = { hidden = false, text = nil, callbacks = {}, entries = {}, disabled = false };
	return setmetatable(c, { __index = function(_, k)
		if k == "SetHide" then return function(self, h) self.hidden = h; end; end
		if k == "IsHidden" then return function(self) return self.hidden; end; end
		if k == "SetText" or k == "LocalizeAndSetText" then return function(self, t) self.text = t; end; end
		if k == "SetDisabled" then return function(self, d) self.disabled = d; end; end
		if k == "RegisterCallback" then return function(self, ev, f) self.callbacks[ev] = f; end; end
		if k == "RegisterSelectionCallback" then return function(self, f) self.selection = f; end; end
		if k == "ClearEntries" then return function(self) self.entries = {}; end; end
		if k == "BuildEntry" then return function(self, _, inst)
			inst.Button = NewControl();
			table.insert(self.entries, inst);
		end; end
		if k == "GetButton" then return function(self)
			self.button = rawget(self, "button") or NewControl();
			return self.button;
		end; end
		if k == "IsSelected" then return function() return false; end; end
		if type(k) == "string" and k:match("^Get") then return function() return 0; end; end
		return function() return Any; end;
	end });
end

local function NewScreen(cfgText, installed)
	local m = { written = {}, popupTexts = {}, built = {} };
	local env = setmetatable({}, { __index = function(_, k) if _G[k] ~= nil then return _G[k]; end return Any; end });
	env.include = function() end;
	env.Controls = setmetatable({}, { __index = function(t, k) local c = NewControl(); rawset(t, k, c); return c; end });
	env.ContextPtr = setmetatable({
		SetShowHandler = function(_, f) m.show = f; end,
		SetInputHandler = function() end,
		BuildInstanceForControl = function(_, name, inst, parent)
			if name == "KeyBindingCategory" then
				inst.CategoryName = NewControl();
			else
				inst.StringName = NewControl();
				inst.PullDown = NewControl();
			end
			table.insert(m.built, { name = name, inst = inst, parent = parent });
		end,
	}, { __index = Any });
	env.Locale = setmetatable({ Lookup = function(s) return s; end, ToUpper = function(s) return s; end }, { __index = Any });
	env.Mouse = { eLClick = "LClick", eMouseEnter = "Enter" };
	env.PopupDialog = { new = function()
		return setmetatable({
			AddText = function(_, t) table.insert(m.popupTexts, t); end,
		}, { __index = function() return function() end; end });
	end };
	env.InstanceManager = { new = function() return Any; end };
	env.UIManager = setmetatable({ GetScreenSizeVal = function() return 1920, 1080; end }, { __index = Any });
	-- Les options du jeu lues par OnShow : des nombres ; les listes (adaptateurs…) : vides.
	env.Options = setmetatable({}, { __index = function(_, k)
		if k:match("^Get%a+Option$") then return function() return 0; end; end
		if k:match("^GetAvailable") then return function() return {}; end; end
		return Any;
	end });
	env.Civ6Proxy_LireReglages = function() return cfgText, installed; end;
	env.Civ6Proxy_EcrireReglages = function(lines)
		table.insert(m.written, lines);
		return 1;
	end;
	-- IsInGame, Options, UserConfiguration… : Any.
	local f = assert(load(OPTIONS .. "\r\n" .. BLOCK, "Options.lua", "t", env));
	f();
	m.env = env;
	-- La ligne de réglage d'une clé : sa liste et son libellé.
	function m.Row(name)
		for _, b in ipairs(m.built) do
			if b.inst.StringName and b.inst.StringName.text == name then return b.inst; end
		end
	end
	-- Ce que fait le jeu quand on choisit l'entrée `label` d'une liste.
	function m.Choose(row, label)
		for i, e in ipairs(row.PullDown.entries) do
			if e.Button.text == label then
				row.PullDown.selection(i, 0, row.PullDown);
				return true;
			end
		end
		return false;
	end
	return m;
end

local failures = 0;
local function Check(name, ok)
	print((ok and "ok      " or "ECHEC   ") .. name);
	if not ok then failures = failures + 1; end
end

do
	local m = NewScreen("mode=coalesce\r\nui_boutons=on\r\nwmd_accel=7\r\n", 1);
	local C = m.env.Controls;
	Check("onglet visible", C.ProxyTab.hidden == false and C.ProxyTab.callbacks.LClick ~= nil);
	local rows, titles = 0, 0;
	for _, b in ipairs(m.built) do
		if b.parent ~= C.ProxyOptionsStack then rows = -100; end
		if b.name == "PullDownParameterInstance" then rows = rows + 1; else titles = titles + 1; end
	end
	Check("12 réglages et 4 titres, dans le panneau", rows == 12 and titles == 4);
	Check("avertissement de redémarrage", (C.ProxyWarning.text or ""):find("redémarrage") ~= nil);
	Check("état : proxy installé", C.ProxyStatus.text == "Proxy de GameCore installé.");

	local boutons = m.Row("Boutons d'attaque au-dessus des drapeaux d'unités");
	Check("valeur du fichier affichée", boutons.PullDown.button.text == "Activé");
	local nuke = m.Row("Accélération de l'animation des armes nucléaires");
	Check("valeur hors liste montrée telle quelle", nuke.PullDown.button.text == "7 (valeur du fichier)");
	local amel = m.Row("Cache des améliorations constructibles");
	Check("clé absente : valeur par défaut", amel.PullDown.button.text == "Désactivé");

	-- Un changement, puis retour à la valeur d'origine : rien à écrire.
	C.ConfirmButton.disabled = true;
	Check("choix d'une valeur", m.Choose(boutons, "Désactivé"));
	Check("confirmer activé", C.ConfirmButton.disabled == false);
	m.Choose(boutons, "Activé");
	C.ConfirmButton.callbacks.LClick();
	Check("rien d'écrit sans changement réel", #m.written == 0 and not m.env._PromptRestartApp);

	-- Deux changements confirmés : écrits, et le message de redémarrage du jeu s'affiche.
	m.Choose(boutons, "Désactivé");
	m.Choose(amel, "Activé");
	C.ConfirmButton.callbacks.LClick();
	local w = m.written[1] or "";
	Check("écrit à la confirmation", #m.written == 1 and w:find("ui_boutons=off", 1, true) and w:find("ameliorations=quiet", 1, true));
	Check("redémarrage demandé", m.env._PromptRestartApp == true);
	local restart = false;
	for _, t in ipairs(m.popupTexts) do
		if t == "LOC_OPTIONS_CHANGES_REQUIRE_APP_RESTART" then restart = true; end
	end
	Check("message de redémarrage du jeu", restart);

	-- Réouverture : choix en attente abandonnés, fichier relu.
	m.Choose(boutons, "Activé");
	m.show();
	C.ConfirmButton.callbacks.LClick();
	Check("annulé par la réouverture", #m.written == 1);
end

do
	local m = NewScreen("", 0);
	Check("état : proxy non installé", (m.env.Controls.ProxyStatus.text or ""):find("non installé") ~= nil);
end

do
	-- Options.xml non modifié (remplacé par un mod) : le bloc ne fait rien.
	local env = setmetatable({}, { __index = function(_, k) if _G[k] ~= nil then return _G[k]; end return Any; end });
	env.Controls = {};
	local ok = pcall(assert(load(BLOCK, "bloc", "t", env)));
	Check("sans le XML : rien d'ajouté, pas d'erreur", ok);
end

if failures > 0 then
	print(failures .. " échec(s)");
	os.exit(1);
end
print("tout est bon");
