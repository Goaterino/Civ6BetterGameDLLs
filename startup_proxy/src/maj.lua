-- BetterGameDLLs : notification de mise à jour au menu principal (voir startup_proxy/src/maj.rs).
-- Ajouté à la fin de MainMenu.lua. Natives posées par le proxy : Civ6Proxy_MiseAJour([action]) et
-- Civ6Proxy_Journal(texte). Le bloc enveloppe le gestionnaire d'affichage du menu, et le temps de la
-- vérification, sa mise à jour par image ; une erreur du bloc va au journal, jamais au menu.

local BGD_MAJ_FR = false;
do
	local language = Locale and Locale.GetCurrentLanguage and Locale.GetCurrentLanguage();
	local code = language and language.Type;
	BGD_MAJ_FR = type(code) == "string" and string.sub(string.lower(code), 1, 2) == "fr";
end

-- Chaque texte : { français, anglais }.
local BGD_MAJ_TEXTES = {
	texte = {
		"La version %s de BetterGameDLLs est disponible (installée : %s).[NEWLINE][NEWLINE]« Mettre à jour » ferme le jeu et lance l'installeur.",
		"BetterGameDLLs version %s is available (installed: %s).[NEWLINE][NEWLINE]\"Update\" closes the game and starts the installer.",
	},
	maj = { "Mettre à jour", "Update" },
	plus_tard = { "Plus tard", "Later" },
	page = {
		"L'installeur de BetterGameDLLs est introuvable : la page de la dernière version s'est ouverte dans le navigateur.",
		"The BetterGameDLLs installer was not found: the latest release page has opened in the browser.",
	},
};

local function BGD_Maj_T(t)
	if BGD_MAJ_FR then return t[1]; end
	return t[2];
end

-- Plus rien à faire : pas de proxy, vérification finie, ou notification déjà montrée.
local bgd_majFinie = Civ6Proxy_MiseAJour == nil;

local function BGD_Maj_Journal(texte)
	if Civ6Proxy_Journal then pcall(Civ6Proxy_Journal, "maj : " .. texte); end
end

local function BGD_Maj_Installer()
	if Civ6Proxy_MiseAJour("installer") == 1 then
		BGD_Maj_Journal("fermeture du jeu pour l'installeur");
		UI.ExitGame();
	else
		g_PopupDialog:ShowOkDialog(BGD_Maj_T(BGD_MAJ_TEXTES.page));
	end
end

local function BGD_Maj_Verifier()
	if bgd_majFinie then return; end
	-- Pas par-dessus un autre message du menu (défis, erreur de lancement) : on attendra sa fermeture.
	if g_PopupDialog == nil or g_PopupDialog:IsOpen() then return; end
	local latest, current = Civ6Proxy_MiseAJour();
	if latest == "" then return; end
	bgd_majFinie = true;
	if latest == nil then return; end
	BGD_Maj_Journal("notification de la version " .. latest);
	g_PopupDialog:Reset();
	g_PopupDialog:AddTitle("BetterGameDLLs");
	g_PopupDialog:AddText(string.format(BGD_Maj_T(BGD_MAJ_TEXTES.texte), latest, current));
	g_PopupDialog:AddButton(BGD_Maj_T(BGD_MAJ_TEXTES.maj), BGD_Maj_Installer, PopupDialog.COMMAND_CONFIRM);
	g_PopupDialog:AddButton(BGD_Maj_T(BGD_MAJ_TEXTES.plus_tard), function() Civ6Proxy_MiseAJour("plus_tard"); end, PopupDialog.COMMAND_CANCEL);
	g_PopupDialog:Open();
end

local function BGD_Maj_Proteger(f, ...)
	local ok, err = pcall(f, ...);
	if not ok then
		bgd_majFinie = true;
		BGD_Maj_Journal("erreur : " .. tostring(err));
	end
end

if not bgd_majFinie and ContextPtr ~= nil then
	local onShow = OnShow;
	local onUpdate = OnUpdate;
	ContextPtr:SetShowHandler(function(...)
		if onShow then onShow(...); end
		-- OnShow vient de poser OnUpdate : on l'enveloppe tant que la notification reste à montrer.
		if not bgd_majFinie then
			ContextPtr:SetUpdate(function(dt)
				if onUpdate then onUpdate(dt); end
				BGD_Maj_Proteger(BGD_Maj_Verifier);
			end);
		end
	end);
	BGD_Maj_Journal("bloc chargé");
end
