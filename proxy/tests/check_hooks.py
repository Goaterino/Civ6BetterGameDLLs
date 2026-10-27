"""Test hors jeu du proxy de GameCore : exports, chargement de l'originale, détours.

On reproduit l'installation dans un dossier temporaire : le proxy sous le nom
GameCore_XP2_FinalRelease.dll, l'originale sous GameCore_XP2_FinalRelease_orig.dll.

L'originale refuse de s'initialiser hors du jeu (erreur 1114). On la projette donc d'abord avec
DONT_RESOLVE_DLL_REFERENCES : le LoadLibraryW du proxy récupère alors ce module sans exécuter son
DllMain. Aucune fonction du jeu n'est appelée ; on relit les octets écrits aux deux sites.

Usage (LOCALAPPDATA redirigé pour ne pas écrire dans le vrai journal) :
    python -I check_hooks.py <proxy compilé> <GameCore_XP2_FinalRelease.dll d'origine> <dossier temporaire>
"""
import ctypes
import os
import shutil
import struct
import sys
from ctypes import wintypes

proxy_src, original_src, tmp = sys.argv[1:4]
game = os.path.join(tmp, "Win64")
os.makedirs(game, exist_ok=True)
proxy = os.path.join(game, "GameCore_XP2_FinalRelease.dll")
original = os.path.join(game, "GameCore_XP2_FinalRelease_orig.dll")
shutil.copyfile(proxy_src, proxy)
shutil.copyfile(original_src, original)
with open(os.path.join(game, "rollover_proxy.cfg"), "w") as f:
    f.write("mode=count\n")
os.environ["LOCALAPPDATA"] = tmp
log_file = os.path.join(tmp, "Civ6RolloverProxy", "RolloverProxy.log")
if os.path.exists(log_file):
    os.remove(log_file)

k32 = ctypes.WinDLL("kernel32", use_last_error=True)
k32.LoadLibraryW.restype = wintypes.HMODULE
k32.LoadLibraryExW.restype = wintypes.HMODULE
k32.LoadLibraryExW.argtypes = [wintypes.LPCWSTR, wintypes.HANDLE, wintypes.DWORD]

DONT_RESOLVE_DLL_REFERENCES = 0x1
base = k32.LoadLibraryExW(original, None, DONT_RESOLVE_DLL_REFERENCES)
assert base, f"projection de l'originale impossible : {ctypes.get_last_error()}"

h = k32.LoadLibraryW(proxy)
assert h, f"chargement du proxy impossible : {ctypes.get_last_error()}"
lib = ctypes.WinDLL(proxy)
status = lib.RolloverProxyStatus
status.restype = wintypes.DWORD
print(f"proxy chargé, état {status():#x} (1 = DllMain)")

load = lib.RolloverProxyLoadOriginal
load.restype = ctypes.c_size_t
got = load()
print(f"originale : {got:#x} (projection préalable en {base:#x}), état {status():#x} (2 = originale, 8 = détours)")
print("journal :\n  " + open(log_file, encoding="utf-8").read().strip().replace("\n", "\n  "))
assert got == base, "le proxy n'a pas récupéré l'originale"
assert status() & 0xB == 0xB, "les détours n'ont pas été posés"

for name, rva in (("instantané", 0x72F80), ("DeclareWar", 0x2A1F90)):
    patch = ctypes.string_at(base + rva, 14)
    assert patch[:6] == b"\xff\x25\x00\x00\x00\x00", f"{name} : pas de saut au site ({patch.hex()})"
    hook = struct.unpack("<Q", patch[6:])[0]
    assert h <= hook < h + 0x400000, f"{name} : le saut ne mène pas au proxy ({hook:#x})"
    print(f"{name} : site {base + rva:#x} -> détour {hook:#x} dans le proxy")

# Le proxy n'exporte que les noms de l'originale, plus DllMain et les deux exports de diagnostic.
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from gen_exports import exports  # noqa: E402

extra = set(exports(proxy)) - set(exports(original))
missing = set(exports(original)) - set(exports(proxy))
assert not missing, f"exports manquants : {sorted(missing)}"
assert extra == {"DllMain", "RolloverProxyStatus", "RolloverProxyLoadOriginal"}, f"exports en trop : {sorted(extra)}"
print(f"exports : les {len(exports(original))} de l'originale, plus {sorted(extra)}")

# Déchargement du proxy (l'originale reste) : les octets d'origine doivent revenir aux deux sites.
k32.FreeLibrary.argtypes = [wintypes.HMODULE]
k32.GetModuleHandleW.restype = wintypes.HMODULE
while k32.GetModuleHandleW(proxy):
    k32.FreeLibrary(lib._handle)
for name, rva, prologue in (("instantané", 0x72F80, "48895c2410574883ec304863da8bf9"),
                            ("DeclareWar", 0x2A1F90, "83faff0f840a020000488bc448894808")):
    now = ctypes.string_at(base + rva, len(prologue) // 2).hex()
    assert now == prologue, f"{name} : octets non restaurés après déchargement ({now})"
    print(f"{name} : octets d'origine restaurés après déchargement du proxy")
print("OK")
