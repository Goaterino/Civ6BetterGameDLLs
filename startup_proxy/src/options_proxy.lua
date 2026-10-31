-- ===========================================================================
-- BetterGameDLLs : onglet de l'écran d'options, ajouté à la fin d'Options.lua par le
-- proxy de démarrage (startup_proxy/src/options.rs). Il édite rollover_proxy.cfg, le fichier de
-- réglages du proxy de GameCore, par deux fonctions natives posées par le proxy :
--   Civ6Proxy_LireReglages()          -> texte, 1|0 (proxy de GameCore installé) ; ou nil, erreur
--   Civ6Proxy_EcrireReglages(lignes)  -> 1 ; ou nil, erreur  (lignes « clé=valeur »)
-- Le proxy de GameCore ne lit ce fichier qu'à son chargement : un changement prend effet au
-- redémarrage du jeu. La confirmation lève donc _PromptRestartApp, et le jeu affiche son propre
-- message « redémarrage nécessaire ».
-- m_tabs, OnSelectTab, OnConfirm, OnShow, PopulateComboBox et _PromptRestartApp sont ceux
-- d'Options.lua.
-- ===========================================================================

local PROXY_ON_OFF = { { "on", "Activé" }, { "off", "Désactivé" } };
local PROXY_CACHE = {
	{ "quiet", "Activé" },
	{ "off", "Désactivé" },
	{ "fast", "Activé, avec journal" },
	{ "count", "Mesure seulement (diagnostic)" },
	{ "verify", "Vérification (diagnostic, plus lent)" },
};

-- Une entrée par réglage lu par le proxy de GameCore (read_config, proxy/src/lib.rs) : un test
-- (cargo test, startup_proxy) vérifie que les deux listes concordent. `defaut` est la valeur que
-- prend le proxy quand la clé manque au fichier.
local PROXY_REGLAGES = {
	{ titre = "Performances du passage de tour" },
	{ cle = "mode", defaut = "coalesce",
		nom = "Regroupement des mises à jour de villes",
		aide = "Pendant une cascade de déclarations de guerre, les données de villes et de diplomatie ne sont rafraîchies qu'une fois, à la fin : plus de 2 s gagnées sur un gros passage de tour. Ne change rien à la partie : sans risque en multijoueur, même face à des joueurs sans proxy.",
		choix = { { "coalesce", "Activé" }, { "off", "Désactivé" }, { "count", "Mesure seulement (diagnostic)" } } },
	{ cle = "portee", defaut = "cascade",
		nom = "Portée du regroupement",
		aide = "« Chaque message du jeu » regroupe aussi les rafraîchissements hors guerre (gouverneurs, ressources, accords). L'interface peut alors montrer une donnée de ville en retard d'un instant.",
		choix = { { "cascade", "Cascades de guerre seulement" }, { "message", "Chaque message du jeu" } } },
	{ cle = "ameliorations", defaut = "off",
		nom = "Cache des améliorations constructibles",
		aide = "Accélère la question « ce joueur peut-il construire cette amélioration ? », posée des millions de fois par passage de tour par l'IA. Même résultat que le jeu, vérifié sans aucun écart.",
		choix = PROXY_CACHE },
	{ cle = "rendements", defaut = "off",
		nom = "Mémoire des rendements de cases pour l'IA",
		aide = "Mémorise le rendement des cases pendant les calculs parallèles de l'IA, où la carte ne peut pas changer.",
		choix = PROXY_CACHE },

	{ titre = "Interface" },
	{ cle = "wmd_accel", defaut = "20",
		nom = "Accélération de l'animation des armes nucléaires",
		aide = "Seulement si le combat rapide est activé (onglet Jeu). Accélère l'affichage pendant la frappe, sans effet sur la partie.",
		choix = { { "off", "Désactivée" }, { "5", "x5" }, { "10", "x10" }, { "20", "x20" }, { "40", "x40" } } },
	{ cle = "ui_boutons", defaut = "on",
		nom = "Boutons d'attaque au-dessus des drapeaux d'unités",
		aide = "Les boutons d'attaque des villes et des campements, et ceux de frappe nucléaire des silos, passent au-dessus des drapeaux d'unités, où ils restent cliquables.",
		choix = PROXY_ON_OFF },
	{ cle = "ui_recentrage", defaut = "off",
		nom = "Recentrer la caméra au clic sur ces boutons",
		aide = "Comportement d'origine du jeu : la caméra se recentre sur la ville au clic sur un bouton d'attaque.",
		choix = { { "on", "Oui" }, { "off", "Non" } } },
	{ cle = "ui_epingles", defaut = "on",
		nom = "Épingles de carte allégées",
		aide = "Les épingles et leur liste ne sont plus reconstruites quand rien n'a changé : moins de ralentissements en multijoueur.",
		choix = PROXY_ON_OFF },
	{ cle = "tours_production", defaut = "on",
		nom = "Compteur de tours de production corrigé",
		aide = "Le nombre de tours affiché tient compte des bonus de gouvernement (fascisme, autocratie, république marchande), comme la production réelle. Affichage seulement.",
		choix = PROXY_ON_OFF },

	{ titre = "Correctifs" },
	{ cle = "ui_merveilles", defaut = "on",
		nom = "Merveille gardée dans la file après un échange",
		aide = "Une merveille retirée de la file de production par un échange y est remise, par des commandes normales du jeu : pas de désynchronisation, même face à des joueurs sans proxy.",
		choix = PROXY_ON_OFF },

	{ titre = "Diagnostic" },
	{ cle = "ui_journal", defaut = "off",
		nom = "Journal des scripts et XML chargés",
		aide = "Note chaque fichier d'interface lu dans le journal du proxy.",
		choix = PROXY_ON_OFF },
	{ cle = "ui_mesure_epingles", defaut = "off",
		nom = "Mesure du coût des épingles",
		aide = "Écrit dans Lua.log la durée des reconstructions d'épingles et du bandeau des dirigeants.",
		choix = PROXY_ON_OFF },
};

