-- ===========================================================================
-- Civ6RolloverProxy (src/ui_merveilles.lua) : merveille retirée de la file par un échange, remise par
-- des commandes ordinaires (voir wonder_queue/README.md). Avec `ui_merveilles=on`, le proxy l'ajoute
-- à la fin de ProductionHelper.lua, qui définit SwapQueueItem pour le panneau de production et le
-- gestionnaire de production.
-- Quand un échange remet en tête une merveille déjà posée, GameCore revérifie sa case et la retire.
-- Une merveille rajoutée par le panneau ne porte plus de case et n'est pas revérifiée. Si la file
-- qui suit l'échange est celle attendue sans sa tête, on envoie donc :
-- - le rajout de la merveille en fin de file (comme le panneau) ;
-- - des échanges voisins qui la ramènent en tête, sans changer l'ordre des autres éléments.
-- Ce ne sont que des commandes du jeu : chaque machine les rejoue avec les règles d'origine, proxy
-- ou non, et toutes ont la même file.
-- Le bloc peut s'exécuter plusieurs fois dans un même contexte : SwapQueueItem n'est enveloppée
-- qu'une fois. Tout s'exécute sous pcall ; un échec écrit `Merveilles echec <erreur>` et laisse le
-- script hôte intact.
-- ===========================================================================
do
local Merveilles_Ok, Merveilles_Err = pcall(function()
	if Merveilles_Etat ~= nil or type(SwapQueueItem) ~= "function" then
		return;
	end
	-- attente : échange envoyé, pas encore vu appliqué.
	local S = { attente = nil };
	Merveilles_Etat = S;

	-- Un élément de la file, en une chaîne (nature, type, formation).
	local function Cle(e)
		if e == nil then
			return "?";
		end
		return table.concat({ tostring(e.Directive), tostring(e.UnitType), tostring(e.BuildingType),
			tostring(e.DistrictType), tostring(e.ProjectType), tostring(e.MilitaryFormationType) }, ":");
	end

	local function Lire(pCity)
		local q = pCity:GetBuildQueue();
		local t = {};
		for i = 0, q:GetSize() - 1 do
			t[#t + 1] = Cle(q:GetAt(i));
		end
		return t;
	end

	local function Egales(a, b, depuis)
		depuis = depuis or 1;
		if #a ~= #b - depuis + 1 then
			return false;
		end
		for i = 1, #a do
			if a[i] ~= b[i + depuis - 1] then
				return false;
			end
		end
		return true;
	end

	local function Merveille(e)
		if e == nil or e.Directive ~= CityProductionDirectives.CONSTRUCT then
			return nil;
		end
		local def = GameInfo.Buildings[e.BuildingType];
		if def ~= nil and def.IsWonder then
			return def;
		end
		return nil;
	end

	local function Echanger(pCity, a, b)
		local p = {};
		p[CityOperationTypes.PARAM_INSERT_MODE] = CityOperationTypes.VALUE_SWAP;
		p[CityOperationTypes.PARAM_QUEUE_SOURCE_LOCATION] = a;
		p[CityOperationTypes.PARAM_QUEUE_DESTINATION_LOCATION] = b;
		CityManager.RequestOperation(pCity, CityOperationTypes.BUILD, p);
	end

	local Origine = SwapQueueItem;
	SwapQueueItem = function(sourceIndex, destIndex)
		local pCity = UI.GetHeadSelectedCity();
		local attente = nil;
		if pCity ~= nil and sourceIndex ~= destIndex and (sourceIndex == 0 or destIndex == 0) then
			local q = pCity:GetBuildQueue();
			local autre = sourceIndex == 0 and destIndex or sourceIndex;
			local def = autre < q:GetSize() and Merveille(q:GetAt(autre)) or nil;
			if def ~= nil then
				local avant = Lire(pCity);
				local apres = {};
				for i, k in ipairs(avant) do apres[i] = k; end
				apres[1], apres[autre + 1] = avant[autre + 1], avant[1];
				attente = { joueur = pCity:GetOwner(), ville = pCity:GetID(), hash = def.Hash,
					nom = def.BuildingType, avant = avant, apres = apres };
			end
		end
		Origine(sourceIndex, destIndex);
		if attente ~= nil then
			S.attente = attente;
		end
	end

	local function OnQueueChanged(playerID, cityID)
		local a = S.attente;
		if a == nil or playerID ~= a.joueur or cityID ~= a.ville then
			return;
		end
		local pCity = CityManager.GetCity(playerID, cityID);
		if pCity == nil then
			S.attente = nil;
			return;
		end
		local file = Lire(pCity);
		if Egales(file, a.avant) then
			return; -- l'échange n'est pas encore appliqué
		end
		S.attente = nil;
		if not Egales(file, a.apres, 2) then
			return; -- appliqué sans retrait, ou file changée autrement : rien à faire
		end
		print("Merveilles : " .. a.nom .. " retiree par l'echange, remise en tete");
		local p = {};
		p[CityOperationTypes.PARAM_BUILDING_TYPE] = a.hash;
		p[CityOperationTypes.PARAM_INSERT_MODE] = CityOperationTypes.VALUE_APPEND;
		CityManager.RequestOperation(pCity, CityOperationTypes.BUILD, p);
		for i = #file, 1, -1 do
			Echanger(pCity, i, i - 1);
		end
	end
	Events.CityProductionQueueChanged.Add(OnQueueChanged);
end);
if not Merveilles_Ok then
	print("Merveilles echec " .. tostring(Merveilles_Err));
end
end
