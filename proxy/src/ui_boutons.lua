
-- ===========================================================================
-- Civ6RolloverProxy (src/ui_boutons.lua), ajouté par le proxy à la fin de CityBannerManager.lua.
-- Les boutons d'attaque (ville, campement) et de frappe nucléaire (silo) passent au-dessus des
-- drapeaux d'unités : chaque bouton est déplacé dans un calque qui reproduit le cadre de sa
-- bannière (instances ProxyStrike* de ui_boutons.xml), placé dans SelectedUnitContainer, que
-- InGame.xml dessine après BannerAndFlags. Le bouton garde ses rappels et ses mises à jour : le
-- code de ce fichier le retrouve toujours par l'instance de sa bannière.
-- Fonction anonyme : ses locales ne comptent pas dans la limite de celles du fichier.
-- ===========================================================================
(function()
	local layer = ContextPtr:LookUpControl("../SelectedUnitContainer");
	if layer == nil then
		print("Civ6RolloverProxy : SelectedUnitContainer introuvable, boutons laissés dans les bannières");
		return;
	end

	local overlays = {};		-- instance de bannière -> calque
	local hiddenAll = false;	-- révélation d'une merveille : le contexte entier est masqué

	local function kindOf(banner)
		local im = banner.m_InstanceManager;
		if im == nil then return nil; end
		if im == m_CityBannerIM then return "City"; end
		if im == m_EncampmentBannerIM then return "Encampment"; end
		if im == m_WMDBannerIM then return "WMD"; end
		return nil;
	end

	-- Le calque est visible quand la bannière et le cadre d'où vient le bouton le sont.
	local function sync(banner)
		local inst = banner.m_Instance;
		local o = inst and overlays[inst];
		if o == nil then return; end
		o.Banner = banner;
		o.Anchor:SetHide(hiddenAll or inst.Anchor:IsHidden() or o.Source:IsHidden());
	end

	-- Même position que CityBanner:UpdatePosition.
	local function place(banner)
		local inst = banner.m_Instance;
		local o = inst and overlays[inst];
		if o == nil or banner.m_PlotX == nil then return; end
		local x, y, z = UI.GridToWorld(banner.m_PlotX, banner.m_PlotY);
		if UI.GetWorldRenderView() == WorldRenderView.VIEW_2D then
			y = y + (YOFFSET_2DVIEW or 26);
		else
			z = z + (ZOFFSET_3DVIEW or 36);
		end
		o.Anchor:SetWorldPositionVal(x, y, z);
	end

	local function attach(banner)
		local inst = banner.m_Instance;
		if inst == nil then return; end
		if overlays[inst] == nil then
			local kind = kindOf(banner);
			if kind == nil then return; end
			local o = {};
			-- Construit dans ce contexte puis déplacé, comme MapPinManager pour l'épingle choisie.
			ContextPtr:BuildInstanceForControl("ProxyStrike" .. kind, o, Controls.CityBanners);
			if kind == "WMD" then
				o.Source = inst.WMDBannerContainer;
				o.Moved = { {inst.NukeBombButtonBackground, inst.NukeBombButtonBackground:GetParent()},
				            {inst.ThermoNukeBombButtonBackground, inst.ThermoNukeBombButtonBackground:GetParent()} };
				inst.NukeBombButtonBackground:ChangeParent(o.NukeSlot);
				inst.ThermoNukeBombButtonBackground:ChangeParent(o.ThermoSlot);
			else
				o.Source = (kind == "City") and inst.Container or inst.EncampmentBannerContainer;
				o.Moved = { {inst.CityStrike, o.Source} };
				inst.CityStrike:ChangeParent(o.Frame);
			end
			o.Anchor:ChangeParent(layer);
			overlays[inst] = o;
		end
		place(banner);
		sync(banner);
	end

	local function after(name, fn)
		local original = CityBanner[name];
		if original == nil then return; end
		CityBanner[name] = function(self, ...)
			local r = original(self, ...);
			fn(self);
			return r;
		end
	end

	after("Initialize", attach);
	after("UpdatePosition", place);
	after("UpdateVisibility", sync);
	after("UpdateRangeStrike", sync);
	after("UpdateWMDBanner", sync);
	-- Bannière détruite : son instance est masquée par l'InstanceManager, le calque aussi.
	after("Uninitialize", function(self)
		local o = self.m_Instance and overlays[self.m_Instance];
		if o ~= nil then o.Anchor:SetHide(true); end
	end);

	Events.BeginWonderReveal.Add(function()
		hiddenAll = true;
		for _, o in pairs(overlays) do o.Anchor:SetHide(true); end
	end);
	Events.EndWonderReveal.Add(function()
		hiddenAll = false;
		for _, o in pairs(overlays) do
			if o.Banner ~= nil then sync(o.Banner); end
		end
	end);

	-- Arrêt du contexte : chaque bouton revient dans sa bannière, rien ne reste dans un conteneur
	-- d'InGame qui survit à ce contexte.
	local shutdown = OnShutdown;
	ContextPtr:SetShutdown(function()
		for _, o in pairs(overlays) do
			for _, m in ipairs(o.Moved) do m[1]:ChangeParent(m[2]); end
			o.Anchor:ChangeParent(Controls.CityBanners);
			o.Anchor:SetHide(true);
		end
		if type(shutdown) == "function" then shutdown(); end
	end);

	-- Bannières créées avant ce bloc (rechargement à chaud).
	for _, t in pairs({CityBannerInstances or {}, MiniBannerInstances or {}}) do
		for _, perPlayer in pairs(t) do
			for _, banner in pairs(perPlayer) do attach(banner); end
		end
	end
	print("Civ6RolloverProxy : boutons d'attaque au-dessus des drapeaux d'unités");
end)();
