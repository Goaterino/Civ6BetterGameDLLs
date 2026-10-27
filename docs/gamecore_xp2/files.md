# GameCore_XP2_FinalRelease.dll : fichiers source

Genere par `~/tools/ghidra-scripts/sourcemap_build.py`, voir README.md pour les methodes.
Ordre = ordre des objets dans `.text` (celui de Release). `-` : fichier present dans Release dont aucune fonction FinalRelease n'est rattachee.

## Par dossier

| dossier | fichiers | fonctions | octets |
|---|---:|---:|---:|
| `GameCore\Core\Cache` | 12 | 68 | 11410 |
| `GameCore\Core\Common\CityCommands` | 1 | 9 | 1760 |
| `GameCore\Core\Common\CityOperations` | 1 | 3 | 435 |
| `GameCore\Core\Lua` | 4 | 174 | 34066 |
| `GameCore\Core\Common\MapGen` | 10 | 51 | 19612 |
| `GameCore\Core\Common\Messages` | 14 | 30 | 12239 |
| `GameCore\Core\Common\Notifications` | 3 | 38 | 7697 |
| `GameCore\Core\Common\UnitCommands` | 1 | 11 | 1704 |
| `GameCore\Core\Common\UnitOperations` | 3 | 39 | 10327 |
| `GameCore\Core\Common` | 21 | 351 | 78886 |
| `GameCore\XP2\Cache` | 98 | 660 | 139064 |
| `GameCore\XP2\Common` | 162 | 4764 | 1725539 |
| `GameCore\XP2\Common\AI` | 40 | 1395 | 709192 |
| `GameCore\XP2\Common\Analysis` | 4 | 10 | 3961 |
| `GameCore\XP2\Common\AI\BehaviorTree` | 17 | 147 | 126169 |
| `GameCore\XP2\Common\DiplomacyDeals` | 10 | 162 | 45490 |
| `GameCore\XP2\Common\Exports` | 15 | 82 | 8700 |
| `GameCore\XP2\Common\GameQuests` | 10 | 49 | 9615 |
| `GameCore\XP2\Common\Lua` | 1 | 0 | 0 |
| `GameCore\XP2\Common\MapGen` | 8 | 157 | 78136 |
| `GameCore\XP2\Common\Notifications` | 14 | 14 | 1372 |
| `GameCore\XP2\Common\UnitCommands` | 9 | 20 | 7320 |
| `GameCore\Shared` | 31 | 268 | 77794 |
| `GameCore\Shared\Base` | 16 | 357 | 117972 |
| `GameCore\Shared\Base\Notifications` | 24 | 27 | 2641 |
| `GameCore\Shared\Base\UnitCommands` | 34 | 127 | 42155 |
| `GameCore\Shared\Base\UnitOperations` | 56 | 146 | 44119 |
| `GameCore\Shared\Base\CityCommands` | 8 | 35 | 6864 |
| `GameCore\Shared\Base\CityOperations` | 1 | 12 | 4564 |
| `GameCore\Shared\Base\Cache` | 60 | 93 | 13055 |
| `GameCore\Shared\Base\Lua` | 17 | 24 | 5062 |
| `GameCore\Shared\XP1` | 1 | 37 | 10536 |
| `GameCore\Shared\XP1\Cache` | 9 | 29 | 12946 |
| `GameCore\Shared\XP1\Lua` | 8 | 8 | 962 |
| `GameCore\Shared\XP1\UnitCommands` | 1 | 3 | 566 |
| `GameCore\Shared\XP1\UnitOperations` | 13 | 21 | 7257 |
| `GameCore\Shared\XP1\CityCommands` | 2 | 3 | 797 |
| `GameCore\Shared\XP2` | 3 | 211 | 68047 |
| `GameCore\Shared\XP2\Cache` | 19 | 28 | 4213 |
| `GameCore\Shared\XP2\Lua` | 6 | 6 | 672 |
| `GameCore\Shared\XP2\UnitCommands` | 1 | 1 | 255 |
| `GameCore\Shared\XP2\UnitOperations` | 14 | 42 | 21117 |
| `GameCore\Shared\XP2\CItyCommands` | 1 | 1 | 119 |
| `GameCore\Shared\XP2\CityOperations` | 1 | 1 | 126 |
| `GameCore\XP2\Target` | 5 | 7 | 3128 |
| `GameCore\Shared\Base\GameEffects` | 38 | 1717 | 393751 |
| `GameCore\XP2\Common\GameEffects` | 24 | 65 | 40039 |
| `GameCore\Shared\XP1\GameEffects` | 10 | 1 | 897 |
| `GameCore\Core\Common\GameEffects` | 12 | 40 | 22353 |
| `CivTech\Platform\Windows` | 4 | 7 | 847 |
| `CivTech\Platform\Common` | 4 | 8 | 649 |
| `CivTech\String\Common` | 3 | 4 | 177 |
| `CivTech\Telemetry\src` | 1 | 1 | 59 |
| `G:\BuildAgent\work\26d31e7d2eca7bbe\CivTech\SDKSources\EASTL\src` | 1 | 0 | 0 |
| `CivTech\AutoVariable` | 1 | 12 | 4120 |
| `CivTech\Database\source\Utility` | 1 | 1 | 175 |
| `CivTech\DataVariant` | 7 | 93 | 22739 |
| `CivTech\Serialization\Common` | 1 | 12 | 755 |
| `CivTech\XML` | 1 | 6 | 920 |
| `CivTech\Lua` | 3 | 18 | 4927 |

## Par fichier