local PROXY_AVERTISSEMENT = "Ces réglages sont ceux des proxys BetterGameDLLs (rollover_proxy.cfg). Ils ne prennent effet qu'au prochain redémarrage du jeu.";

local m_proxySaved = {};     -- valeurs du fichier
local m_proxyPending = {};   -- choix pas encore confirmés
local m_proxyPullDowns = {};

local function Proxy_Current(r)
	local v = m_proxyPending[r.cle];
	if v == nil then v = m_proxySaved[r.cle]; end
	if v == nil then v = r.defaut; end
	return v;
end

-- Relit le fichier ; rend le texte d'état à afficher.
local function Proxy_Read()
	m_proxySaved = {};
	if Civ6Proxy_LireReglages == nil then
		return "Fonctions du proxy de démarrage absentes : les réglages ne peuvent être ni lus ni enregistrés.";
	end
	local text, installed = Civ6Proxy_LireReglages();
	if text == nil then
		return "Lecture des réglages impossible : " .. tostring(installed);
	end
	-- Comme read_config : la dernière ligne d'une clé l'emporte.
	for line in string.gmatch(text, "[^\r\n]+") do
		local k, v = string.match(line, "^%s*([%w_]+)%s*=%s*(.-)%s*$");
		if k ~= nil then m_proxySaved[k] = v; end
	end
	if installed == 1 then
		return "Proxy de GameCore installé.";
	end
	return "Proxy de GameCore non installé : ces réglages n'auront d'effet qu'une fois installé.";
end

