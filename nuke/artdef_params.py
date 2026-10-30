"""Affiche les paramètres d'éléments d'un artdef, hors valeurs nulles ou fausses.

    python -I nuke/artdef_params.py <fichier .artdef> <collection> <élément>...
"""
import sys
import xml.etree.ElementTree as ET


def value(v):
    cls = v.get("class", "").split("..")[-1]
    for tag, attr in (("m_fValue", None), ("m_nValue", None), ("m_bValue", None), ("m_Value", "text"),
                      ("m_ElementName", "text"), ("m_EntryName", "text")):
        e = v.find(tag)
        if e is not None:
            return cls, (e.get(attr) if attr else e.text)
    return cls, None


def main():
    root = ET.parse(sys.argv[1]).getroot()
    wanted = set(sys.argv[3:])
    for coll in root.find("m_RootCollections"):
        if coll.find("m_CollectionName").get("text") != sys.argv[2]:
            continue
        for el in coll.findall("Element"):
            name = el.find("m_Name").get("text")
            if name not in wanted:
                continue
            print(f"== {name}")
            for v in el.find("m_Fields").find("m_Values"):
                cls, val = value(v)
                if val in (None, "", "false", "0.000000", "0"):
                    continue
                print(f"   {v.find('m_ParamName').get('text'):28} {cls:22} {val}")
            children = el.find("m_ChildCollections")
            for child in children if children is not None else []:
                print(f"   [collection enfant] {child.find('m_CollectionName').get('text')}")


if __name__ == "__main__":
    main()
