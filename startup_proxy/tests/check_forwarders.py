"""Test hors jeu du proxy de démarrage : renvois des exports et ligne de journal.

On reproduit l'installation dans un dossier temporaire : le proxy sous le nom
DatabaseDLL_FinalRelease.dll, l'originale sous DatabaseDLL_FinalRelease_orig.dll. Chaque export du
proxy doit être résolu par le chargeur dans l'originale. Aucune fonction n'est appelée.

Usage (LOCALAPPDATA redirigé pour ne pas écrire dans le vrai journal) :
    python -I check_forwarders.py <proxy compilé> <DatabaseDLL_FinalRelease.dll d'origine> <dossier temporaire>
"""
import ctypes
import os
import shutil
import sys
from ctypes import wintypes

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "proxy"))
from gen_exports import exports  # noqa: E402

proxy_src, original_src, tmp = sys.argv[1:4]
game = os.path.join(tmp, "Win64Steam")
os.makedirs(game, exist_ok=True)
proxy = os.path.join(game, "DatabaseDLL_FinalRelease.dll")
original = os.path.join(game, "DatabaseDLL_FinalRelease_orig.dll")
shutil.copyfile(proxy_src, proxy)
shutil.copyfile(original_src, original)
os.environ["LOCALAPPDATA"] = tmp
log_file = os.path.join(tmp, "Civ6RolloverProxy", "RolloverProxy.log")
if os.path.exists(log_file):
    os.remove(log_file)

k32 = ctypes.WinDLL("kernel32", use_last_error=True)
k32.LoadLibraryW.restype = wintypes.HMODULE
k32.LoadLibraryW.argtypes = [wintypes.LPCWSTR]
k32.GetModuleHandleW.restype = wintypes.HMODULE
k32.GetModuleHandleW.argtypes = [wintypes.LPCWSTR]
k32.GetProcAddress.restype = ctypes.c_void_p
k32.GetProcAddress.argtypes = [wintypes.HMODULE, ctypes.c_char_p]

# Le renvoi vise un nom de module sans chemin : le chargeur le cherche dans le dossier de l'exe
# (ici python.exe), comme le jeu dans Win64Steam. On ajoute donc le dossier du test à la recherche.
k32.SetDllDirectoryW(game)
h = k32.LoadLibraryW(proxy)
assert h, f"chargement du proxy impossible : {ctypes.get_last_error()}"
status = ctypes.WinDLL(proxy).StartupProxyStatus
status.restype = wintypes.DWORD
assert status() & 1, "DllMain du proxy non appelée"

names = exports(original_src)
assert len(names) == 5, names
for name in names:
    addr = k32.GetProcAddress(h, name.encode())
    assert addr, f"{name} : non résolu ({ctypes.get_last_error()})"
    orig = k32.GetModuleHandleW("DatabaseDLL_FinalRelease_orig.dll")
    assert orig, "l'originale n'a pas été chargée par le renvoi"
    assert addr == k32.GetProcAddress(orig, name.encode()), f"{name} : ne vise pas l'originale"
    print(f"ok {name} -> {addr:#x}")

text = open(log_file, encoding="utf-8").read()
print("journal :\n  " + text.strip().replace("\n", "\n  "))
assert "démarrage : proxy DatabaseDLL chargé" in text
print("tout est bon")