local function Proxy_Refresh()
	Controls.ProxyStatus:SetText(Proxy_Read());
	for _, r in ipairs(PROXY_REGLAGES) do
		if r.cle ~= nil then
			local current = Proxy_Current(r);
			local values = {};
			local known = false;
			for _, c in ipairs(r.choix) do
				table.insert(values, { c[2], c[1] });
				if c[1] == current then known = true; end
			end
			-- Une valeur écrite à la main, hors de la liste : on la montre telle quelle.
			if not known then
				table.insert(values, { current .. " (valeur du fichier)", current });
			end
			local saved = m_proxySaved[r.cle] or r.defaut;
			PopulateComboBox(m_proxyPullDowns[r.cle], values, current, function(value)
				if value == saved then
					m_proxyPending[r.cle] = nil;
				else
					m_proxyPending[r.cle] = value;
				end
				Controls.ConfirmButton:SetDisabled(false);
			end);
		end
	end
	Controls.ProxyOptionsStack:CalculateSize();
	Controls.ProxyOptionsScrollPanel:CalculateInternalSize();
end

-- Enregistre les choix en attente ; vrai s'il y en avait et qu'ils sont écrits.
local function Proxy_Save()
	local lines = {};
	for k, v in pairs(m_proxyPending) do
		table.insert(lines, k .. "=" .. v);
	end
	if #lines == 0 then return false; end
	if Civ6Proxy_EcrireReglages == nil then
		Controls.ProxyStatus:SetText("Réglages non enregistrés : fonctions du proxy de démarrage absentes.");
		return false;
	end
	local ok, err = Civ6Proxy_EcrireReglages(table.concat(lines, "\n"));
	if ok == nil then
		Controls.ProxyStatus:SetText("Réglages non enregistrés : " .. tostring(err));
		return false;
	end
	for k, v in pairs(m_proxyPending) do
		m_proxySaved[k] = v;
	end
	m_proxyPending = {};
	return true;
end

local function Proxy_Build()
	Controls.ProxyWarning:SetText(PROXY_AVERTISSEMENT);
	for _, r in ipairs(PROXY_REGLAGES) do
		local inst = {};
		if r.titre ~= nil then
			ContextPtr:BuildInstanceForControl("KeyBindingCategory", inst, Controls.ProxyOptionsStack);
			inst.CategoryName:SetText(r.titre);
		else
			ContextPtr:BuildInstanceForControl("PullDownParameterInstance", inst, Controls.ProxyOptionsStack);
			inst.StringName:SetText(r.nom);
			inst.StringName:SetToolTipString(r.aide);
			inst.PullDown:GetButton():SetToolTipString(r.aide);
			m_proxyPullDowns[r.cle] = inst.PullDown;
		end
	end
end

-- Si le XML n'a pas été modifié (Options.xml remplacé par un mod, par exemple), rien n'est ajouté.
if Controls.ProxyTab ~= nil and Controls.ProxyOptions ~= nil and m_tabs ~= nil then
	-- 1 : pas de bouton « Restaurer les valeurs par défaut » sur cet onglet, comme pour les raccourcis.
	local tab = { Controls.ProxyTab, Controls.ProxyOptions, "BetterGameDLLs", 1 };
	table.insert(m_tabs, tab);
	Controls.ProxyTab:RegisterCallback(Mouse.eLClick, function() OnSelectTab(tab); end);
	Controls.ProxyTab:RegisterCallback(Mouse.eMouseEnter, function() UI.PlaySound("Main_Menu_Mouse_Over"); end);
	Controls.ProxyTab:SetHide(false);
	Controls.TabStack:CalculateSize();
	Proxy_Build();
	Proxy_Refresh();

	-- À chaque ouverture : choix en attente abandonnés (Annuler ferme l'écran sans rien écrire),
	-- fichier relu (il a pu changer depuis l'autre écran d'options, menu principal ou partie).
	ContextPtr:SetShowHandler(function()
		OnShow();
		m_proxyPending = {};
		Proxy_Refresh();
	end);

	-- Confirmer : le fichier d'abord, puis la confirmation du jeu, qui affiche le message de
	-- redémarrage si _PromptRestartApp est levé.
	local Proxy_BaseOnConfirm = OnConfirm;
	OnConfirm = function()
		if Proxy_Save() then
			_PromptRestartApp = true;
		end
		Proxy_BaseOnConfirm();
	end
	Controls.ConfirmButton:RegisterCallback(Mouse.eLClick, OnConfirm);

	print("BetterGameDLLs : onglet des proxys ajouté aux options");
end