| fichier | debut | fin | fonctions | octets | methodes | ligne max | Lua |
|---|---|---|---:|---:|---|---:|---|
| `GameCore\Core\Cache\Cache_BarbarianTribe_Manager.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Cache\Cache_Context_Instance.cpp` | 18000a1a0 | 18000a1a0 | 1 | 82 | fichier:1 | 227 |  |
| `GameCore\Core\Cache\Cache_Game_EconomicManager.cpp` | 18000aab0 | 18000b0e0 | 12 | 1492 | bloc:2 release:10 |  |  |
| `GameCore\Core\Cache\Cache_Game_EconomicManager_EventStream.cpp` | 18000b400 | 18000b510 | 4 | 406 | bloc:1 fichier:1 release:2 | 28 | BarbsKilledIcon, CompStompIcon, TotalAICities, UnpillagedIcon |
| `GameCore\Core\Cache\Cache_Game_HeroesManager.cpp` | 18000c060 | 18000c940 | 9 | 2204 | bloc:3 release:6 |  |  |
| `GameCore\Core\Cache\Cache_Game_HeroesManager_EventStream.cpp` | 18000ce20 | 18000cef0 | 3 | 387 | fichier:1 release:2 | 29 |  |
| `GameCore\Core\Cache\Cache_Lua_IGameEconomicManager.cpp` | 18000d5b0 | 18000d5b0 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Core\Cache\Cache_Map_CityPlots.cpp` | 18000e7a0 | 18000ed50 | 12 | 1626 | bloc:5 release:7 |  |  |
| `GameCore\Core\Cache\Cache_Map_EventStream.cpp` | 18000f3b0 | 180010180 | 16 | 4175 | bloc:3 fichier:1 release:12 | 103 |  |
| `GameCore\Core\Cache\Cache_Map_Instance.cpp` | 180011920 | 180011bd0 | 10 | 926 | bloc:1 fichier:1 release:8 |  |  |
| `GameCore\Core\Cache\Cache_Map_UnitContainer.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Cache\Cache_Plot_Instance.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\CityCommands\City_Commands_Manager.cpp` | 180013650 | 180013dd0 | 9 | 1760 | fichier:1 release:8 |  | CityCommands |
| `GameCore\Core\Common\CityOperations\City_Operations_Manager.cpp` | 180014070 | 1800141e0 | 3 | 435 | fichier:1 release:2 |  | CityOperations |
| `GameCore\Core\Lua\Lua_IMap.cpp` | 180018270 | 180018270 | 1 | 447 | lua:1 |  | GetPlotDistance |
| `GameCore\Core\Lua\Lua_IMapPlot.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Lua\Lua_IMapStartPositioner.cpp` | 180018fd0 | 1800215a0 | 172 | 33369 | bloc:168 chaine:3 lua:1 |  | Calculate, CanHaveDistrict, CanHaveImprovement, CanHaveResource, CanHaveRoute, CanHaveWonder (+131) |
| `GameCore\Core\Lua\Lua_Utility.cpp` | 180026680 | 180026680 | 1 | 250 | release:1 |  |  |
| `GameCore\Core\Common\MapGen\Area_Container.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\MapGen\Landmass_Container.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\MapGen\Map_Area.cpp` | 180029bb0 | 18002a180 | 2 | 2085 | release:2 |  |  |
| `GameCore\Core\Common\MapGen\Map_Continent.cpp` | 18002aae0 | 18002aae0 | 1 | 311 | release:1 |  |  |
| `GameCore\Core\Common\MapGen\Map_Landmass.cpp` | 18002ae10 | 18002ae90 | 2 | 158 | release:2 |  |  |
| `GameCore\Core\Common\MapGen\Map_Region.cpp` | 18002b470 | 18002c360 | 16 | 4109 | bloc:2 release:14 |  |  |
| `GameCore\Core\Common\MapGen\Map_Territory.cpp` | 18002c6c0 | 18002ce60 | 11 | 1904 | bloc:2 release:9 |  |  |
| `GameCore\Core\Common\MapGen\Region_Container.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\MapGen\Territory_Builder.cpp` | 18002f390 | 180030f30 | 18 | 10872 | bloc:4 fichier:1 release:13 | 29 | TerritoryBuilder |
| `GameCore\Core\Common\MapGen\Territory_Container.cpp` | 180032ea0 | 180032ea0 | 1 | 173 | fichier:1 |  | Territories |
| `GameCore\Core\Common\Messages\Messages_CityCommand.cpp` | 180033e80 | 180033e80 | 1 | 225 | release:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_CityOperation.cpp` | 1800343b0 | 1800343b0 | 1 | 145 | release:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_CivicsCommand.cpp` | 180034840 | 180034840 | 1 | 412 | release:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_GameTurnComplete.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\Messages\Messages_GameUpdate.cpp` | 180036180 | 180036180 | 1 | 586 | fichier:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_Manager.cpp` | 1800366b0 | 180037f70 | 19 | 6267 | bloc:16 fichier:2 release:1 | 204 |  |
| `GameCore\Core\Common\Messages\Messages_PlayerConnect.cpp` | 1800383e0 | 1800383e0 | 1 | 1100 | chaine:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_PlayerRetire.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\Messages\Messages_PlayerSnapshotStatus.cpp` | 180039020 | 180039020 | 1 | 867 | chaine:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_PlayerSwap.cpp` | 180039650 | 180039650 | 1 | 361 | release:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_PlayerTurnComplete.cpp` | 180039920 | 180039920 | 1 | 1230 | fichier:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_PlayerTurnUnready.cpp` | 180039fb0 | 180039fb0 | 1 | 753 | fichier:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_UnitCommands.cpp` | 18003a560 | 18003a560 | 1 | 152 | release:1 |  |  |
| `GameCore\Core\Common\Messages\Messages_UnitOperation.cpp` | 18003aad0 | 18003aad0 | 1 | 141 | release:1 |  |  |
| `GameCore\Core\Common\Notifications\Notification_Instance.cpp` | 18003ae30 | 18003bea0 | 21 | 4819 | bloc:2 fichier:1 release:18 | 744 |  |
| `GameCore\Core\Common\Notifications\Notification_Manager.cpp` | 18003c9b0 | 18003d380 | 16 | 2373 | bloc:11 chaine:1 fichier:1 release:3 | 114 |  |
| `GameCore\Core\Common\Notifications\Notification_Player.cpp` | 18003d850 | 18003d850 | 1 | 505 | release:1 |  |  |
| `GameCore\Core\Common\UnitCommands\Unit_Commands_Manager.cpp` | 18003e770 | 18003edc0 | 11 | 1704 | bloc:2 release:9 |  |  |
| `GameCore\Core\Common\UnitOperations\Unit_Operations_BaseHandler.cpp` | 18003f060 | 18003f060 | 1 | 462 | release:1 |  |  |
| `GameCore\Core\Common\UnitOperations\Unit_Operations_Manager.cpp` | 18003f740 | 180041980 | 34 | 8496 | bloc:22 chaine:5 fichier:1 release:6 |  | UnitOperations |
| `GameCore\Core\Common\UnitOperations\Unit_Operations_Queue.cpp` | 180041c30 | 180041fc0 | 4 | 1369 | bloc:2 chaine:1 release:1 |  |  |
| `GameCore\Core\Common\City_CManager.cpp` | 180042b60 | 180042ec0 | 9 | 885 | bloc:1 release:8 |  |  |
| `GameCore\Core\Common\City_Utility.cpp` | 180044810 | 180044ba0 | 6 | 1236 | release:6 |  |  |
| `GameCore\Core\Common\Context_Instance.cpp` | 180045630 | 1800470b0 | 32 | 6890 | bloc:19 chaine:1 fichier:7 release:5 | 648 |  |
| `GameCore\Core\Common\Game_Capabilities.cpp` | 180048160 | 180048760 | 10 | 2002 | bloc:2 chaine:1 release:7 |  |  |
| `GameCore\Core\Common\Game_Processor.cpp` | 180049390 | 18004afc0 | 18 | 9285 | bloc:14 fichier:4 | 652 |  |
| `GameCore\Core\Common\Map_CityPlots.cpp` | 18004bed0 | 18004c6f0 | 12 | 2279 | bloc:1 release:11 |  |  |
| `GameCore\Core\Common\Map_FeaturePlots.cpp` | 18004d160 | 18004da20 | 13 | 2220 | bloc:6 release:7 |  |  |
| `GameCore\Core\Common\Map_IInstance.cpp` | 18004eb20 | 18004f150 | 4 | 3077 | release:4 |  |  |
| `GameCore\Core\Common\Map_Instance.cpp` | 180051010 | 1800532e0 | 51 | 8944 | bloc:9 fichier:1 release:41 | 225 |  |
| `GameCore\Core\Common\Map_UnitContainer.cpp` | 180055980 | 1800571a0 | 40 | 6296 | bloc:11 fichier:1 release:28 |  |  |
| `GameCore\Core\Common\Map_Utility.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\Pathfinder_Instance.cpp` | 180057ff0 | 180058fe0 | 13 | 4183 | bloc:8 chaine:1 fichier:1 release:3 | 198 |  |
| `GameCore\Core\Common\Pathfinder_NodeSerialization.cpp` | 180059860 | 180059860 | 1 | 373 | release:1 |  |  |
| `GameCore\Core\Common\Pathfinder_TwoLayer.cpp` | 180059bf0 | 180059de0 | 3 | 922 | bloc:1 chaine:1 fichier:1 | 51 |  |
| `GameCore\Core\Common\Phase_Processor.cpp` | 18005a950 | 18005b8d0 | 19 | 4011 | bloc:9 fichier:1 release:9 | 180 |  |
| `GameCore\Core\Common\Player_Operations_ExecuteScript.cpp` | 18005be30 | 18005be30 | 1 | 99 | fichier:1 | 100 |  |
| `GameCore\Core\Common\Player_Operations_StartObserverMode.cpp` | 18005bfb0 | 18005bfb0 | 1 | 99 | fichier:1 | 80 |  |
| `GameCore\Core\Common\Player_Processor.cpp` | 18005c4a0 | 18005de40 | 32 | 7268 | bloc:29 fichier:3 | 743 |  |
| `GameCore\Core\Common\Plot_Instance.cpp` | 18005ee90 | 180062180 | 57 | 14037 | bloc:5 release:52 |  |  |
| `GameCore\Core\Common\Route_Instance.cpp` | 180063240 | 1800643c0 | 23 | 4422 | bloc:4 fichier:1 release:18 | 380 | Route |
| `GameCore\Core\Common\Rules_Manager.cpp` | 180065570 | 1800656f0 | 6 | 358 | bloc:1 fichier:1 release:4 | 207 |  |
| `GameCore\XP2\Cache\Cache_Ai_GrandStrategic_EventStream.cpp` | 18006a160 | 18006a2e0 | 3 | 435 | fichier:1 release:2 | 34 |  |
| `GameCore\XP2\Cache\Cache_Ai_InfluenceMap.cpp` | 18006a780 | 18006a780 | 1 | 301 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_City_BuildQueue.cpp` | 18006b060 | 18006ca10 | 40 | 6028 | bloc:15 release:25 |  |  |
| `GameCore\XP2\Cache\Cache_City_BuildQueue_EventStream.cpp` | 18006d880 | 18006d950 | 3 | 401 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_City_Buildings.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_Buildings_EventStream.cpp` | 18006e960 | 18006ecb0 | 6 | 1000 | fichier:1 release:5 | 33 |  |
| `GameCore\XP2\Cache\Cache_City_Citizens.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_Citizens_EventStream.cpp` | 18006f760 | 18006f830 | 3 | 390 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_City_Culture.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_EventStream.cpp` | 1800725d0 | 180072db0 | 17 | 2065 | bloc:1 fichier:1 release:15 | 44 |  |
| `GameCore\XP2\Cache\Cache_City_Gold.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_Growth.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_Instance.cpp` | 180075360 | 180075aa0 | 7 | 1901 | chaine:2 release:5 |  |  |
| `GameCore\XP2\Cache\Cache_City_Power.cpp` | 180077cb0 | 180077cb0 | 1 | 284 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_City_Religion.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_City_Trade.cpp` | 18007a150 | 18007a9f0 | 7 | 2183 | release:7 |  |  |
| `GameCore\XP2\Cache\Cache_Dispatcher.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_District_EventStream.cpp` | 18007cfa0 | 18007d1c0 | 6 | 775 | bloc:1 fichier:1 release:4 | 33 |  |
| `GameCore\XP2\Cache\Cache_District_Instance.cpp` | 18007de00 | 18007de00 | 1 | 325 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Emergency_EventStream.cpp` | 18007f280 | 18007f300 | 2 | 161 | fichier:1 release:1 | 36 |  |
| `GameCore\XP2\Cache\Cache_Emergency_Manager.cpp` | 18007fc60 | 180081de0 | 3 | 8572 | chaine:1 release:2 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Climate.cpp` | 180085740 | 180087030 | 30 | 6323 | bloc:2 fichier:1 release:27 | 302 | GameClimate |
| `GameCore\XP2\Cache\Cache_Game_Climate_EventStream.cpp` | 18008c740 | 18008cfe0 | 12 | 3651 | bloc:1 fichier:1 release:10 | 48 |  |
| `GameCore\XP2\Cache\Cache_Game_Culture.cpp` | 18008deb0 | 18008e330 | 7 | 1190 | bloc:1 release:6 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Diplomacy.cpp` | 18008e800 | 18008ee30 | 9 | 1610 | bloc:1 release:8 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Eras.cpp` | 18008f820 | 180090c90 | 67 | 4647 | bloc:63 release:4 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Eras_EventStream.cpp` | 180091140 | 180091210 | 3 | 387 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Game_EventStream.cpp` | 180091bb0 | 180091d40 | 5 | 909 | fichier:1 release:4 | 34 |  |
| `GameCore\XP2\Cache\Cache_Game_Fallout_Manager.cpp` | 1800926a0 | 180092980 | 9 | 717 | bloc:2 release:7 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Fallout_Manager_EventStream.cpp` | 180092e90 | 1800930e0 | 5 | 718 | bloc:1 fichier:1 release:3 | 34 |  |
| `GameCore\XP2\Cache\Cache_Game_Gossip_Manager.cpp` | 180093270 | 180093cc0 | 12 | 3295 | bloc:10 fichier:2 | 96 |  |
| `GameCore\XP2\Cache\Cache_Game_GreatPeople.cpp` | 180094b90 | 180095340 | 12 | 1911 | release:12 |  |  |
| `GameCore\XP2\Cache\Cache_Game_GreatPeople_EventStream.cpp` | 180095ac0 | 180095bd0 | 3 | 455 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Game_History_Manager.cpp` | 180096780 | 180097130 | 12 | 2400 | bloc:4 release:8 |  |  |
| `GameCore\XP2\Cache\Cache_Game_History_Manager_EventStream.cpp` | 180097be0 | 180097d00 | 3 | 725 | bloc:1 fichier:1 release:1 | 35 |  |
| `GameCore\XP2\Cache\Cache_Game_Instance.cpp` | 180098160 | 180098a50 | 14 | 2899 | bloc:9 fichier:2 release:3 | 326 |  |
| `GameCore\XP2\Cache\Cache_Game_NationalParks.cpp` | 180099f60 | 18009a930 | 14 | 2439 | bloc:1 release:13 |  |  |
| `GameCore\XP2\Cache\Cache_Game_NationalParks_EventStream.cpp` | 18009aff0 | 18009b0c0 | 3 | 387 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Game_Religion.cpp` | 18009b890 | 18009c110 | 11 | 2128 | release:11 |  |  |
| `GameCore\XP2\Cache\Cache_Game_Religion_EventStream.cpp` | 18009c7f0 | 18009c8c0 | 3 | 387 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Game_Techs.cpp` | 18009ce10 | 18009ced0 | 4 | 252 | bloc:1 release:3 |  |  |
| `GameCore\XP2\Cache\Cache_Game_VictoryManager.cpp` | 18009d110 | 18009d2f0 | 6 | 474 | release:6 |  |  |
| `GameCore\XP2\Cache\Cache_Game_VictoryManager_EventStream.cpp` | 18009d920 | 18009d9f0 | 3 | 387 | fichier:1 release:2 | 29 |  |
| `GameCore\XP2\Cache\Cache_Lua_ICityPower.cpp` | 18009f2a0 | 1800a0150 | 8 | 4568 | bloc:4 chaine:4 |  | GetGeneratedPowerSources, GetMissingPower, GetPlotsCoveredByRegionalPower, GetPlotsProvidingPower, GetRequiredPower, GetRequiredPowerSources (+2) |
| `GameCore\XP2\Cache\Cache_Lua_IGameEras.cpp` | 1800a6b80 | 1800a7db0 | 9 | 5541 | bloc:1 chaine:2 lua:6 |  | GetPlayerCurrentEraScoreBreakdown, GetPlayerCurrentScore, GetPlayerDarkAgeThreshold, GetPlayerDarkAgeThresholdBreakdown, GetPlayerGoldenAgeThreshold, GetPlayerGoldenAgeThresholdBreakdown (+3) |
| `GameCore\XP2\Cache\Cache_Lua_IWorldCongress.cpp` | 1800a9680 | 1800ad610 | 6 | 22564 | bloc:1 chaine:5 |  | GetEmergencies, GetMeetingStatus, GetPreviousVotesOnResolution, GetProposals, GetResolutions, GetReview |
| `GameCore\XP2\Cache\Cache_ObjectPairingManager.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_ObjectPairingManager_EventStream.cpp` | 1800b0b50 | 1800b0b50 | 1 | 157 | fichier:1 | 28 |  |
| `GameCore\XP2\Cache\Cache_Player_Ai_EventStream.cpp` | 1800b1160 | 1800b1160 | 1 | 88 | fichier:1 | 35 |  |
| `GameCore\XP2\Cache\Cache_Player_Cities.cpp` | 1800b1b60 | 1800b1cc0 | 4 | 412 | release:4 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Culture.cpp` | 1800b2520 | 1800b3950 | 51 | 4931 | bloc:3 release:48 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Culture_EventStream.cpp` | 1800b4280 | 1800b4490 | 5 | 564 | fichier:1 release:4 | 32 |  |
| `GameCore\XP2\Cache\Cache_Player_Diplomacy.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Player_Diplomacy_EventStream.cpp` | 1800b7350 | 1800b7630 | 5 | 798 | fichier:1 release:4 | 34 |  |
| `GameCore\XP2\Cache\Cache_Player_Districts.cpp` | 1800b7ea0 | 1800b7ea0 | 1 | 198 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_EventStream.cpp` | 1800b8670 | 1800b9060 | 19 | 3115 | bloc:3 fichier:1 release:15 | 46 |  |
| `GameCore\XP2\Cache\Cache_Player_Governors_EventStream.cpp` | 1800bb2f0 | 1800bb3c0 | 3 | 386 | fichier:1 release:2 | 31 |  |
| `GameCore\XP2\Cache\Cache_Player_GreatPeoplePoints.cpp` | 1800bb960 | 1800bb960 | 1 | 120 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_GreatPeoplePoints_EventStream.cpp` | 1800bbe20 | 1800bbf30 | 3 | 453 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Improvements_EventStream.cpp` | 1800bc490 | 1800bc560 | 3 | 395 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Influence.cpp` | 1800bca10 | 1800bca10 | 1 | 65 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_InfluenceMap_EventStream.cpp` | 1800bcca0 | 1800bcdb0 | 3 | 451 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Influence_EventStream.cpp` | 1800bd280 | 1800bd440 | 5 | 608 | fichier:1 release:4 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Instance.cpp` | 1800bd830 | 1800be8e0 | 41 | 4907 | fichier:1 release:40 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Manager.cpp` | 1800bef40 | 1800bfb60 | 14 | 3934 | bloc:3 fichier:1 release:10 | 85 |  |
| `GameCore\XP2\Cache\Cache_Player_Religion.cpp` | 1800c0110 | 1800c0430 | 11 | 1770 | bloc:1 release:10 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Religion_EventStream.cpp` | 1800c0c90 | 1800c0e70 | 6 | 604 | fichier:1 release:5 | 32 |  |
| `GameCore\XP2\Cache\Cache_Player_Resources.cpp` | 1800c1680 | 1800c1680 | 1 | 288 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Resources_EventStream.cpp` | 1800c2290 | 1800c2420 | 5 | 564 | fichier:1 release:4 | 31 |  |
| `GameCore\XP2\Cache\Cache_Player_Stats.cpp` | 1800c2960 | 1800c2b50 | 13 | 580 | bloc:3 release:10 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Stats_EventStream.cpp` | 1800c3120 | 1800c32f0 | 6 | 623 | fichier:1 release:5 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Techs.cpp` | 1800c3790 | 1800c3790 | 1 | 1139 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Techs_EventStream.cpp` | 1800c49a0 | 1800c4bb0 | 6 | 692 | fichier:1 release:5 | 31 |  |
| `GameCore\XP2\Cache\Cache_Player_Trade.cpp` | 1800c5350 | 1800c5350 | 1 | 119 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Trade_EventStream.cpp` | 1800c5800 | 1800c58d0 | 3 | 400 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Player_Treasury.cpp` | 1800c5c90 | 1800c5f10 | 14 | 2533 | release:14 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Treasury_EventStream.cpp` | 1800c6d40 | 1800c6f70 | 6 | 574 | fichier:1 release:5 | 32 |  |
| `GameCore\XP2\Cache\Cache_Player_Units.cpp` | 1800c7c10 | 1800c8080 | 7 | 1183 | release:7 |  |  |
| `GameCore\XP2\Cache\Cache_Player_Visibility_Instance.cpp` | 1800c8950 | 1800c8b00 | 3 | 596 | bloc:1 fichier:2 | 131 |  |
| `GameCore\XP2\Cache\Cache_Player_Visibility_Manager.cpp` | 1800c90e0 | 1800c9320 | 3 | 902 | bloc:1 fichier:2 | 65 |  |
| `GameCore\XP2\Cache\Cache_Player_WMDs_EventStream.cpp` | 1800c9fa0 | 1800ca0b0 | 3 | 448 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Trade_Manager.cpp` | 1800caf90 | 1800caf90 | 1 | 828 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Unit_Ability.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_Ability_EventStream.cpp` | 1800cb730 | 1800cb7b0 | 2 | 172 | fichier:1 release:1 | 30 |  |
| `GameCore\XP2\Cache\Cache_Unit_Archaeology.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_Archaeology_EventStream.cpp` | 1800cbf30 | 1800cc000 | 3 | 401 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Unit_EventStream.cpp` | 1800cccd0 | 1800cd260 | 9 | 1529 | fichier:1 release:8 | 40 |  |
| `GameCore\XP2\Cache\Cache_Unit_Experience.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_Experience_EventStream.cpp` | 1800ce3b0 | 1800ce480 | 3 | 401 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Unit_GreatPerson.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_GreatPerson_EventStream.cpp` | 1800ceff0 | 1800cf100 | 3 | 454 | fichier:1 release:2 | 30 |  |
| `GameCore\XP2\Cache\Cache_Unit_Instance.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_Manager.cpp` | 1800d0790 | 1800d16a0 | 13 | 4310 | bloc:1 release:12 |  |  |
| `GameCore\XP2\Cache\Cache_Unit_RockBand.cpp` | 1800d22b0 | 1800d22b0 | 1 | 71 | release:1 |  |  |
| `GameCore\XP2\Cache\Cache_Unit_RockBand_EventStream.cpp` | 1800d2550 | 1800d25d0 | 2 | 172 | fichier:1 release:1 | 30 |  |
| `GameCore\XP2\Cache\Cache_Unit_Trade.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Cache\Cache_Unit_Trade_EventStream.cpp` | 1800d2bb0 | 1800d2c70 | 2 | 225 | fichier:1 release:1 | 30 |  |
| `GameCore\XP2\Cache\Cache_World_Congress.cpp` | 1800d30a0 | 1800d30a0 | 1 | 119 | fichier:1 |  | WorldCongress |
| `GameCore\XP2\Common\BarbarianTribe_Instance.cpp` | 1800d87e0 | 1800dd060 | 50 | 18467 | bloc:34 chaine:5 release:11 |  |  |
| `GameCore\XP2\Common\BarbarianTribe_Manager.cpp` | 1800e1e20 | 1800e6490 | 53 | 17560 | bloc:25 chaine:5 fichier:1 release:22 | 142 | BarbarianManager |
| `GameCore\XP2\Common\Boost_Manager.cpp` | 1800ea3d0 | 1800ee660 | 40 | 17066 | bloc:19 chaine:3 fichier:1 release:17 |  | GameQuests |
| `GameCore\XP2\Common\City_Advisor.cpp` | 1800f1960 | 1800f1e90 | 2 | 1913 | chaine:2 |  |  |
| `GameCore\XP2\Common\City_BuildQueue.cpp` | 1800f4080 | 180105f00 | 170 | 72331 | bloc:81 chaine:8 release:81 |  |  |
| `GameCore\XP2\Common\City_Buildings.cpp` | 180107c90 | 18010e8b0 | 82 | 27849 | bloc:77 chaine:2 release:3 |  |  |
| `GameCore\XP2\Common\City_Citizens.cpp` | 18010f760 | 1801111d0 | 28 | 6682 | bloc:26 chaine:1 release:1 |  |  |
| `GameCore\XP2\Common\City_Combat.cpp` | 180111590 | 180111800 | 4 | 755 | chaine:1 release:3 |  |  |
| `GameCore\XP2\Common\City_CulturalIdentity.cpp` | 1801132a0 | 1801185b0 | 63 | 22096 | bloc:54 chaine:7 release:2 |  |  |
| `GameCore\XP2\Common\City_Culture.cpp` | 18011a3e0 | 18011d590 | 42 | 12473 | bloc:21 chaine:2 release:19 |  |  |
| `GameCore\XP2\Common\City_Districts.cpp` | 18011dba0 | 18011f610 | 39 | 7321 | bloc:36 chaine:2 release:1 |  |  |
| `GameCore\XP2\Common\City_Gold.cpp` | 18011fe30 | 180120e00 | 7 | 4207 | chaine:1 release:6 |  |  |
| `GameCore\XP2\Common\City_Growth.cpp` | 180122100 | 180124c70 | 32 | 11328 | bloc:7 chaine:2 release:23 |  |  |
| `GameCore\XP2\Common\City_Instance.cpp` | 18012c2e0 | 180135a30 | 117 | 38070 | bloc:53 chaine:10 release:54 |  |  |
| `GameCore\XP2\Common\City_Manager.cpp` | 180141330 | 180142db0 | 17 | 13222 | bloc:6 chaine:2 release:9 |  |  |
| `GameCore\XP2\Common\City_Placement.cpp` | 180144c50 | 1801452e0 | 2 | 1970 | release:2 |  |  |
| `GameCore\XP2\Common\City_Power.cpp` | 180146610 | 1801472b0 | 14 | 3182 | bloc:2 chaine:3 release:9 |  |  |
| `GameCore\XP2\Common\City_Religion.cpp` | 180148a90 | 18014a090 | 20 | 6197 | bloc:3 chaine:1 release:16 |  |  |
| `GameCore\XP2\Common\City_Resources.cpp` | 18014bb30 | 18014c6c0 | 8 | 3282 | bloc:1 release:7 |  |  |
| `GameCore\XP2\Common\City_Trade.cpp` | 18014d170 | 18014e640 | 39 | 5191 | bloc:29 chaine:1 release:9 |  |  |
| `GameCore\XP2\Common\Combat_Manager.cpp` | 18014eb20 | 1801620d0 | 103 | 80598 | bloc:72 chaine:29 release:2 |  |  |
| `GameCore\XP2\Common\Combat_Results.cpp` | 180163c40 | 180163c40 | 1 | 790 | fichier:1 | 335 |  |
| `GameCore\XP2\Common\Configuration_Game.cpp` | 180164230 | 180165730 | 31 | 5284 | bloc:27 fichier:3 release:1 | 483 |  |
| `GameCore\XP2\Common\Configuration_Game_Resolver.cpp` | 180166cb0 | 180168a50 | 8 | 8407 | bloc:4 chaine:3 release:1 |  |  |
| `GameCore\XP2\Common\Configuration_Map.cpp` | 180169900 | 180169900 | 1 | 171 | fichier:1 |  |  |
| `GameCore\XP2\Common\Configuration_Player.cpp` | 18016a490 | 18016b430 | 25 | 4350 | bloc:22 chaine:2 fichier:1 |  |  |
| `GameCore\XP2\Common\Context_Achievements.cpp` | 18016e100 | 18017fcd0 | 112 | 69777 | bloc:32 chaine:79 release:1 |  |  |
| `GameCore\XP2\Common\Diplomacy_Action_RenewAlliance.cpp` | 180181720 | 1801817c0 | 2 | 260 | fichier:2 | 136 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetAllied.cpp` | 180181b30 | 180181bd0 | 2 | 260 | fichier:2 | 107 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetDelegation.cpp` | 180181c80 | 180181e60 | 4 | 842 | fichier:2 release:2 | 132 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetDenounced.cpp` | 180182020 | 1801820e0 | 2 | 297 | fichier:2 | 103 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetEmbassy.cpp` | 180182420 | 180182540 | 3 | 660 | fichier:2 release:1 | 119 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetFriend.cpp` | 180182700 | 1801827b0 | 2 | 286 | fichier:2 | 126 | StopAsking |
| `GameCore\XP2\Common\Diplomacy_Action_SetOpenBorders.cpp` | 180182c40 | 180182d60 | 3 | 620 | fichier:2 release:1 | 129 |  |
| `GameCore\XP2\Common\Diplomacy_Action_SetPromise.cpp` | 180182f40 | 180183000 | 2 | 305 | fichier:2 | 144 | PromiseType |
| `GameCore\XP2\Common\Diplomacy_Action_SetWarState.cpp` | 180183380 | 180183430 | 2 | 296 | fichier:2 | 232 | WarState |
| `GameCore\XP2\Common\Diplomacy_Action_StatementResult.cpp` | 180183a10 | 180183ab0 | 2 | 264 | fichier:2 | 102 | StatementResult |
| `GameCore\XP2\Common\Diplomacy_Manager.cpp` | 180186150 | 1801893b0 | 26 | 8000 | bloc:15 chaine:4 fichier:4 release:3 | 825 |  |
| `GameCore\XP2\Common\Diplomacy_Session.cpp` | 1801889b0 | 180188d40 | 12 | 939 | chaine:1 release:11 |  |  |
| `GameCore\XP2\Common\Diplomacy_Statement.cpp` | 180189530 | 180189530 | 1 | 85 | release:1 |  |  |
| `GameCore\XP2\Common\Diplomacy_Statement_DeclareFriend.cpp` | 180189ad0 | 180189b80 | 2 | 278 | fichier:2 | 69 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_DeclareWar.cpp` | 180189d10 | 180189d10 | 1 | 110 | fichier:1 | 120 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Defeat.cpp` | 18018a0a0 | 18018a0a0 | 1 | 110 | fichier:1 | 75 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Delegation.cpp` | 18018a1e0 | 18018a290 | 2 | 275 | fichier:2 | 63 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Denounce.cpp` | 18018a390 | 18018a430 | 2 | 270 | fichier:2 | 89 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Embassy.cpp` | 18018a5b0 | 18018a660 | 2 | 275 | fichier:2 | 64 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_FirstMeeting.cpp` | 18018a7d0 | 18018a7d0 | 1 | 125 | fichier:1 | 203 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Greeting.cpp` | 18018ad50 | 18018ade0 | 2 | 406 | chaine:1 fichier:1 | 127 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_MakeAlliance.cpp` | 18018af80 | 18018af80 | 1 | 110 | fichier:1 | 56 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_MakeDeal.cpp` | 18018b150 | 18018b150 | 1 | 131 | fichier:1 | 316 | DealAction |
| `GameCore\XP2\Common\Diplomacy_Statement_MakeDemand.cpp` | 18018b980 | 18018b980 | 1 | 110 | fichier:1 | 32 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_MakePeace.cpp` | 18018ba10 | 18018bac0 | 2 | 279 | fichier:2 | 66 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_OpenBorders.cpp` | 18018bbc0 | 18018bc70 | 2 | 275 | fichier:2 | 63 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_ProposeAction.cpp` | 18018bf80 | 18018c150 | 2 | 714 | fichier:1 release:1 | 167 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_RenewAlliance.cpp` | 18018c250 | 18018c300 | 2 | 275 | fichier:2 | 69 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Response.cpp` | 18018c400 | 18018c400 | 1 | 110 | fichier:1 | 58 |  |
| `GameCore\XP2\Common\Diplomacy_Statement_Warning.cpp` | 18018c490 | 18018c6b0 | 4 | 1634 | fichier:2 release:2 | 248 |  |
| `GameCore\XP2\Common\District_Instance.cpp` | 18018f520 | 180193000 | 59 | 14797 | bloc:44 chaine:4 release:11 |  |  |
| `GameCore\XP2\Common\Emergency_Manager.cpp` | 180195e70 | 18019ae30 | 47 | 20852 | bloc:20 chaine:3 fichier:1 release:23 | 220 | EmergencyManager |
| `GameCore\XP2\Common\Formation_Instance.cpp` | 1801a3080 | 1801a4ba0 | 36 | 7830 | bloc:3 release:33 |  |  |
| `GameCore\XP2\Common\Formation_Movement.cpp` | 1801a6830 | 1801aa680 | 30 | 16053 | bloc:4 fichier:1 release:25 | 211 |  |
| `GameCore\XP2\Common\FreeCities_Manager.cpp` | 1801ad250 | 1801ad430 | 5 | 465 | fichier:1 release:4 | 70 | FreeCitiesManager |
| `GameCore\XP2\Common\Game_Archaeology.cpp` | 1801b3400 | 1801b4c70 | 22 | 6366 | bloc:14 chaine:5 fichier:1 release:2 | 127 | GameArchaeology |
| `GameCore\XP2\Common\Game_Calendar.cpp` | 1801b6290 | 1801b66c0 | 4 | 1809 | chaine:1 release:3 |  |  |
| `GameCore\XP2\Common\Game_Climate.cpp` | 1801b73a0 | 1801c2aa0 | 116 | 50120 | bloc:89 chaine:19 fichier:1 release:7 | 1571 | GameClimate |
| `GameCore\XP2\Common\Game_Culture.cpp` | 1801c50e0 | 1801c7da0 | 26 | 11330 | bloc:21 chaine:4 fichier:1 | 164 | Culture |
| `GameCore\XP2\Common\Game_Diplomacy.cpp` | 1801cdc40 | 1801cf680 | 22 | 9545 | bloc:16 chaine:3 fichier:1 release:2 | 258 | DiplomaticRelations |
| `GameCore\XP2\Common\Game_Eras.cpp` | 1801d8520 | 1801e8d80 | 183 | 66763 | bloc:160 chaine:17 fichier:1 release:5 | 452 | GameEras |
| `GameCore\XP2\Common\Game_FalloutManager.cpp` | 1801ed150 | 1801ed8b0 | 11 | 1888 | bloc:7 chaine:1 fichier:1 release:2 | 135 | FalloutManager |
| `GameCore\XP2\Common\Game_GossipManager.cpp` | 1801f5200 | 1801f6370 | 17 | 5206 | bloc:12 chaine:2 fichier:1 release:2 | 126 | GossipManager |
| `GameCore\XP2\Common\Game_Gossip_OccurrenceHandlers.cpp` | 1801f8110 | 1801f8be0 | 14 | 2765 | bloc:2 release:12 |  |  |
| `GameCore\XP2\Common\Game_GreatPeople.cpp` | 180208770 | 18020e3e0 | 69 | 23713 | bloc:50 chaine:14 fichier:1 release:4 | 184 | GreatPeople |
| `GameCore\XP2\Common\Game_History_Manager.cpp` | 1802120b0 | 1802132f0 | 19 | 5387 | bloc:10 chaine:2 fichier:1 release:6 | 67 | HistoryManager |
| `GameCore\XP2\Common\Game_History_Moment.cpp` | 1802149b0 | 180215ec0 | 9 | 5390 | bloc:2 chaine:1 release:6 |  |  |
| `GameCore\XP2\Common\Game_History_MomentHandlers.cpp` | 1802160c0 | 180221db0 | 155 | 47508 | bloc:59 chaine:5 release:91 |  |  |
| `GameCore\XP2\Common\Game_Initializer.cpp` | 180227d90 | 18022d060 | 25 | 21739 | bloc:12 chaine:13 |  |  |
| `GameCore\XP2\Common\Game_NameManager.cpp` | 180230af0 | 180230c40 | 4 | 373 | bloc:1 fichier:1 release:2 | 89 | NameManager |
| `GameCore\XP2\Common\Game_NationalParks.cpp` | 180235200 | 1802357e0 | 8 | 1510 | bloc:5 chaine:1 fichier:1 release:1 | 132 | GameNationalParks |
| `GameCore\XP2\Common\Game_RandomEvents.cpp` | 18023a950 | 1802400d0 | 56 | 22253 | bloc:30 chaine:8 fichier:1 release:17 | 190 | RandomEvents |
| `GameCore\XP2\Common\Game_Religion.cpp` | 180241d70 | 180242ed0 | 18 | 5095 | bloc:9 chaine:3 fichier:1 release:5 | 216 | GameReligion |
| `GameCore\XP2\Common\Game_Stats.cpp` | 180245b30 | 180245f00 | 6 | 1007 | fichier:1 release:5 | 69 | GameStats |
| `GameCore\XP2\Common\Game_Summary.cpp` | 1802490d0 | 18024bbf0 | 27 | 14953 | bloc:9 chaine:2 release:16 |  |  |
| `GameCore\XP2\Common\Game_Techs.cpp` | 18024fec0 | 180250d40 | 9 | 3744 | chaine:1 fichier:1 release:7 | 92 | Techs |
| `GameCore\XP2\Common\Game_VictoryManager.cpp` | 180253770 | 180256bc0 | 20 | 13652 | bloc:2 chaine:3 release:15 |  |  |
| `GameCore\XP2\Common\Governor_Instance.cpp` | 180258510 | 1802597b0 | 16 | 4726 | bloc:1 release:15 |  |  |
| `GameCore\XP2\Common\Improvement_Builder.cpp` | 180259ff0 | 18025d460 | 17 | 13962 | chaine:1 release:16 |  |  |
| `GameCore\XP2\Common\Improvement_Yields.cpp` | 180259f60 | 180260720 | 15 | 9851 | bloc:4 chaine:3 release:8 |  |  |
| `GameCore\XP2\Common\Player_Bonuses.cpp` | 180260db0 | 180261060 | 5 | 717 | fichier:1 release:4 | 72 | Bonuses |
| `GameCore\XP2\Common\Player_Cities.cpp` | 180262fb0 | 180266470 | 53 | 13372 | bloc:5 release:48 |  |  |
| `GameCore\XP2\Common\Player_Congress.cpp` | 18026bb80 | 18026c5d0 | 10 | 2629 | bloc:6 chaine:1 fichier:1 release:2 | 122 | Congress |
| `GameCore\XP2\Common\Player_CulturalIdentity.cpp` | 18026eba0 | 18026ef60 | 3 | 981 | release:3 |  |  |
| `GameCore\XP2\Common\Player_Culture.cpp` | 180277390 | 180286040 | 184 | 59448 | bloc:157 chaine:19 release:8 |  |  |
| `GameCore\XP2\Common\Player_Diplomacy.cpp` | 180291b50 | 1802b0520 | 322 | 122999 | bloc:266 chaine:34 release:22 |  |  |
| `GameCore\XP2\Common\Player_DiplomacyBlackboard.cpp` | 1802bbd00 | 1802bc5c0 | 6 | 2483 | release:6 |  |  |
| `GameCore\XP2\Common\Player_Districts.cpp` | 1802c3dd0 | 1802c4f10 | 28 | 5034 | bloc:21 chaine:2 release:5 |  |  |
| `GameCore\XP2\Common\Player_Eras.cpp` | 1802c9480 | 1802c99d0 | 10 | 1589 | bloc:2 release:8 |  |  |
| `GameCore\XP2\Common\Player_Espionage.cpp` | 1802cde10 | 1802d2530 | 59 | 18047 | bloc:47 chaine:5 release:7 |  |  |
| `GameCore\XP2\Common\Player_EventManager.cpp` | 1802d6c40 | 1802d80b0 | 14 | 5205 | release:14 |  |  |
| `GameCore\XP2\Common\Player_Formations.cpp` | 1802d9070 | 1802d9070 | 1 | 107 | release:1 |  |  |
| `GameCore\XP2\Common\Player_Goody_Hut.cpp` | 1802da010 | 1802dad40 | 5 | 4336 | bloc:1 chaine:2 release:2 |  |  |
| `GameCore\XP2\Common\Player_Governors.cpp` | 1802dc7e0 | 1802df800 | 53 | 12867 | bloc:38 chaine:8 fichier:1 release:6 | 127 | Governors |
| `GameCore\XP2\Common\Player_GreatPeoplePoints.cpp` | 1802e1ae0 | 1802e46a0 | 42 | 11195 | bloc:11 chaine:2 release:29 |  |  |
| `GameCore\XP2\Common\Player_Improvements.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Player_Influence.cpp` | 1802e84a0 | 1802edf10 | 91 | 24504 | bloc:59 chaine:5 release:27 |  |  |
| `GameCore\XP2\Common\Player_Initializer.cpp` | 1802ef560 | 1802efdb0 | 3 | 2732 | chaine:1 release:2 |  |  |
| `GameCore\XP2\Common\Player_Instance.cpp` | 1802f1510 | 1802fa9a0 | 111 | 37428 | bloc:79 fichier:2 release:30 | 347 |  |
| `GameCore\XP2\Common\Player_Instance_Serialization.cpp` | 1803037b0 | 1803037b0 | 1 | 5363 | release:1 |  |  |
| `GameCore\XP2\Common\Player_Manager.cpp` | 1803059b0 | 180306b90 | 13 | 4478 | bloc:2 fichier:1 release:10 | 174 |  |
| `GameCore\XP2\Common\Player_NameManager.cpp` | 1803091f0 | 180309640 | 6 | 2114 | chaine:1 release:5 |  |  |
| `GameCore\XP2\Common\Player_Operations_Manager.cpp` | 18030c100 | 18030edf0 | 13 | 12039 | bloc:2 chaine:1 release:10 |  |  |
| `GameCore\XP2\Common\Player_Quests.cpp` | 18030f740 | 18030f8f0 | 5 | 486 | bloc:3 release:2 |  |  |
| `GameCore\XP2\Common\Player_Religion.cpp` | 180312270 | 1803165b0 | 65 | 17669 | bloc:57 chaine:6 release:2 |  |  |
| `GameCore\XP2\Common\Player_Resources.cpp` | 18031cd20 | 180323220 | 82 | 27158 | bloc:77 chaine:4 release:1 |  |  |
| `GameCore\XP2\Common\Player_Stats.cpp` | 180329020 | 18032f180 | 89 | 24079 | bloc:11 chaine:1 release:77 |  |  |
| `GameCore\XP2\Common\Player_Techs.cpp` | 180332700 | 180336940 | 54 | 17530 | bloc:44 chaine:6 release:4 |  |  |
| `GameCore\XP2\Common\Player_Trade.cpp` | 18033a490 | 18033e250 | 80 | 15683 | bloc:42 chaine:1 release:37 |  |  |
| `GameCore\XP2\Common\Player_Treasury.cpp` | 180340bc0 | 180343360 | 19 | 10336 | bloc:2 chaine:7 release:10 |  |  |
| `GameCore\XP2\Common\Player_Units.cpp` | 18034a1a0 | 1803512d0 | 75 | 28534 | bloc:59 chaine:8 release:8 |  |  |
| `GameCore\XP2\Common\Player_Visibility_Instance.cpp` | 180356a40 | 180357e40 | 23 | 5114 | bloc:2 fichier:1 release:20 | 206 |  |
| `GameCore\XP2\Common\Player_Visibility_Manager.cpp` | 180359d20 | 18035dff0 | 41 | 18710 | bloc:21 chaine:2 fichier:3 release:15 | 231 |  |
| `GameCore\XP2\Common\Player_WMDs.cpp` | 18035f010 | 18035f330 | 7 | 857 | release:7 |  |  |
| `GameCore\XP2\Common\Route_Builder.cpp` | 180360060 | 1803619b0 | 19 | 6600 | bloc:1 release:18 |  |  |
| `GameCore\XP2\Common\Rules_Appeal.cpp` | 180361d80 | 180361e30 | 3 | 681 | bloc:1 fichier:1 release:1 | 42 | BoostManager |
| `GameCore\XP2\Common\Rules_Buildings.cpp` | 1803624f0 | 1803624f0 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Cities.cpp` | 180363180 | 180365ad0 | 25 | 10977 | bloc:18 chaine:4 fichier:1 release:2 | 59 |  |
| `GameCore\XP2\Common\Rules_Civics.cpp` | 180366390 | 180366390 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Civilizations.cpp` | 180366470 | 180366470 | 1 | 157 | fichier:1 | 22 |  |
| `GameCore\XP2\Common\Rules_Combat.cpp` | 180367320 | 18036c890 | 48 | 22166 | bloc:9 chaine:17 fichier:1 release:21 | 68 |  |
| `GameCore\XP2\Common\Rules_Diplomacy.cpp` | 18036e9d0 | 18036e9d0 | 1 | 157 | fichier:1 | 33 |  |
| `GameCore\XP2\Common\Rules_Districts.cpp` | 18036ef60 | 18036ef60 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Economic.cpp` | 18036f140 | 1803715b0 | 31 | 9092 | bloc:3 chaine:1 fichier:1 release:26 | 58 |  |
| `GameCore\XP2\Common\Rules_Era.cpp` | 1803716e0 | 1803716e0 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Espionage.cpp` | 180371800 | 1803755c0 | 37 | 16345 | bloc:24 chaine:12 fichier:1 | 51 |  |
| `GameCore\XP2\Common\Rules_Governments.cpp` | 180375960 | 180375960 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Improvement.cpp` | 180375be0 | 180375f20 | 4 | 1783 | fichier:1 release:3 | 55 |  |
| `GameCore\XP2\Common\Rules_Movement.cpp` | 180377c60 | 180379000 | 14 | 5060 | bloc:1 fichier:1 release:12 | 48 |  |
| `GameCore\XP2\Common\Rules_Placement.cpp` | 1803798c0 | 18037a210 | 8 | 4609 | fichier:1 release:7 | 55 |  |
| `GameCore\XP2\Common\Rules_Player.cpp` | 18037b7b0 | 18037bf50 | 7 | 1993 | bloc:3 chaine:1 fichier:1 release:2 | 55 |  |
| `GameCore\XP2\Common\Rules_Religions.cpp` | 18037c020 | 18037c020 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Techs.cpp` | 18037c100 | 18037c100 | 1 | 157 | fichier:1 | 25 |  |
| `GameCore\XP2\Common\Rules_Tourism.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Rules_Units.cpp` | 18037db30 | 18037dd10 | 5 | 1001 | bloc:2 fichier:1 release:2 | 53 |  |
| `GameCore\XP2\Common\Rules_Yield.cpp` | 18037eb00 | 18037ec60 | 4 | 3588 | bloc:1 fichier:1 release:2 | 61 |  |
| `GameCore\XP2\Common\Trade_Graph.cpp` | 180382c60 | 180388cf0 | 85 | 26003 | bloc:80 chaine:3 fichier:1 release:1 | 999 |  |
| `GameCore\XP2\Common\Trade_Manager.cpp` | 18038a730 | 180391f10 | 85 | 30281 | bloc:45 chaine:8 fichier:1 release:31 | 97 | TradeManager |
| `GameCore\XP2\Common\Trade_Movement.cpp` | 1803928b0 | 180393470 | 10 | 3381 | fichier:1 release:9 | 106 |  |
| `GameCore\XP2\Common\Trade_Route.cpp` | 180393df0 | 180393df0 | 1 | 914 | release:1 |  |  |
| `GameCore\XP2\Common\Unit_Archaeology.cpp` | 180394890 | 180394890 | 1 | 450 | release:1 |  |  |
| `GameCore\XP2\Common\Unit_Espionage.cpp` | 180395500 | 1803960c0 | 13 | 3246 | bloc:3 chaine:1 release:9 |  |  |
| `GameCore\XP2\Common\Unit_Experience.cpp` | 1803964e0 | 1803969a0 | 5 | 1533 | release:5 |  |  |
| `GameCore\XP2\Common\Unit_GreatPerson.cpp` | 1803977a0 | 180399650 | 23 | 7786 | bloc:3 chaine:1 release:19 |  |  |
| `GameCore\XP2\Common\Unit_Hero.cpp` | 1803998c0 | 180399f50 | 7 | 1663 | bloc:2 chaine:2 release:3 |  |  |
| `GameCore\XP2\Common\Unit_Instance.cpp` | 18039d6a0 | 1803afbb0 | 237 | 72856 | bloc:218 chaine:11 release:8 |  |  |
| `GameCore\XP2\Common\Unit_Placement.cpp` | 1803affd0 | 1803b11b0 | 11 | 8304 | release:11 |  |  |
| `GameCore\XP2\Common\Unit_Religion.cpp` | 1803b2230 | 1803b2230 | 1 | 1162 | release:1 |  |  |
| `GameCore\XP2\Common\Unit_RockBand.cpp` | 1803b2d60 | 1803b3a80 | 9 | 3305 | release:9 |  |  |
| `GameCore\XP2\Common\Unit_Sight.cpp` | 1803b4490 | 1803b5c00 | 4 | 6146 | release:4 |  |  |
| `GameCore\XP2\Common\Unit_Trade.cpp` | 1803b7450 | 1803b7760 | 9 | 768 | bloc:4 chaine:1 release:4 |  |  |
| `GameCore\XP2\Common\Unit_Upgrade_Manager.cpp` | 1803b7870 | 1803b7c80 | 2 | 1141 | chaine:1 fichier:1 | 42 | UnitUpgrade |
| `GameCore\XP2\Common\World_Congress.cpp` | 1803bad80 | 1803c0d40 | 63 | 24385 | bloc:30 chaine:5 fichier:1 release:27 | 205 | WorldCongress |
| `GameCore\XP2\Common\World_Congress_Resolutions.cpp` | 1803c5ab0 | 1803cf120 | 135 | 39144 | bloc:129 chaine:2 fichier:1 release:3 | 226 |  |
| `GameCore\XP2\Common\Yield_Monitor.cpp` | 1803d2c20 | 1803d32d0 | 9 | 2148 | release:9 |  |  |
| `GameCore\XP2\Common\AI\BoostSupport.cpp` | 1803d6080 | 1803d6740 | 3 | 3538 | chaine:1 release:2 |  |  |
| `GameCore\XP2\Common\AI\CityAIManager.cpp` | 1803d8fc0 | 1803d9580 | 7 | 1647 | bloc:1 release:6 |  |  |
| `GameCore\XP2\Common\AI\CityBuild.cpp` | 1803df040 | 1803fa8e0 | 71 | 48866 | bloc:52 chaine:9 release:10 |  |  |
| `GameCore\XP2\Common\AI\CivKnowledge.cpp` | 1803f15c0 | 1803f5f70 | 51 | 18806 | bloc:16 chaine:5 release:30 |  |  |
| `GameCore\XP2\Common\AI\Contracts.cpp` | 1803f8f90 | 1803fa2a0 | 20 | 5569 | bloc:1 chaine:2 release:17 |  |  |
| `GameCore\XP2\Common\AI\Control_Contract.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\Control_Factory.cpp` | 1803fea90 | 1803fea90 | 1 | 434 | fichier:1 | 114 |  |
| `GameCore\XP2\Common\AI\Control_Instance.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\Control_Manager.cpp` | 180400930 | 180403970 | 25 | 12523 | bloc:4 release:21 |  |  |
| `GameCore\XP2\Common\AI\CultureAi.cpp` | 180412840 | 180419a10 | 48 | 31203 | bloc:29 chaine:7 release:12 |  |  |
| `GameCore\XP2\Common\AI\Culture_GreatWorks.cpp` | 18041fb80 | 180421910 | 16 | 10262 | bloc:6 release:10 |  |  |
| `GameCore\XP2\Common\AI\Diplomatic.cpp` | 18042ba80 | 180434ed0 | 55 | 40648 | bloc:34 chaine:7 release:14 |  |  |
| `GameCore\XP2\Common\AI\DiplomaticActions.cpp` | 18043fc00 | 180448780 | 96 | 35691 | bloc:16 chaine:1 release:79 |  |  |
| `GameCore\XP2\Common\AI\DiplomaticState.cpp` | 18044cd70 | 18044d140 | 3 | 1130 | release:3 |  |  |
| `GameCore\XP2\Common\AI\DiplomaticSupport.cpp` | 18044e2b0 | 1804529c0 | 24 | 18767 | bloc:9 chaine:2 lua:13 |  | CanOfferAlliance, GoToWar, GreatWorkObjects, GrievancesType, HasMaxInfluence, MakePeace (+7) |
| `GameCore\XP2\Common\AI\Economic.cpp` | 180456580 | 180459320 | 26 | 12374 | bloc:3 release:23 |  |  |
| `GameCore\XP2\Common\AI\Espionage.cpp` | 18045bba0 | 18045f670 | 19 | 15066 | bloc:14 chaine:3 release:2 |  |  |
| `GameCore\XP2\Common\AI\EspionageManager.cpp` | 180461140 | 180461700 | 6 | 2307 | bloc:3 fichier:3 | 157 |  |
| `GameCore\XP2\Common\AI\GameEffectAnalyzers.cpp` | 1804631b0 | 180487010 | 387 | 143542 | bloc:185 fichier:198 release:4 | 8921 |  |
| `GameCore\XP2\Common\AI\GameEffects.cpp` | 18048e9e0 | 18048e9e0 | 1 | 303 | release:1 |  |  |
| `GameCore\XP2\Common\AI\GameRequirementAnalyzers.cpp` | 18048f420 | 180494030 | 100 | 18169 | bloc:40 fichier:60 | 1838 |  |
| `GameCore\XP2\Common\AI\GlobalAi.cpp` | 18049c460 | 1804a1750 | 36 | 21133 | bloc:9 chaine:11 fichier:1 release:15 | 535 |  |
| `GameCore\XP2\Common\AI\Governors.cpp` | 1804a7380 | 1804a9110 | 18 | 7545 | bloc:8 chaine:2 fichier:1 release:7 | 390 | AIGovernors |
| `GameCore\XP2\Common\AI\GrandStrategic.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\Homeland.cpp` | 1804ba7c0 | 1804c3490 | 39 | 36686 | bloc:12 chaine:10 release:17 |  |  |
| `GameCore\XP2\Common\AI\InfluenceMap.cpp` | 1804c59b0 | 1804c59b0 | 1 | 1287 | release:1 |  |  |
| `GameCore\XP2\Common\AI\Military.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\Operation.cpp` | 1804da650 | 1804e4450 | 74 | 41603 | bloc:61 chaine:10 release:3 |  |  |
| `GameCore\XP2\Common\AI\OperationEvaluator.cpp` | 1804e6130 | 1804e6130 | 1 | 1681 | release:1 |  |  |
| `GameCore\XP2\Common\AI\Religious.cpp` | 1804ec000 | 1804ee820 | 17 | 11002 | bloc:9 chaine:4 release:4 |  |  |
| `GameCore\XP2\Common\AI\ReligiousManager.cpp` | 1804f21a0 | 1804f27b0 | 7 | 2348 | bloc:4 fichier:3 | 171 |  |
| `GameCore\XP2\Common\AI\Research.cpp` | 1804f7b80 | 1804fa7e0 | 29 | 11585 | bloc:13 chaine:4 release:12 |  |  |
| `GameCore\XP2\Common\AI\SiteEvaluator.cpp` | 1804ffaa0 | 1805001d0 | 4 | 2335 | chaine:1 release:3 |  |  |
| `GameCore\XP2\Common\AI\Tactical.cpp` | 1805022e0 | 18050e440 | 63 | 50524 | bloc:25 chaine:21 release:17 |  |  |
| `GameCore\XP2\Common\AI\Targets.cpp` | 1805114f0 | 1805175d0 | 53 | 25744 | bloc:41 chaine:2 release:10 |  |  |
| `GameCore\XP2\Common\AI\UnitPlanning_Manager.cpp` | 18051b7e0 | 18051da80 | 26 | 9250 | bloc:18 chaine:4 release:4 |  |  |
| `GameCore\XP2\Common\AI\UnitProduction.cpp` | 180522740 | 1805253e0 | 11 | 11750 | bloc:9 chaine:1 fichier:1 | 169 |  |
| `GameCore\XP2\Common\AI\Utilities_ListManagement.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\Value_Manager.cpp` | 180528140 | 180532f00 | 35 | 44849 | bloc:3 release:32 |  |  |
| `GameCore\XP2\Common\AI\VictoryStrategy.cpp` | 180536b00 | 180538f20 | 22 | 9025 | bloc:6 chaine:3 release:13 |  |  |
| `GameCore\XP2\Common\Analysis\Analysis_Civic.cpp` | 180539d10 | 18053a5a0 | 8 | 2594 | bloc:3 release:5 |  |  |
| `GameCore\XP2\Common\Analysis\Analysis_Economy.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Analysis\Analysis_Manager.cpp` | 18053af60 | 18053b3d0 | 2 | 1367 | chaine:1 release:1 |  |  |
| `GameCore\XP2\Common\Analysis\Analysis_Technology.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\BehaviorTree\BarbarianManagement.cpp` | 18053d6b0 | 18053d6b0 | 1 | 1290 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\BehaviorTree.cpp` | 18053ffe0 | 180544360 | 22 | 17571 | bloc:5 chaine:1 release:16 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\Blackboard.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\BehaviorTree\BtBuildImprovement.cpp` | 1805526c0 | 1805526c0 | 1 | 796 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\BtFindAvailableUnit.cpp` | 1805554e0 | 180555b60 | 2 | 1976 | chaine:1 release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\BtMoveUnit.cpp` | 180556710 | 180558b50 | 13 | 9999 | chaine:3 release:10 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\BtNotDecorator.cpp` | 180559670 | 180559670 | 1 | 1414 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\BtRunBehaviorTree.cpp` | 18055a680 | 18055a680 | 1 | 4308 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\ChooseDistrict.cpp` | 18055cbc0 | 18055cbc0 | 1 | 1134 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\ClearPlots.cpp` | 18055e160 | 18055ed80 | 3 | 3477 | release:3 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\Factory.cpp` | 1805601f0 | 180564190 | 50 | 16971 | bloc:45 chaine:1 fichier:3 release:1 | 382 | BehaviorTree, BehaviorTreeFactory |
| `GameCore\XP2\Common\AI\BehaviorTree\Manager.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\AI\BehaviorTree\NavalSupport.cpp` | 180566a50 | 180566a50 | 1 | 629 | release:1 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\NodeDataStructs.cpp` | 180567520 | 18056a320 | 13 | 16010 | bloc:10 chaine:1 release:2 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\OperationAttack.cpp` | 18056c360 | 18056ef70 | 6 | 11720 | chaine:3 release:3 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\OperationCityAttack.cpp` | 18056fa60 | 1805732d0 | 13 | 14511 | bloc:2 chaine:5 release:6 |  |  |
| `GameCore\XP2\Common\AI\BehaviorTree\OperationSupport.cpp` | 1805760d0 | 18057bd20 | 19 | 24363 | bloc:3 chaine:10 release:6 |  |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Instance.cpp` | 18057e0f0 | 18057efc0 | 18 | 3886 | bloc:2 chaine:1 release:15 |  |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Agreements.cpp` | 180580260 | 180582780 | 22 | 9379 | bloc:16 chaine:2 fichier:1 release:3 | 484 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Captive.cpp` | 180582990 | 180583180 | 8 | 2300 | bloc:3 chaine:2 fichier:1 release:2 | 180 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Cities.cpp` | 180583510 | 1805848e0 | 16 | 5830 | bloc:10 chaine:4 fichier:1 release:1 | 242 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Favor.cpp` | 180584d00 | 180585240 | 13 | 1425 | bloc:5 chaine:1 fichier:1 release:6 | 114 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Gold.cpp` | 180585350 | 180585a40 | 12 | 1711 | bloc:6 chaine:3 fichier:1 release:2 | 144 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_GreatWork.cpp` | 180585b20 | 180586860 | 11 | 3355 | bloc:5 chaine:2 fichier:2 release:2 | 226 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Instance.cpp` | 180586c00 | 180587140 | 10 | 2030 | bloc:2 chaine:2 release:6 |  |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Item_Resources.cpp` | 1805879b0 | 180588a40 | 19 | 4111 | bloc:9 chaine:2 fichier:1 release:7 | 259 |  |
| `GameCore\XP2\Common\DiplomacyDeals\Deal_Manager.cpp` | 180589590 | 18058bfb0 | 33 | 11463 | bloc:26 fichier:2 release:5 | 941 |  |
| `GameCore\XP2\Common\Exports\Export_Context.cpp` | 18058fb40 | 180590570 | 29 | 2386 | bloc:2 fichier:1 release:26 | 107 |  |
| `GameCore\XP2\Common\Exports\Export_Game.cpp` | 180596e20 | 1805976f0 | 15 | 2437 | release:15 |  |  |
| `GameCore\XP2\Common\Exports\Export_GameAsynch.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Exports\Export_GameGreatPeople.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Exports\Export_GameReligion.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\Exports\Export_InfluenceMap.cpp` | 18059a6f0 | 18059a6f0 | 1 | 127 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_Player.cpp` | 18059c8c0 | 18059d500 | 29 | 2825 | bloc:4 release:25 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerCulture.cpp` | 18059e160 | 18059e160 | 1 | 174 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerGreatPeoplePoints.cpp` | 18059f1d0 | 18059f210 | 2 | 168 | release:2 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerTechs.cpp` | 18059fb90 | 18059fb90 | 1 | 173 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerTreasury.cpp` | 1805a00b0 | 1805a00b0 | 1 | 131 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerVisibility.cpp` | 1805a0590 | 1805a0590 | 1 | 98 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_PlayerVisibilityManager.cpp` | 1805a0a50 | 1805a0a50 | 1 | 86 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_Unit.cpp` | 1805a2580 | 1805a2580 | 1 | 95 | release:1 |  |  |
| `GameCore\XP2\Common\Exports\Export_UnitExperience.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_BaseQuest.cpp` | 1805a3c70 | 1805a3c70 | 1 | 548 | chaine:1 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_ClearBarbarianCamp.cpp` | 1805a42f0 | 1805a4540 | 4 | 844 | release:4 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_ConvertCapitalToReligion.cpp` | 1805a4810 | 1805a49e0 | 4 | 481 | release:4 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_RecruitGreatPersonClass.cpp` | 1805a4cc0 | 1805a51a0 | 7 | 1408 | chaine:1 release:6 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_SendTradeRoute.cpp` | 1805a5530 | 1805a5530 | 1 | 156 | release:1 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_TrainUnitType.cpp` | 1805a5890 | 1805a5eb0 | 9 | 1730 | chaine:1 release:8 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_TriggerCivicBoost.cpp` | 1805a6440 | 1805a68f0 | 8 | 1382 | chaine:1 release:7 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_TriggerTechBoost.cpp` | 1805a6d90 | 1805a71c0 | 7 | 1247 | chaine:1 release:6 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_Logic_ZoneDistrictType.cpp` | 1805a7640 | 1805a7aa0 | 7 | 1299 | chaine:1 release:6 |  |  |
| `GameCore\XP2\Common\GameQuests\Quests_QuestData.cpp` | 1805a7d70 | 1805a7d70 | 1 | 520 | release:1 |  |  |
| `GameCore\XP2\Common\Lua\Lua_GameSignals.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\MapGen\Area_Builder.cpp` | 1805bf220 | 1805bf650 | 4 | 1592 | release:4 |  |  |
| `GameCore\XP2\Common\MapGen\Landmass_Builder.cpp` | 1805bf890 | 1805bf890 | 1 | 112 | release:1 |  |  |
| `GameCore\XP2\Common\MapGen\MapAnalysis.cpp` | 1805c4060 | 1805cbc60 | 39 | 31717 | bloc:13 chaine:7 release:19 |  |  |
| `GameCore\XP2\Common\MapGen\Map_Generator.cpp` | 1805cdbb0 | 1805cde50 | 5 | 844 | bloc:1 chaine:3 release:1 |  |  |
| `GameCore\XP2\Common\MapGen\Region_Builder.cpp` | 1805cf5a0 | 1805d1e00 | 22 | 11719 | bloc:6 release:16 |  |  |
| `GameCore\XP2\Common\MapGen\Resource_Builder.cpp` | 1805d24d0 | 1805d24d0 | 1 | 562 | release:1 |  |  |
| `GameCore\XP2\Common\MapGen\Start_Positioner.cpp` | 1805d53b0 | 1805d8150 | 41 | 11958 | bloc:6 release:35 |  |  |
| `GameCore\XP2\Common\MapGen\Terrain_Builder.cpp` | 1805d8de0 | 1805dd0f0 | 44 | 19632 | chaine:2 release:42 |  |  |
| `GameCore\XP2\Common\Notifications\Notification_AcknowledgeEmergency.cpp` | 1805ddc80 | 1805ddc80 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_CommemorationAvailable.cpp` | 1805ddde0 | 1805ddde0 | 1 | 98 | fichier:1 | 65 |  |
| `GameCore\XP2\Common\Notifications\Notification_CongressInSession.cpp` | 1805ddf50 | 1805ddf50 | 1 | 98 | fichier:1 | 38 |  |
| `GameCore\XP2\Common\Notifications\Notification_CongressResults.cpp` | 1805de0c0 | 1805de0c0 | 1 | 98 | fichier:1 | 35 |  |
| `GameCore\XP2\Common\Notifications\Notification_ConsiderCongress.cpp` | 1805de220 | 1805de220 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_ConsiderDisloyalCity.cpp` | 1805de400 | 1805de400 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_DedicationQuest.cpp` | 1805de5a0 | 1805de5a0 | 1 | 98 | fichier:1 | 56 |  |
| `GameCore\XP2\Common\Notifications\Notification_EmergencyHappenings.cpp` | 1805de7c0 | 1805de7c0 | 1 | 98 | fichier:1 | 56 |  |
| `GameCore\XP2\Common\Notifications\Notification_ForeignCityBecameFreeCity.cpp` | 1805de8f0 | 1805de8f0 | 1 | 98 | fichier:1 | 72 |  |
| `GameCore\XP2\Common\Notifications\Notification_GovernorAppointment.cpp` | 1805dea50 | 1805dea50 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_GovernorIdle.cpp` | 1805deb70 | 1805deb70 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_GovernorOpportunity.cpp` | 1805dece0 | 1805dece0 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_GovernorPromotion.cpp` | 1805dee60 | 1805dee60 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\XP2\Common\Notifications\Notification_PrideMomentRecorded.cpp` | 1805def90 | 1805def90 | 1 | 98 | fichier:1 | 60 |  |
| `GameCore\XP2\Common\UnitCommands\Unit_Commands_BuildingProduction.cpp` | 1805df200 | 1805df3b0 | 2 | 653 | fichier:1 release:1 |  |  |
| `GameCore\XP2\Common\UnitCommands\Unit_Commands_Jump.cpp` | 1805dfb40 | 1805dfc10 | 2 | 437 | fichier:2 | 235 |  |
| `GameCore\XP2\Common\UnitCommands\Unit_Commands_Paradrop.cpp` | 1805e0680 | 1805e07e0 | 3 | 566 | bloc:1 fichier:2 | 326 |  |
| `GameCore\XP2\Common\UnitCommands\Unit_Commands_PriorityTarget.cpp` | 1805e11b0 | 1805e1280 | 2 | 437 | fichier:2 | 185 |  |
| `GameCore\XP2\Common\UnitCommands\Unit_Commands_ProjectProduction.cpp` | 1805e1500 | 1805e1670 | 2 | 601 | fichier:1 release:1 |  |  |
| `GameCore\XP2\Common\UnitCommands\XP2_Unit_Commands_CondemnHeretic.cpp` | 1805e1990 | 1805e1e10 | 3 | 2040 | chaine:1 fichier:1 release:1 |  |  |
| `GameCore\XP2\Common\UnitCommands\XP2_Unit_Commands_DistrictProduction.cpp` | 1805e2280 | 1805e2440 | 2 | 696 | fichier:1 release:1 |  |  |
| `GameCore\XP2\Common\UnitCommands\XP2_Unit_Commands_NameUnit.cpp` | 1805e28a0 | 1805e28a0 | 1 | 255 | fichier:1 |  |  |
| `GameCore\XP2\Common\UnitCommands\XP2_Unit_Commands_PlunderTradeRoute.cpp` | 1805e29a0 | 1805e2df0 | 3 | 1635 | fichier:1 release:2 |  |  |
| `GameCore\Shared\DatabaseLoader.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\DatabaseLogger.cpp` | 1805e3590 | 1805e3590 | 1 | 190 | chaine:1 |  |  |
| `GameCore\Shared\Fractal_Instance.cpp` | 1805e3910 | 1805e3a80 | 3 | 2560 | chaine:1 release:2 |  |  |
| `GameCore\Shared\GameSummary_DataProviders.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\GameSummary_NewRepository.cpp` | 1805e6fd0 | 1805e88c0 | 7 | 6900 | release:7 |  |  |
| `GameCore\Shared\GameSummary_Repository.cpp` | 1805eaaf0 | 1805ec8c0 | 16 | 7561 | bloc:6 release:10 |  |  |
| `GameCore\Shared\Lua_IWorldBuilderCityManager.cpp` | 1805ef630 | 1805ef630 | 1 | 372 | lua:1 |  | GetPlotOwner |
| `GameCore\Shared\Lua_IWorldBuilderMapManager.cpp` | 1805f2c40 | 1805f4420 | 15 | 11527 | bloc:11 chaine:3 release:1 |  | CanPlaceFeature, CanPlaceImprovement, CanPlaceResource, DoesPlotBorderRiver, EditCliff, EditRiver (+7) |
| `GameCore\Shared\Lua_IWorldBuilderPlayerManager.cpp` | 1805f75e0 | 1805f8580 | 11 | 4218 | bloc:1 chaine:2 lua:8 |  | AddPlayer, ClearPlayerStartingPosition, ClearStartingPosition, GetPlayerConfig, GetSlotStatus, GetStartPositionInfo (+5) |
| `GameCore\Shared\MathAlgorithm_GeneralizedAssignmentProblem.cpp` | 1805f9630 | 1805f9630 | 1 | 2215 | release:1 |  |  |
| `GameCore\Shared\Random.cpp` | 1805fa120 | 1805fa370 | 4 | 563 | chaine:1 release:3 |  |  |
| `GameCore\Shared\Reporting_Event.cpp` | 1805fa790 | 1805fa830 | 3 | 169 | release:3 |  |  |
| `GameCore\Shared\Reporting_Strings.cpp` | 1805fd5a0 | 1805fd5a0 | 1 | 210 | fichier:1 |  |  |
| `GameCore\Shared\Scripting_Manager.cpp` | 1805fdb90 | 1805fde10 | 2 | 1143 | release:2 |  |  |
| `GameCore\Shared\Serialization_Helpers.cpp` | 1805fe490 | 180600000 | 39 | 6974 | release:39 |  |  |
| `GameCore\Shared\SharedParameters.cpp` | 1806025e0 | 180602660 | 2 | 309 | release:2 |  |  |
| `GameCore\Shared\Signals.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Tuner_Manager.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Utilities.cpp` | 180606270 | 180606290 | 2 | 274 | release:2 |  |  |
| `GameCore\Shared\WorldBuilder_City_Manager.cpp` | 1806066a0 | 180608b60 | 45 | 9167 | bloc:35 fichier:10 | 993 |  |
| `GameCore\Shared\WorldBuilder_Command_Manager.cpp` | 18060a790 | 18060a990 | 4 | 828 | fichier:1 release:3 | 224 |  |
| `GameCore\Shared\WorldBuilder_Configuration_Manager.cpp` | 18060ae30 | 18060ae30 | 1 | 187 | fichier:1 | 40 |  |
| `GameCore\Shared\WorldBuilder_Manager.cpp` | 18060b600 | 18060b600 | 1 | 142 | release:1 |  |  |
| `GameCore\Shared\WorldBuilder_Map_Manager.cpp` | 18060c840 | 18060eb40 | 38 | 8483 | bloc:3 chaine:1 fichier:1 release:33 | 921 |  |
| `GameCore\Shared\WorldBuilder_Map_Serializer.cpp` | 18060fcd0 | 18060ff40 | 3 | 1457 | chaine:1 release:2 |  |  |
| `GameCore\Shared\WorldBuilder_Player_Manager.cpp` | 180616ef0 | 180618bc0 | 48 | 7112 | bloc:28 fichier:8 release:12 | 1128 |  |
| `GameCore\Shared\WorldBuilder_Player_Remapping.cpp` | 180618e70 | 180618e70 | 1 | 437 | release:1 |  |  |
| `GameCore\Shared\WorldBuilder_RiverUtilities.cpp` | 18061b4a0 | 18061ba80 | 5 | 1879 | release:5 |  |  |
| `GameCore\Shared\WorldBuilder_SerializationDatabase.cpp` | 18061c4f0 | 18061c7f0 | 4 | 963 | bloc:1 chaine:3 |  |  |
| `GameCore\Shared\WorldBuilder_TerrainUtilities.cpp` | 18061cbd0 | 18061cbd0 | 1 | 247 | chaine:1 |  |  |
| `GameCore\Shared\WorldBuilder_Unit_Manager.cpp` | 18061ce80 | 18061d440 | 9 | 1707 | bloc:5 fichier:4 | 265 |  |
| `GameCore\Shared\Base\BarbarianClans_Manager.cpp` | 18061f120 | 180624b40 | 59 | 23299 | bloc:32 chaine:23 release:4 |  |  |
| `GameCore\Shared\Base\Definition_IProperties.cpp` | 180625350 | 180625350 | 1 | 643 | fichier:1 | 123 |  |
| `GameCore\Shared\Base\Game_EconomicManager.cpp` | 18062a9a0 | 180633200 | 98 | 33991 | bloc:82 chaine:13 fichier:1 release:2 | 156 | GameEconomic |
| `GameCore\Shared\Base\Game_HeroesManager.cpp` | 180633b20 | 180638490 | 56 | 18823 | bloc:29 chaine:14 fichier:1 release:12 | 101 | HeroesManager |
| `GameCore\Shared\Base\Game_Instance.cpp` | 18063ba90 | 18063e160 | 37 | 9955 | bloc:14 fichier:4 release:19 | 419 |  |
| `GameCore\Shared\Base\ObjectPairingManager.cpp` | 180640590 | 180640d10 | 7 | 1935 | fichier:1 release:6 | 80 | ObjectPairing |
| `GameCore\Shared\Base\Player_Heroes.cpp` | 180643a10 | 180646070 | 35 | 10352 | bloc:19 chaine:12 release:4 |  |  |
| `GameCore\Shared\Base\Player_Operations_BribeClan.cpp` | 180647e30 | 180648230 | 5 | 1404 | bloc:1 chaine:2 fichier:1 release:1 | 201 |  |
| `GameCore\Shared\Base\Player_Operations_HireClan.cpp` | 1806483e0 | 180648a90 | 5 | 2414 | bloc:1 chaine:2 fichier:1 release:1 | 305 |  |
| `GameCore\Shared\Base\Player_Operations_InciteClan.cpp` | 180648d70 | 1806492f0 | 5 | 1903 | bloc:1 chaine:2 fichier:1 release:1 | 265 |  |
| `GameCore\Shared\Base\Player_Operations_RansomClan.cpp` | 180649500 | 180649850 | 4 | 2018 | chaine:1 fichier:1 release:2 | 210 |  |
| `GameCore\Shared\Base\Quests_Manager.cpp` | 18064b380 | 18064c790 | 21 | 5176 | bloc:8 chaine:2 fichier:1 release:10 | 107 |  |
| `GameCore\Shared\Base\Rules_Era_Shared.cpp` | 18064e6c0 | 18064e810 | 4 | 541 | bloc:1 release:3 |  |  |
| `GameCore\Shared\Base\Rules_Player_Shared.cpp` | 18064ed10 | 18064ed10 | 1 | 399 | chaine:1 |  |  |
| `GameCore\Shared\Base\Rules_Units_Shared.cpp` | 18064ef90 | 18064f020 | 2 | 360 | release:2 |  |  |
| `GameCore\Shared\Base\Unit_Manager.cpp` | 18064f9b0 | 1806507d0 | 17 | 4759 | bloc:15 chaine:1 fichier:1 | 91 | Formation, UnitManager |
| `GameCore\Shared\Base\Notifications\Notification_BoostCivic.cpp` | 180651360 | 180651360 | 1 | 98 | fichier:1 | 47 |  |
| `GameCore\Shared\Base\Notifications\Notification_BoostTech.cpp` | 180651500 | 180651500 | 1 | 98 | fichier:1 | 46 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseArtifactPlayer.cpp` | 1806515e0 | 1806515e0 | 1 | 98 | fichier:1 | 68 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseBelief.cpp` | 180651770 | 180651770 | 1 | 98 | fichier:1 | 65 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseCityProduction.cpp` | 180651900 | 180651900 | 1 | 98 | fichier:1 | 71 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseCivic.cpp` | 180651a80 | 180651a80 | 1 | 98 | fichier:1 | 65 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseDragnetPriority.cpp` | 180651c80 | 180651c80 | 1 | 98 | fichier:1 | 70 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseEscapeRoute.cpp` | 180651dd0 | 180651dd0 | 1 | 98 | fichier:1 | 67 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChoosePantheon.cpp` | 180651f30 | 180651f30 | 1 | 98 | fichier:1 | 60 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseReligion.cpp` | 1806520a0 | 1806520a0 | 1 | 98 | fichier:1 | 73 |  |
| `GameCore\Shared\Base\Notifications\Notification_ChooseTech.cpp` | 180652250 | 180652250 | 1 | 98 | fichier:1 | 65 |  |
| `GameCore\Shared\Base\Notifications\Notification_CityRangeAttack.cpp` | 1806523f0 | 1806523f0 | 1 | 98 | fichier:1 | 79 |  |
| `GameCore\Shared\Base\Notifications\Notification_CivicDiscovered.cpp` | 1806525a0 | 1806525a0 | 1 | 98 | fichier:1 | 47 |  |
| `GameCore\Shared\Base\Notifications\Notification_ClaimGreatPerson.cpp` | 180652680 | 180652680 | 1 | 98 | fichier:1 | 68 |  |
| `GameCore\Shared\Base\Notifications\Notification_CommandUnits.cpp` | 1806527f0 | 1806527f0 | 1 | 98 | fichier:1 | 94 |  |
| `GameCore\Shared\Base\Notifications\Notification_ConsiderGovernmentChange.cpp` | 1806529d0 | 180652ab0 | 4 | 346 | fichier:1 release:3 | 84 |  |
| `GameCore\Shared\Base\Notifications\Notification_ConsiderRazeCity.cpp` | 180652b90 | 180652b90 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\Shared\Base\Notifications\Notification_DiplomacySession.cpp` | 180652d20 | 180652d20 | 1 | 97 | fichier:1 | 350 |  |
| `GameCore\Shared\Base\Notifications\Notification_FillCivicSlot.cpp` | 1806539f0 | 1806539f0 | 1 | 98 | fichier:1 | 98 |  |
| `GameCore\Shared\Base\Notifications\Notification_GiveInfluenceToken.cpp` | 180653bb0 | 180653bb0 | 1 | 98 | fichier:1 | 66 |  |
| `GameCore\Shared\Base\Notifications\Notification_PlayerDefeated.cpp` | 180653d20 | 180653d20 | 1 | 98 | fichier:1 | 90 |  |
| `GameCore\Shared\Base\Notifications\Notification_PromoteUnit.cpp` | 180654030 | 180654030 | 1 | 140 | fichier:1 | 132 |  |
| `GameCore\Shared\Base\Notifications\Notification_ReciprocalPlayer.cpp` | 1806545b0 | 1806545b0 | 1 | 98 | fichier:1 | 57 |  |
| `GameCore\Shared\Base\Notifications\Notification_TechDiscovered.cpp` | 180654820 | 180654820 | 1 | 98 | fichier:1 | 46 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_ActivateGreatPerson.cpp` | 1806549c0 | 180655560 | 4 | 3687 | bloc:1 chaine:1 fichier:1 release:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Airlift.cpp` | 1806560e0 | 1806561b0 | 2 | 437 | fichier:2 | 333 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Cancel.cpp` | 1806565c0 | 1806565c0 | 1 | 237 | fichier:1 | 16 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_CondemnHeretic.cpp` | 180656720 | 180656b40 | 4 | 1808 | fichier:1 release:3 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Delete.cpp` | 180656ef0 | 180656ef0 | 1 | 235 | fichier:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_DisperseClan.cpp` | 180657110 | 1806574b0 | 7 | 1080 | bloc:3 chaine:1 fichier:1 release:2 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_DistrictProduction.cpp` | 180657590 | 180657770 | 3 | 699 | fichier:1 release:2 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_ExecuteScript.cpp` | 180657b20 | 180657df0 | 5 | 930 | bloc:3 fichier:2 | 166 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_FormArmy.cpp` | 180658850 | 1806589c0 | 3 | 588 | bloc:1 fichier:2 | 246 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_FormCorps.cpp` | 1806592b0 | 180659420 | 3 | 588 | bloc:1 fichier:2 | 231 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Formation.cpp` | 180659d00 | 18065a080 | 6 | 1101 | bloc:3 fichier:3 | 224 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_HarvestWonder.cpp` | 18065a430 | 18065a430 | 1 | 235 | fichier:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_AreaDamageHeal.cpp` | 18065a7c0 | 18065ab80 | 4 | 1989 | fichier:1 release:3 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_BaseHandler.cpp` | 18065b210 | 18065b210 | 1 | 445 | release:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_CompleteDistrict.cpp` | 18065b540 | 18065b820 | 3 | 882 | chaine:1 fichier:1 release:1 | 34 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_CreateRandomResource.cpp` | 18065ba60 | 18065bee0 | 5 | 2053 | bloc:2 chaine:2 fichier:1 | 34 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_DestroyDevelopment.cpp` | 18065c290 | 18065c750 | 4 | 1818 | chaine:1 fichier:1 release:2 | 36 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_ForceLevyMilitary.cpp` | 18065c9c0 | 18065ce50 | 5 | 1487 | chaine:2 fichier:1 release:2 | 35 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_GrantEnvoy.cpp` | 18065cfb0 | 18065d460 | 5 | 1782 | chaine:1 fichier:1 release:3 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_GrantRandomTechBoost.cpp` | 18065d6e0 | 18065dcb0 | 5 | 1956 | chaine:1 fichier:1 release:3 | 36 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_KillWeakerUnit.cpp` | 18065dfa0 | 18065e560 | 5 | 2095 | bloc:1 fichier:2 release:2 | 196 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_NavalGoldRaid.cpp` | 18065e8c0 | 18065f010 | 8 | 2623 | bloc:3 fichier:2 release:3 | 210 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_ResourceInspire.cpp` | 18065f380 | 18065f9b0 | 6 | 2361 | chaine:2 fichier:1 release:3 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_RestoreUnitMoves.cpp` | 18065fdf0 | 1806602f0 | 5 | 1749 | bloc:1 fichier:2 release:2 | 166 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Hero_TransformUnit.cpp` | 180660630 | 180660c40 | 6 | 2174 | bloc:2 fichier:2 release:2 | 203 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_NameUnit.cpp` | 1806610a0 | 180661170 | 2 | 432 | fichier:2 | 156 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_PetTheDog.cpp` | 1806615e0 | 1806616e0 | 2 | 595 | fichier:1 release:1 | 24 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_PlunderTradeRoute.cpp` | 180661870 | 180661be0 | 3 | 1298 | release:3 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Promote.cpp` | 1806621d0 | 180662400 | 5 | 776 | bloc:3 fichier:2 | 182 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_RaidClan.cpp` | 180662620 | 180662cf0 | 6 | 2350 | bloc:3 chaine:2 fichier:1 | 32 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Unlevy.cpp` | 1806630a0 | 1806630a0 | 1 | 235 | fichier:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Upgrade.cpp` | 1806632a0 | 180663370 | 2 | 437 | fichier:2 | 202 |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_Wake.cpp` | 180663c50 | 180663c50 | 1 | 235 | fichier:1 |  |  |
| `GameCore\Shared\Base\UnitCommands\Unit_Commands_WonderProduction.cpp` | 180663db0 | 180663fd0 | 3 | 758 | fichier:1 release:2 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_AirAttack.cpp` | 1806648b0 | 180664a70 | 3 | 551 | bloc:1 fichier:2 | 170 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Alert.cpp` | 1806650e0 | 1806650e0 | 1 | 122 | fichier:1 | 24 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_AutomateExplore.cpp` | 180665560 | 180665630 | 2 | 324 | fichier:2 | 161 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_BuildRoute.cpp` | 180665bf0 | 180665fd0 | 7 | 1468 | bloc:2 fichier:1 release:4 | 206 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ClearContamination.cpp` | 180666360 | 1806663e0 | 2 | 654 | fichier:1 release:1 | 33 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_CoastalRaid.cpp` | 180666c10 | 180666e50 | 4 | 2110 | bloc:1 fichier:2 release:1 | 445 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ConvertBarbarians.cpp` | 180667600 | 180667600 | 1 | 122 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Deploy.cpp` | 1806681c0 | 180668380 | 3 | 550 | bloc:1 fichier:2 | 147 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_DesignatePark.cpp` | 180668740 | 180668950 | 3 | 637 | bloc:1 fichier:2 | 138 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Disembark.cpp` | 180668b60 | 180668b60 | 1 | 126 | fichier:1 | 17 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Embark.cpp` | 180668c20 | 180668c20 | 1 | 126 | fichier:1 | 18 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_EvangelizeBelief.cpp` | 180668e20 | 180668e20 | 1 | 122 | fichier:1 | 32 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ExecuteScript.cpp` | 180669110 | 1806692c0 | 4 | 542 | bloc:2 fichier:2 | 156 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ExtractArtifact.cpp` | 1806696c0 | 1806696c0 | 1 | 122 | fichier:1 | 41 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Fortify.cpp` | 180669bd0 | 180669bd0 | 1 | 122 | fichier:1 | 21 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_FoundCity.cpp` | 180669e40 | 180669e40 | 1 | 122 | fichier:1 | 38 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_FoundReligion.cpp` | 18066a6a0 | 18066a720 | 2 | 901 | fichier:1 release:1 | 44 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_HarvestResource.cpp` | 18066ab10 | 18066ab90 | 2 | 2423 | fichier:1 release:1 | 46 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Heal.cpp` | 18066b770 | 18066b770 | 1 | 122 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_LaunchInquisition.cpp` | 18066ba20 | 18066ba20 | 1 | 122 | fichier:1 | 32 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_MakeTradeRoute.cpp` | 18066bf70 | 18066c630 | 7 | 2132 | bloc:1 fichier:1 release:5 | 308 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_MoveTo.cpp` | 18066ccc0 | 18066d0e0 | 6 | 2209 | bloc:2 fichier:2 release:2 | 374 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_MoveToUnit.cpp` | 18066d580 | 18066d800 | 5 | 718 | fichier:2 release:3 | 136 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Pillage.cpp` | 18066dc70 | 18066dee0 | 5 | 1455 | bloc:2 fichier:2 release:1 | 419 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_PillageRoute.cpp` | 18066e320 | 18066e520 | 3 | 770 | fichier:1 release:2 | 30 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_PlantForest.cpp` | 18066e750 | 18066e750 | 1 | 122 | fichier:1 | 35 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RangeAttack.cpp` | 18066eb60 | 18066f660 | 5 | 2904 | bloc:1 fichier:2 release:2 | 230 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ReBase.cpp` | 18066fe60 | 180670020 | 3 | 551 | bloc:1 fichier:2 | 234 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_ReligiousHeal.cpp` | 180670610 | 180670610 | 1 | 122 | fichier:1 | 33 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RemoveFeature.cpp` | 180671090 | 180671110 | 2 | 1047 | fichier:1 release:1 | 44 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RemoveHeresy.cpp` | 1806718a0 | 1806718a0 | 1 | 126 | fichier:1 | 34 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RemoveImprovement.cpp` | 180671940 | 180671af0 | 3 | 702 | release:3 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Repair.cpp` | 180671c40 | 180672030 | 6 | 1342 | bloc:1 fichier:1 release:4 | 242 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RepairRoute.cpp` | 180672220 | 180672300 | 3 | 442 | fichier:1 release:2 | 24 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RestRepair.cpp` | 180672800 | 180672800 | 1 | 122 | fichier:1 | 33 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Retrain.cpp` | 1806728c0 | 1806728c0 | 1 | 122 | fichier:1 | 21 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_RouteTo.cpp` | 180672a30 | 180672b00 | 2 | 323 | fichier:2 | 109 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SkipTurn.cpp` | 180672d90 | 180672d90 | 1 | 122 | fichier:1 | 23 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Sleep.cpp` | 180672fe0 | 180672fe0 | 1 | 122 | fichier:1 | 21 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpreadReligion.cpp` | 180673740 | 1806737c0 | 2 | 1387 | fichier:1 release:1 | 34 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyCounterspy.cpp` | 180674250 | 180674520 | 3 | 1175 | fichier:1 release:2 | 167 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyDisruptRocketry.cpp` | 180674aa0 | 180674aa0 | 1 | 2103 | chaine:1 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyGainSources.cpp` | 1806757b0 | 1806759b0 | 4 | 1163 | bloc:1 chaine:3 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyGreatWorkHeist.cpp` | 180676410 | 180676b60 | 3 | 2192 | chaine:1 release:2 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyRecruitPartisans.cpp` | 1806775b0 | 180677900 | 4 | 994 | chaine:1 release:3 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpySabotageProduction.cpp` | 180677ba0 | 180677ba0 | 1 | 702 | release:1 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpySiphonFunds.cpp` | 180679420 | 180679420 | 1 | 366 | release:1 |  |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyStealTechBoost.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyTravelNewCity.cpp` | 18067ad30 | 18067aef0 | 3 | 783 | fichier:1 release:2 | 205 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SpyZombify.cpp` | 18067b1e0 | 18067b510 | 3 | 928 | chaine:1 fichier:1 release:1 | 39 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_SwapUnits.cpp` | 18067c5c0 | 18067c7e0 | 3 | 651 | bloc:1 fichier:2 | 90 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_TeleportTo.cpp` | 18067ca70 | 18067cfb0 | 7 | 2038 | bloc:1 fichier:2 release:4 | 129 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_TeleportToCity.cpp` | 18067d7b0 | 18067d900 | 3 | 558 | fichier:2 release:1 | 142 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_Upgrade.cpp` | 18067d9f0 | 18067d9f0 | 1 | 122 | fichier:1 | 35 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_WMDStrike.cpp` | 18067e260 | 18067e900 | 6 | 1782 | bloc:3 chaine:1 fichier:2 | 284 |  |
| `GameCore\Shared\Base\UnitOperations\Unit_Operations_WaitFor.cpp` | 18067ec00 | 18067ece0 | 2 | 334 | fichier:2 | 78 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_CityFocus.cpp` | 18067ed90 | 18067f130 | 5 | 1002 | bloc:3 fichier:2 | 144 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_Destroy.cpp` | 18067f3d0 | 18067f490 | 2 | 392 | fichier:1 release:1 | 200 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_ManageCitizen.cpp` | 18067fbe0 | 18067fd20 | 3 | 420 | bloc:1 fichier:2 | 138 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_NameCity.cpp` | 18067ff30 | 180680120 | 4 | 584 | bloc:2 fichier:2 | 128 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_Purchase.cpp` | 1806807e0 | 180680b80 | 5 | 1320 | fichier:1 release:4 | 411 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_RangeAttack.cpp` | 180681df0 | 180682070 | 5 | 739 | bloc:3 fichier:2 | 197 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_SwapTileOwner.cpp` | 180682470 | 180682740 | 6 | 849 | bloc:2 fichier:2 release:2 | 146 |  |
| `GameCore\Shared\Base\CityCommands\City_Commands_WMDStrike.cpp` | 1806828e0 | 180682ea0 | 5 | 1558 | bloc:2 fichier:2 release:1 | 256 |  |
| `GameCore\Shared\Base\CityOperations\City_Operations_Build.cpp` | 1806832e0 | 180684490 | 12 | 4564 | bloc:10 fichier:2 | 645 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IAiCityBuild.cpp` | 1806849d0 | 180684b80 | 3 | 456 | chaine:1 fichier:1 lua:1 | 27 | GetBuildRecommendations, GetImprovementRecommendations |
| `GameCore\Shared\Base\Cache\Cache_Lua_IAiDiplomacy.cpp` | 1806850e0 | 1806850e0 | 1 | 112 | fichier:1 | 20 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IAiGrandStrategic.cpp` | 180685890 | 180685890 | 1 | 112 | fichier:1 | 21 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IBarbarians.cpp` | 180686510 | 180686510 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICities.cpp` | 180687430 | 180687430 | 1 | 131 | fichier:1 | 37 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICity.cpp` | 18068a5d0 | 18068a5d0 | 1 | 112 | fichier:1 | 37 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityBuildQueue.cpp` | 18068b970 | 18068b970 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityBuildings.cpp` | 18068d2f0 | 18068d2f0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityCitizens.cpp` | 18068de00 | 18068de00 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityCulture.cpp` | 18068e2e0 | 18068e2e0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityDistricts.cpp` | 18068ead0 | 18068ec00 | 4 | 410 | fichier:1 lua:3 | 29 | FindID, GetCount, GetDistrict |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityGold.cpp` | 18068f2c0 | 18068f2c0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityGrowth.cpp` | 18068fe90 | 18068fe90 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityManager.cpp` | 180690d50 | 180690d50 | 1 | 131 | fichier:1 | 38 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityReligion.cpp` | 180692130 | 180692130 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ICityTrade.cpp` | 180692d70 | 180692d70 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IDealManager.cpp` | 1806938f0 | 1806938f0 | 1 | 131 | fichier:1 | 33 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IDiplomacyManager.cpp` | 180694210 | 180694970 | 7 | 2014 | chaine:1 fichier:1 lua:5 | 30 | AddCommand, AddResponse, AddStatement, CloseSession, FindOpenSessionID |
| `GameCore\Shared\Base\Cache\Cache_Lua_IDistrict.cpp` | 180695c70 | 180695c70 | 1 | 112 | fichier:1 | 32 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IFalloutManager.cpp` | 180696950 | 180696950 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGame.cpp` | 18069a1c0 | 18069a1c0 | 1 | 131 | fichier:1 | 79 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameDiplomacy.cpp` | 18069d180 | 18069d180 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameGossipManager.cpp` | 18069d440 | 18069d440 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameGreatPeople.cpp` | 18069dbb0 | 18069dbb0 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameHeroesManager.cpp` | 18069eb70 | 18069eb70 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameNationalParks.cpp` | 18069f370 | 18069f370 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IGameReligion.cpp` | 1806a01c0 | 1806a01c0 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IInfluenceMap.cpp` | 1806a0be0 | 1806a0be0 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IMap.cpp` | 1806a14c0 | 1806a14c0 | 1 | 131 | fichier:1 | 35 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IMapCityPlots.cpp` | 1806a3400 | 1806a3400 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IMapPlotUnitList.cpp` | 1806a3c10 | 1806a3c10 | 1 | 112 | fichier:1 | 31 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayer.cpp` | 1806a93b0 | 1806a93b0 | 1 | 131 | fichier:1 | 71 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerCities.cpp` | 1806aaf60 | 1806aaf60 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerCulture.cpp` | 1806ac270 | 1806ac270 | 1 | 112 | fichier:1 | 34 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerDiplomacy.cpp` | 1806aeed0 | 1806afcb0 | 23 | 3694 | bloc:14 chaine:2 fichier:1 lua:6 | 38 | CanDeclareWarOn, CanLiberateCityTo, CanMakePeaceWith, ComputeCityWarmongerPoints, ComputeDOWWarmongerPoints, ComputeResearchAgreementTurns (+16) |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerDistricts.cpp` | 1806b08c0 | 1806b08c0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerGreatPeoplePoints.cpp` | 1806b0dd0 | 1806b0dd0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerImprovements.cpp` | 1806b1010 | 1806b1010 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerInfluence.cpp` | 1806b1630 | 1806b1630 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerManager.cpp` | 1806b1c90 | 1806b1c90 | 1 | 131 | fichier:1 | 29 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerReligion.cpp` | 1806b33b0 | 1806b33b0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerResources.cpp` | 1806b3ba0 | 1806b3ba0 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerStats.cpp` | 1806b4a30 | 1806b4a30 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerTechs.cpp` | 1806b5400 | 1806b5400 | 1 | 112 | fichier:1 | 31 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerTrade.cpp` | 1806b6090 | 1806b6090 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerTreasury.cpp` | 1806b66d0 | 1806b66d0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerUnits.cpp` | 1806b6dd0 | 1806b6dd0 | 1 | 112 | fichier:1 | 33 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerVisibility.cpp` | 1806b7970 | 1806b7970 | 1 | 131 | fichier:1 | 56 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerVisibilityManager.cpp` | 1806b7f70 | 1806b7f70 | 1 | 131 | fichier:1 | 29 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IPlayerWMDs.cpp` | 1806b8390 | 1806b8390 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IQuestsManager.cpp` | 1806b8830 | 1806b8830 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_ITradeManager.cpp` | 1806b9630 | 1806b9630 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnit.cpp` | 1806bc620 | 1806bc620 | 1 | 112 | fichier:1 | 34 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitAbility.cpp` | 1806bde50 | 1806bde50 | 1 | 112 | fichier:1 | 25 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitArchaeology.cpp` | 1806be2d0 | 1806be2d0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitExperience.cpp` | 1806be9e0 | 1806be9e0 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitGreatPerson.cpp` | 1806bf0e0 | 1806bf0e0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitManager.cpp` | 1806bfa30 | 1806bfa30 | 1 | 131 | fichier:1 | 44 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnitTrade.cpp` | 1806c34b0 | 1806c34b0 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Cache\Cache_Lua_IUnits.cpp` | 1806c35a0 | 1806c35a0 | 1 | 131 | fichier:1 | 43 |  |
| `GameCore\Shared\Base\Lua\Lua_ICity.cpp` | 1806c8a10 | 1806c8a10 | 1 | 112 | fichier:1 | 47 |  |
| `GameCore\Shared\Base\Lua\Lua_ICityManager.cpp` | 1806cc680 | 1806cc680 | 1 | 131 | fichier:1 | 32 |  |
| `GameCore\Shared\Base\Lua\Lua_IDeal.cpp` | 1806ce270 | 1806cecb0 | 8 | 3175 | lua:8 |  | AddItemOfType, ClearItemsByType, ClearItemsByValue, FindItemByID, FindItemByType, FindItemByValueType (+2) |
| `GameCore\Shared\Base\Lua\Lua_IFalloutManager.cpp` | 1806d2cc0 | 1806d2cc0 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\Base\Lua\Lua_IGame.cpp` | 1806d5bf0 | 1806d5bf0 | 1 | 131 | fichier:1 | 57 |  |
| `GameCore\Shared\Base\Lua\Lua_IGameDiplomacy.cpp` | 1806d8890 | 1806d8890 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Lua\Lua_IGameEffects.cpp` | 1806db2e0 | 1806db2e0 | 1 | 131 | fichier:1 | 45 |  |
| `GameCore\Shared\Base\Lua\Lua_IGameHeroesManager.cpp` | 1806e0480 | 1806e0480 | 1 | 112 | fichier:1 | 32 |  |
| `GameCore\Shared\Base\Lua\Lua_IGameReligion.cpp` | 1806e0bf0 | 1806e0bf0 | 1 | 112 | fichier:1 | 30 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayer.cpp` | 1806ed4c0 | 1806ed4c0 | 1 | 131 | fichier:1 | 79 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerCulture.cpp` | 1806f0b60 | 1806f0b60 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerDiplomacy.cpp` | 1806f1df0 | 1806f1df0 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerDistricts.cpp` | 1806f2b00 | 1806f2b00 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerManager.cpp` | 1806f3cd0 | 1806f3cd0 | 1 | 131 | fichier:1 | 34 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerResources.cpp` | 1806f5f50 | 1806f5f50 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerStats.cpp` | 1806f6420 | 1806f6420 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\Base\Lua\Lua_IPlayerTrade.cpp` | 1806f79a0 | 1806f79a0 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\XP1\Game_SecretSocietyManager.cpp` | 180704140 | 1807066c0 | 37 | 10536 | bloc:18 chaine:5 fichier:1 release:13 | 99 | SecretSocietyManager |
| `GameCore\Shared\XP1\Cache\Cache_Lua_ICityCulturalIdentity.cpp` | 1807082a0 | 1807082a0 | 1 | 112 | fichier:1 | 31 |  |
| `GameCore\Shared\XP1\Cache\Cache_Lua_IGameHistoryManager.cpp` | 18070be70 | 18070d3c0 | 21 | 12022 | bloc:16 chaine:4 fichier:1 | 41 | GetAllMomentsData, GetMomentCount, GetMomentData, GetMomentType, HasPrideMoments, WritePrideMomentInfo |
| `GameCore\Shared\XP1\Cache\Cache_Lua_IGovernor.cpp` | 18070f1d0 | 18070f1d0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\XP1\Cache\Cache_Lua_IPlayerGovernors.cpp` | 18070ff80 | 18070ff80 | 1 | 112 | fichier:1 | 32 |  |
| `GameCore\Shared\XP1\Cache\XP1_Cache_Lua_ICityGrowth.cpp` | 180711160 | 180711160 | 1 | 112 | fichier:1 | 20 |  |
| `GameCore\Shared\XP1\Cache\XP1_Cache_Lua_IMap.cpp` | 180712140 | 180712140 | 1 | 131 | fichier:1 | 24 |  |
| `GameCore\Shared\XP1\Cache\XP1_Cache_Lua_IPlayer.cpp` | 180712900 | 180712900 | 1 | 121 | fichier:1 | 27 |  |
| `GameCore\Shared\XP1\Cache\XP1_Cache_Lua_IPlayerCities.cpp` | 180712ae0 | 180712ae0 | 1 | 112 | fichier:1 | 20 |  |
| `GameCore\Shared\XP1\Cache\XP1_Cache_Lua_IPlayerGreatPeoplePoints.cpp` | 180713180 | 180713180 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP1\Lua\Lua_IGameEras.cpp` | 1807138a0 | 1807138a0 | 1 | 112 | fichier:1 | 21 |  |
| `GameCore\Shared\XP1\Lua\Lua_IPlayerGovernors.cpp` | 180713bd0 | 180713bd0 | 1 | 112 | fichier:1 | 24 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_ICity.cpp` | 180713d90 | 180713d90 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_ICityManager.cpp` | 180714220 | 180714220 | 1 | 131 | fichier:1 | 23 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_IGame.cpp` | 180714760 | 180714760 | 1 | 131 | fichier:1 | 22 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_IPlayer.cpp` | 180714cd0 | 180714cd0 | 1 | 121 | fichier:1 | 25 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_IPlayerCulture.cpp` | 180714df0 | 180714df0 | 1 | 112 | fichier:1 | 27 |  |
| `GameCore\Shared\XP1\Lua\XP1_Lua_IPlayerManager.cpp` | 180714ed0 | 180714ed0 | 1 | 131 | fichier:1 | 24 |  |
| `GameCore\Shared\XP1\UnitCommands\Unit_Commands_SpreadDissent.cpp` | 180715270 | 1807153d0 | 3 | 566 | bloc:1 fichier:2 | 203 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyCounterspy.cpp` | 180715c10 | 180715c10 | 1 | 126 | fichier:1 | 35 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyDisruptRocketry.cpp` | 180715fe0 | 180715fe0 | 1 | 126 | fichier:1 | 35 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyFabricateScandal.cpp` | 180716220 | 180716910 | 6 | 1877 | bloc:2 chaine:3 fichier:1 | 40 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyFomentUnrest.cpp` | 180717950 | 180717950 | 1 | 126 | fichier:1 | 36 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyGainSources.cpp` | 1807184d0 | 1807184d0 | 1 | 126 | fichier:1 | 31 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyGreatWorkHeist.cpp` | 180718900 | 180718900 | 1 | 126 | fichier:1 | 40 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyListeningPost.cpp` | 180718b30 | 180718b30 | 1 | 126 | fichier:1 | 32 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyNeutralizeGovernor.cpp` | 180718fe0 | 180718fe0 | 1 | 126 | fichier:1 | 40 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyRecruitPartisans.cpp` | 1807199c0 | 180719d50 | 3 | 3167 | chaine:1 fichier:1 release:1 | 35 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpySabotageProduction.cpp` | 18071a640 | 18071a980 | 2 | 953 | fichier:1 release:1 | 35 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpySiphonFunds.cpp` | 18071ad30 | 18071ad30 | 1 | 126 | fichier:1 | 36 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyStealTechBoost.cpp` | 18071b180 | 18071b180 | 1 | 126 | fichier:1 | 40 |  |
| `GameCore\Shared\XP1\UnitOperations\XP1_Unit_Operations_SpyTravelNewCity.cpp` | 18071b780 | 18071b780 | 1 | 126 | fichier:1 | 36 |  |
| `GameCore\Shared\XP1\CityCommands\XP1_City_Commands_Destroy.cpp` | 18071b860 | 18071b860 | 1 | 119 | fichier:1 | 27 |  |
| `GameCore\Shared\XP1\CityCommands\XP1_City_Commands_Purchase.cpp` | 18071bf50 | 18071c040 | 2 | 678 | fichier:1 release:1 | 371 |  |
| `GameCore\Shared\XP2\XP2_Map_Feature_Manager.cpp` | 1807201d0 | 180727e70 | 87 | 31254 | bloc:71 chaine:12 fichier:1 release:3 | 457 | Feature |
| `GameCore\Shared\XP2\XP2_Map_River_Manager.cpp` | 18072c380 | 18072ff80 | 66 | 15486 | bloc:60 fichier:3 release:3 | 1222 | River |
| `GameCore\Shared\XP2\XP2_Map_Terrain_Manager.cpp` | 180734dd0 | 180739e20 | 58 | 21307 | bloc:42 chaine:11 fichier:1 release:4 | 280 | Terrain |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_ICity.cpp` | 18073d6d0 | 18073d6d0 | 1 | 112 | fichier:1 | 23 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_ICityBuildQueue.cpp` | 18073d8a0 | 18073d8a0 | 1 | 112 | fichier:1 | 21 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_ICityCulturalIdentity.cpp` | 18073dc70 | 18073dc70 | 1 | 112 | fichier:1 | 24 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IFalloutManager.cpp` | 180740300 | 180740300 | 1 | 112 | fichier:1 | 22 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IGame.cpp` | 180740ca0 | 180740ca0 | 1 | 131 | fichier:1 | 20 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IGameDiplomacy.cpp` | 180740e80 | 180740e80 | 1 | 112 | fichier:1 | 18 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IGovernor.cpp` | 180741630 | 180741630 | 1 | 112 | fichier:1 | 18 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayer.cpp` | 180741800 | 180741880 | 2 | 754 | chaine:1 fichier:1 | 26 | GetAgendasAndVisibilities |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerCulture.cpp` | 180741e20 | 180741e20 | 1 | 112 | fichier:1 | 24 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerDiplomacy.cpp` | 1807421b0 | 1807421b0 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerInfluence.cpp` | 180742350 | 180742350 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerResources.cpp` | 1807425b0 | 1807425b0 | 1 | 112 | fichier:1 | 20 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerStats.cpp` | 180742ae0 | 180742ae0 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IPlayerTrade.cpp` | 180742ec0 | 180742ec0 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Lua_IUnit.cpp` | 1807433e0 | 1807433e0 | 1 | 112 | fichier:1 | 22 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Map_Feature_Manager.cpp` | 180743550 | 180743550 | 1 | 123 | fichier:1 | 20 | FeatureManager |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Map_River_Manager.cpp` | 180743b10 | 180743b10 | 1 | 123 | fichier:1 | 21 | RiverManager |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Map_Terrain_EventStream.cpp` | 180744520 | 1807446c0 | 3 | 674 | fichier:1 release:2 | 29 |  |
| `GameCore\Shared\XP2\Cache\XP2_Cache_Map_Terrain_Manager.cpp` | 180744c20 | 180744f90 | 7 | 952 | fichier:1 release:6 | 81 | Terrain |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IFalloutManager.cpp` | 1807451a0 | 1807451a0 | 1 | 112 | fichier:1 | 19 |  |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IGameDiplomacy.cpp` | 180745720 | 180745720 | 1 | 112 | fichier:1 | 21 |  |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IPlayerDiplomacy.cpp` | 180745f60 | 180745f60 | 1 | 112 | fichier:1 | 23 |  |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IPlayerResources.cpp` | 180746200 | 180746200 | 1 | 112 | fichier:1 | 28 |  |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IPlayerStats.cpp` | 180746370 | 180746370 | 1 | 112 | fichier:1 | 26 |  |
| `GameCore\Shared\XP2\Lua\XP2_Lua_IPlayerTrade.cpp` | 1807464c0 | 1807464c0 | 1 | 112 | fichier:1 | 18 |  |
| `GameCore\Shared\XP2\UnitCommands\XP2_Unit_Commands_Upgrade.cpp` | 180746580 | 180746580 | 1 | 255 | fichier:1 |  |  |
| `GameCore\Shared\XP2\UnitOperations\Unit_Operations_CauseDisaster.cpp` | 180747240 | 180747240 | 1 | 122 | fichier:1 | 39 |  |
| `GameCore\Shared\XP2\UnitOperations\Unit_Operations_SacrificeUnit.cpp` | 180747aa0 | 180747c70 | 3 | 569 | bloc:1 fichier:2 | 227 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_BuildImprovement.cpp` | 180747f90 | 1807490c0 | 6 | 5488 | bloc:1 chaine:1 fichier:2 release:2 | 754 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_BuildImprovementAdjacent.cpp` | 1807496e0 | 18074a680 | 9 | 5095 | bloc:3 fichier:2 release:4 | 556 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_BuildRoute.cpp` | 18074ac90 | 18074b180 | 4 | 2081 | bloc:1 chaine:1 fichier:1 release:1 | 36 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_CoastalRaid.cpp` | 18074b820 | 18074b8a0 | 2 | 1680 | fichier:1 release:1 | 42 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_MakeTradeRoute.cpp` | 18074bec0 | 18074bec0 | 1 | 126 | fichier:1 | 37 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_Pillage.cpp` | 18074c5b0 | 18074c5b0 | 1 | 126 | fichier:1 | 47 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_RemoveFeature.cpp` | 18074c630 | 18074c6b0 | 2 | 1225 | fichier:1 release:1 | 35 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_RemoveImprovement.cpp` | 18074cb00 | 18074cc60 | 2 | 478 | fichier:1 release:1 | 32 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_Repair.cpp` | 18074cce0 | 18074cf30 | 2 | 710 | chaine:1 fichier:1 | 36 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_SpreadReligion.cpp` | 18074cfb0 | 18074d030 | 2 | 1554 | fichier:1 release:1 | 36 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_SpyBreachDam.cpp` | 18074d5d0 | 18074da10 | 4 | 1195 | bloc:1 chaine:1 fichier:1 release:1 | 37 |  |
| `GameCore\Shared\XP2\UnitOperations\XP2_Unit_Operations_TourismBomb.cpp` | 18074e810 | 18074ea40 | 3 | 668 | bloc:1 fichier:2 | 240 |  |
| `GameCore\Shared\XP2\CItyCommands\XP2_City_Commands_Purchase.cpp` | 18074f600 | 18074f600 | 1 | 119 | fichier:1 | 47 |  |
| `GameCore\Shared\XP2\CityOperations\XP2_City_Operations_Build.cpp` | 18074fce0 | 18074fce0 | 1 | 126 | fichier:1 | 29 |  |
| `GameCore\XP2\Target\Target_Context_Instance.cpp` | 180750450 | 1807504c0 | 2 | 183 | fichier:2 | 50 |  |
| `GameCore\XP2\Target\Target_Game_Instance.cpp` | 180750670 | 1807506d0 | 2 | 369 | fichier:2 | 369 |  |
| `GameCore\XP2\Target\Target_Game_Instance_Serialization.cpp` | 180751b60 | 180751b60 | 1 | 2332 | release:1 |  |  |
| `GameCore\XP2\Target\Target_Notification_Manager.cpp` | 1807529d0 | 1807529d0 | 1 | 93 | fichier:1 | 54 | Notifications |
| `GameCore\XP2\Target\Target_Unit_Commands_Manager.cpp` | 180752bd0 | 180752bd0 | 1 | 151 | fichier:1 |  | UnitCommands |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_StealPopOnPlotToCreateUnit.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Effects_AdjustCityFreePower.cpp` | 1807e3ef0 | 1808d8ef0 | 3 | 535 | chaine:2 release:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantFreeUnitByDomain.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Collections_CityTrainedUnits.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantUnitByClassAndApplyAbility.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_Collections_SimpleCollection.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Collections_AllianceCombats.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_Diplomacy_Agenda_AmbiorixArmy.cpp` | 1807e4000 | 1808d3ed0 | 1710 | 389174 | bloc:1687 chaine:17 release:6 |  |  |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Effects_AdjustReligionAwardsEnvoyReligiousPressure.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Effects_AttachModifierToPlayerType.cpp` | 1807eb130 | 1807eb180 | 2 | 142 | chaine:2 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AdjustReligionEnableAutoSpread.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Collections_EmergencyCombats.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_UnitEffect.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Requirements_GameEraAtMostExpansion.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_CityHasCounterspy.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantFreeUnitOnPlot.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_TriggerGameMechanic.cpp` | 1808d4850 | 1808d4850 | 1 | 99 | chaine:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AddReligiousUnit.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantUnitInEachDistrict.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_Diplomacy_BaseEffect.cpp` | 18080bab0 | 18080bab0 | 1 | 2624 | chaine:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_CityLevyUnitsInTerritory.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Effects_AdjustAccumulatingBonus.cpp` | 1808d0b10 | 1808d0b10 | 1 | 897 | chaine:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_UnitInFormation.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Effects_AdjustPlayerFavorRefundForSuccessfulResolution.cpp` | 1808c5280 | 1808c5280 | 1 | 926 | chaine:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_PlayerEraAtLeast.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantRelic.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantPlayerRandomCivic.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AddReligiousBuilding.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Requirements_GameEraAtLeastExpansion.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Requirements_CityHasGarrisonUnit.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_CityGrantHero.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Requirements_GameEraIs.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantPlayerRandomTechnology.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_CityGrantRandomResourceProduct.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantUnitWithExperience.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Collections_PlayerCombat.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AdjustWarWeariness.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_EraIs.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AdjustUnitMilitaryFormation.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_GrantFreeUnitByClass.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Effects_AttachModifierToMinorCivBonusType.cpp` | 1808c8150 | 1808c8ab0 | 9 | 2373 | bloc:3 chaine:2 release:4 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_GameEraAtLeast.cpp` | 180859c90 | 180859c90 | 1 | 449 | chaine:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AdjustReligionAwardsEnvoy.cpp` | 1808d1890 | 1808d1890 | 1 | 242 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_District.cpp` | 18092a3d0 | 180935480 | 2 | 1072 | release:2 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_CombatProposals.cpp` | 1809303d0 | 1809303d0 | 1 | 266 | release:1 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_Objects_ModifierObject.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_ObjectPool.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_PlotYields.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_RequirementSystem.cpp` | 180933aa0 | 180933aa0 | 1 | 343 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Alliance.cpp` | 180934d40 | 180934d40 | 1 | 579 | release:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Objects_ArtifactExtraction.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Belief.cpp` | 180935110 | 180935110 | 1 | 596 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Congress.cpp` | 180935370 | 180935370 | 1 | 267 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Emergency.cpp` | 1809357a0 | 1809357a0 | 1 | 630 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Game.cpp` | 180935a20 | 180935a20 | 1 | 405 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Serialization.cpp` | 180936240 | 18094b3f0 | 32 | 28461 | bloc:24 chaine:6 release:2 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Improvement.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Player.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Team.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_TradeRoute.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Unit.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Objects_UnitDeath.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_PlayerHasTechnology.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_GameEventHandler.cpp` | 18093f6a0 | 18093f990 | 6 | 851 | release:6 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_Validation.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_PlayerHasCivic.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\XP1\GameEffects\GameEffects_Requirements_AllianceCityHasRouteBetweenAllies.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_Modifiers_DynamicModifier.cpp` | 180942ac0 | 1809499f0 | 4 | 2832 | release:4 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_ModifierSystem.cpp` | 180943110 | 180970d60 | 33 | 18204 | bloc:10 chaine:2 fichier:2 release:19 | 367 | DynamicModifier |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_Diplomacy_SimpleEffect.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_UnitTeleport.cpp` | 18095bda0 | 18095bda0 | 1 | 731 | chaine:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Effects_Diplomacy_Agenda_Lawgiver.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Effects_AdjustUnitCombatStrengthForEra.cpp` | 1809511e0 | 1809511e0 | 1 | 314 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_PotentialTradeRoute.cpp` | 1809535a0 | 1809770a0 | 2 | 1240 | chaine:2 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_ModifierLibrary.cpp` | 18095b450 | 18095b530 | 2 | 974 | chaine:2 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_Requirements_ConstRequirement.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_Requirements_SimpleRequirement.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_City.cpp` | 18095c510 | 18095c510 | 1 | 988 | release:1 |  |  |
| `GameCore\XP2\Common\GameEffects\GameEffects_Objects_Governor.cpp` | 18095c8f0 | 18095c8f0 | 1 | 708 | release:1 |  |  |
| `GameCore\Core\Common\GameEffects\GameEffects_Logging.cpp` | - | - | 0 | 0 | | | |
| `GameCore\XP2\Common\GameEffects\GameEffects_ExternalTypes.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_PlotNearCapital.cpp` | 180961380 | 180961380 | 1 | 118 | release:1 |  |  |
| `GameCore\Shared\Base\GameEffects\GameEffects_Requirements_PlayerMajorityReligionIsOwner.cpp` | - | - | 0 | 0 | | | |
| `GameCore\Core\Common\GameEffects\GameEffects_RequirementSets_SimpleRequirementSet.cpp` | - | - | 0 | 0 | | | |
| `CivTech\Platform\Windows\Platform_Memory.cpp` | - | - | 0 | 0 | | | |
| `CivTech\Platform\Windows\Platform_String.cpp` | 180990f60 | 180990fb0 | 2 | 154 | release:2 |  |  |
| `CivTech\Platform\Common\Platform_MemoryBuffer.cpp` | 1809919c0 | 180991b40 | 6 | 486 | release:6 |  |  |
| `CivTech\Platform\Common\Platform_JobManager.cpp` | - | - | 0 | 0 | | | |
| `CivTech\Platform\Common\Platform_StringCommon.cpp` | 1809923d0 | 1809923d0 | 1 | 121 | release:1 |  |  |
| `CivTech\Platform\Windows\Platform_IO.cpp` | 1809930c0 | 1809932f0 | 5 | 693 | bloc:1 release:4 |  |  |
| `CivTech\Platform\Common\Platform_TempHeap.cpp` | 1809934b0 | 1809934b0 | 1 | 42 | chaine:1 |  |  |
| `CivTech\Platform\Windows\Platform_DialogTemplate.cpp` | - | - | 0 | 0 | | | |
| `CivTech\String\Common\String_Basic.cpp` | 180993b30 | 180993b30 | 1 | 107 | chaine:1 |  |  |
| `CivTech\String\Common\String_PlatformFunctions.cpp` | 180993fc0 | 180993fc0 | 1 | 16 | release:1 |  |  |
| `CivTech\String\Common\String_Functions.cpp` | 180994300 | 180994330 | 2 | 54 | release:2 |  |  |
| `CivTech\Telemetry\src\Telemetry.cpp` | 1809947a0 | 1809947a0 | 1 | 59 | chaine:1 |  |  |
| `G:\BuildAgent\work\26d31e7d2eca7bbe\CivTech\SDKSources\EASTL\src\fixed_pool.cpp` | - | - | 0 | 0 | | | |
| `CivTech\AutoVariable\FAutoArchive.cpp` | 180995d70 | 180996d40 | 12 | 4120 | bloc:4 chaine:1 fichier:1 release:6 | 709 |  |
| `CivTech\Database\source\Utility\DatabaseUtility_QueryCache.cpp` | 180997ff0 | 180997ff0 | 1 | 175 | release:1 |  |  |
| `CivTech\DataVariant\Data_TypedVariantManager.cpp` | 18099d860 | 18099e080 | 12 | 2116 | bloc:7 fichier:5 | 272 |  |
| `CivTech\DataVariant\Data_TypedVariantMap.cpp` | 18099e9d0 | 18099f740 | 15 | 3734 | bloc:5 fichier:2 release:8 | 964 |  |
| `CivTech\DataVariant\Data_TypedVariant.cpp` | 1809a1270 | 1809a1320 | 4 | 730 | bloc:1 fichier:1 release:2 |  |  |
| `CivTech\DataVariant\Data_Variant.cpp` | 1809a1620 | 1809a44f0 | 52 | 12320 | bloc:48 fichier:4 | 1433 |  |
| `CivTech\DataVariant\Data_TypedVariantVectorMap.cpp` | 1809a52c0 | 1809a5da0 | 7 | 2713 | fichier:2 release:5 | 932 |  |
| `CivTech\DataVariant\Data_Variant_JSONSerialization.cpp` | 1809a89d0 | 1809a8a60 | 2 | 274 | fichier:2 | 146 |  |
| `CivTech\DataVariant\Data_TypedVariantKeyedContainer.cpp` | 1809a9a40 | 1809a9a40 | 1 | 852 | release:1 |  |  |
| `CivTech\Serialization\Common\Serialization_MemoryStreams.cpp` | 1809a9ec0 | 1809aa140 | 12 | 755 | bloc:2 release:10 |  |  |
| `CivTech\XML\XML.cpp` | 1809ad400 | 1809ad780 | 6 | 920 | release:6 |  |  |
| `CivTech\Lua\Lua_Common.cpp` | 1809add10 | 1809add10 | 1 | 367 | release:1 |  |  |
| `CivTech\Lua\Lua_Types.cpp` | 1809ae120 | 1809aeaa0 | 15 | 2507 | bloc:1 chaine:1 release:13 |  |  |
| `CivTech\Lua\Lua_DataVariantSupport.cpp` | 1809af0b0 | 1809af470 | 2 | 2053 | fichier:2 | 611 |  |
