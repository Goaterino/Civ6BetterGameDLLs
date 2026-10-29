"""Construit le mod « Nuke Quick Vis », qui raccourcit la visualisation des WMD.

    python -I perf/nuke/build_mod.py [--vitesse 10] [--gravite 50] [--impact AtomBomb_Hit]
        [--modele-silo Special/TargetPlot] [--dest <dossier Mods>]

Il recopie les six attaques WMD de Base/ArtDefs/Units.artdef dans un Units.artdef partiel et
multiplie leur ProjectileSpeed et leur ProjectileGravity. Il peut aussi remplacer leur VFXImpact,
et le modèle avec lequel le silo tire (UnitMemberTypes/MissileSilo), pour supprimer le décollage.
Il écrit ensuite le .modinfo et le .dep à côté. Le mod ne touche que l'affichage
(AffectsSavedGames = 0).
"""
import argparse
import copy
import pathlib
import xml.etree.ElementTree as ET

GAME = pathlib.Path(r"C:/Program Files (x86)/Steam/steamapps/common/Sid Meier's Civilization VI")
MODS = pathlib.Path.home() / "Documents/My Games/Sid Meier's Civilization VI/Mods"
NAME = "NukeQuickVis"
MOD_ID = "6f1c2d7e-4b8a-4e51-9c3f-2a7d9e0b1c64"
DEP_ID = "0b9e7c41-5d2f-4a6e-8f13-7c4b2e9a6d05"
ATTACKS = [
    "MissileSilo_ABOMB_Attack", "MissileSilo_HBOMB_Attack",
    "Bomber_ABOMB_Hit", "Bomber_HBOMB_Hit",
    "NuclearSubmarine_ABOMB_Hit", "NuclearSubmarine_HBOMB_Hit",
]

MODINFO = f"""<?xml version="1.0" encoding="utf-8"?>
<Mod id="{MOD_ID}" version="1">
  <Properties>
    <Name>Nuke Quick Vis</Name>
    <Description>Raccourcit l'animation des armes nucléaires (affichage seulement).</Description>
    <Authors>Tim</Authors>
    <AffectsSavedGames>0</AffectsSavedGames>
    <CompatibleVersions>2.0</CompatibleVersions>
  </Properties>
  <Dependencies>
    <Mod id="4873eb62-8ccc-4574-b784-dda455e74e68" title="LOC_EXPANSION2_MOD_TITLE"/>
  </Dependencies>
  <FrontEndActions>
    <UpdateArt id="NukeQuickVis_Art">
      <File>{NAME}.dep</File>
    </UpdateArt>
  </FrontEndActions>
  <InGameActions>
    <UpdateArt id="NukeQuickVis_Art">
      <Properties>
        <LoadOrder>1000</LoadOrder>
      </Properties>
      <File>{NAME}.dep</File>
    </UpdateArt>
  </InGameActions>
  <Files>
    <File>{NAME}.dep</File>
    <File>ArtDefs/Units.artdef</File>
  </Files>
</Mod>
"""

DEP = f"""<?xml version="1.0" encoding="UTF-8" ?>
<AssetObjects..GameDependencyData>
	<ID>
		<name text="{NAME}"/>
		<id text="{DEP_ID}"/>
	</ID>
	<RequiredGameArtIDs>
		<Element>
			<name text="Expansion2"/>
			<id text="b1b63999-6b16-4dd2-a5b6-eb19794aa8ca"/>
		</Element>
	</RequiredGameArtIDs>
	<SystemDependencies>
		<Element>
			<ConsumerName text="Units"/>
			<ArtDefDependencyPaths>
				<Element text="Units.artdef"/>
			</ArtDefDependencyPaths>
			<LibraryDependencies>
				<Element text="Unit"/>
				<Element text="VFX"/>
				<Element text="Light"/>
			</LibraryDependencies>
			<LoadsLibraries>true</LoadsLibraries>
		</Element>
	</SystemDependencies>
	<ArtDefDependencies>
		<Element>
			<ArtDefPath text="Units.artdef"/>
			<ArtDefDependencyPaths>
				<Element text="Units.artdef"/>
			</ArtDefDependencyPaths>
		</Element>
	</ArtDefDependencies>
	<Libraries/>
</AssetObjects..GameDependencyData>
"""


def param(el, name):
    for v in el.find("m_Fields").find("m_Values"):
        if v.find("m_ParamName").get("text") == name:
            return v
    raise KeyError(name)


def collection(base, name):
    return next(c for c in base.find("m_RootCollections") if c.find("m_CollectionName").get("text") == name)


def add_collection(root, name):
    coll = ET.SubElement(root.find("m_RootCollections"), "Element")
    ET.SubElement(coll, "m_CollectionName", text=name)
    ET.SubElement(coll, "m_ReplaceMergedCollectionElements").text = "false"
    return coll


def build_artdef(speed, gravity, impact, silo_bin):
    base = ET.parse(GAME / "Base/ArtDefs/Units.artdef").getroot()
    root = ET.Element("AssetObjects..ArtDefSet")
    root.append(copy.deepcopy(base.find("m_Version")))
    root.append(copy.deepcopy(base.find("m_TemplateName")))
    ET.SubElement(root, "m_RootCollections")
    coll = add_collection(root, "CombatAttack")
    found = []
    for el in collection(base, "CombatAttack").findall("Element"):
        name = el.find("m_Name").get("text")
        if name not in ATTACKS:
            continue
        el = copy.deepcopy(el)
        v = param(el, "ProjectileSpeed").find("m_fValue")
        v.text = f"{float(v.text) * speed:.6f}"
        g = param(el, "ProjectileGravity").find("m_fValue")
        g.text = f"{float(g.text) * gravity:.6f}"
        if impact:
            param(el, "VFXImpact").find("m_ElementName").set("text", impact)
        coll.append(el)
        found.append(name)
    missing = set(ATTACKS) - set(found)
    if missing:
        raise SystemExit(f"attaques absentes de Units.artdef : {sorted(missing)}")
    if silo_bin:
        # Le décollage est une animation du modèle Special/MissileSilo (units.blp), que l'artdef ne
        # règle pas : on fait tirer le silo avec un autre modèle.
        silo = next(e for e in collection(base, "UnitMemberTypes").findall("Element")
                    if e.find("m_Name").get("text") == "MissileSilo")
        silo = copy.deepcopy(silo)
        bins = [e for e in silo.iter("Element") if e.find("m_CollectionName") is not None
                and e.find("m_CollectionName").get("text") == "Bins"]
        for b in bins:
            for e in b.findall("Element"):
                e.find("m_Name").set("text", silo_bin)
        if not bins:
            raise SystemExit("collection Bins introuvable dans UnitMemberTypes/MissileSilo")
        add_collection(root, "UnitMemberTypes").append(silo)
    ET.indent(root, space="\t")
    return '<?xml version="1.0" encoding="UTF-8" ?>\n' + ET.tostring(root, encoding="unicode") + "\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--vitesse", type=float, default=10.0, help="facteur appliqué à ProjectileSpeed")
    ap.add_argument("--gravite", type=float, help="facteur appliqué à ProjectileGravity (défaut : vitesse² / 2)")
    ap.add_argument("--impact", help="VFXImpact de remplacement (élément de VFX.artdef)")
    ap.add_argument("--modele-silo", help="modèle de Unit_Bins.artdef avec lequel le silo tire, par ex. Special/TargetPlot")
    ap.add_argument("--dest", type=pathlib.Path, default=MODS)
    a = ap.parse_args()
    gravity = a.gravite if a.gravite is not None else a.vitesse ** 2 / 2
    out = a.dest / NAME
    (out / "ArtDefs").mkdir(parents=True, exist_ok=True)
    (out / "ArtDefs/Units.artdef").write_text(build_artdef(a.vitesse, gravity, a.impact, a.modele_silo),
                                              encoding="utf-8")
    (out / f"{NAME}.dep").write_text(DEP, encoding="utf-8")
    (out / f"{NAME}.modinfo").write_text(MODINFO, encoding="utf-8")
    print(f"mod écrit dans {out} (vitesse ×{a.vitesse:g}, gravité ×{gravity:g}, "
          f"impact {a.impact or 'inchangé'}, modèle du silo {a.modele_silo or 'inchangé'})")


if __name__ == "__main__":
    main()
