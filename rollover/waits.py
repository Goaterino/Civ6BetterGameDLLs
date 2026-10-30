"""Attentes d'un thread pendant un passage de tour, d'après les changements de contexte d'une trace WPR.

Pour le thread donné (en général le thread GameCore), dans la fenêtre donnée :
- le temps hors CPU, par état et raison d'attente ;
- où il attend (pile à la reprise) et qui le réveille (pile du thread qui réveille) ;
- les plus longues attentes ;
- ce que fait le thread qui réveille pendant les attentes de plus de 20 ms, par module et par branche
  de sa boucle principale (cadres sous exe+0x6ff769).

Usage :
    python -I rollover/waits.py <dossier de capture> <tid> <début ISO> <fin ISO> [tid qui réveille]
"""
import sys, pathlib, collections, bisect, datetime as dt
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import analyze
cap = pathlib.Path(sys.argv[1]); G = int(sys.argv[2])
t0 = dt.datetime.fromisoformat(sys.argv[3]); t1 = dt.datetime.fromisoformat(sys.argv[4])
tr = analyze.load_trace(cap / "trace.etl")
start = dt.datetime(1601,1,1,tzinfo=dt.timezone.utc) + dt.timedelta(microseconds=tr.start_filetime/10)
def wall(q): return (start + dt.timedelta(seconds=(q-tr.first_qpc)/tr.perf_freq)).astimezone().replace(tzinfo=None)
def ms(dq): return dq * 1000 / tr.perf_freq
pid = [i for i in tr.images if i[3].startswith("civilizationvi")][0][0]
mods = sorted({(b, s, n) for p, b, s, n in tr.images if p == pid})
if any(n == analyze.GAMECORE_ORIG for _, _, n in mods):
    ren = {analyze.GAMECORE: analyze.PROXY, analyze.GAMECORE_ORIG: analyze.GAMECORE}
    mods = [(b, s, ren.get(n, n)) for b, s, n in mods]
starts = [m[0] for m in mods]
funcs = analyze.Functions(analyze.FUNCTIONS_TSV)
SYS = {"noyau", "ntdll.dll", "kernelbase.dll", "kernel32.dll", "?", "win32u.dll", "ucrtbase.dll", "vcruntime140.dll", "msvcp140.dll"}
def where(ip):
    if ip >= 0xFFFF800000000000: return "noyau", None
    i = bisect.bisect_right(starts, ip) - 1
    if i >= 0 and ip < mods[i][0] + mods[i][1]:
        b, s, n = mods[i]
        if n == analyze.GAMECORE:
            r = funcs.find(ip - b + analyze.GAMECORE_BASE)
            return n, (funcs.label(r["adresse"]) if r else f"+{ip-b:#x}")
        return n, f"+{ip-b:#x}"
    return "?", None
def site(frames, depth=3):
    out = []
    for ip in frames:
        n, f = where(ip)
        if n in SYS and not out: continue
        out.append(f"{n}{' ' + f if f and n == analyze.GAMECORE else (f if f else '')}")
        if len(out) >= depth: break
    return " < ".join(out) or "?"
qa = min(q for q, *_ in tr.cswitches if wall(q) >= t0)
cs = sorted(tr.cswitches)
ready = sorted(r for r in tr.readies if r[2] == G)
rts = [r[0] for r in ready]
waits = []
out_at = None
for q, new, old, reason, state, wait in cs:
    w = wall(q)
    if w < t0 or w >= t1: continue
    if old == G: out_at = (q, reason, state)
    if new == G and out_at:
        dq = q - out_at[0]
        i = bisect.bisect_right(rts, q) - 1
        rd = ready[i] if i >= 0 and ready[i][0] >= out_at[0] else None
        waits.append((dq, out_at[1], out_at[2], q, rd))
        out_at = None
tot = sum(w[0] for w in waits)
print(f"fenêtre {t0:%H:%M:%S.%f}–{t1:%H:%M:%S.%f} : {len(waits)} sorties du CPU, {ms(tot):.0f} ms hors CPU")
REASONS = {0:"Executive",6:"UserRequest",7:"WrExecutive",13:"WrUserRequest",15:"WrQueue",16:"WrLpcReceive",23:"WrPreempted",31:"WrAlertByThreadId",32:"WrDeferredPreempt",4:"Suspended",5:"Delay?",}
STATES = {1:"Ready(préempté)",5:"Waiting",2:"Running"}
by = collections.Counter(); n = collections.Counter()
for dq, reason, state, q, rd in waits:
    k = f"{STATES.get(state, state)} / {REASONS.get(reason, reason)}"; by[k] += dq; n[k] += 1
print("\n## par état et raison"); [print(f"{ms(v):8.0f} ms  {n[k]:5} fois  {k}") for k, v in by.most_common(8)]
by = collections.Counter(); n = collections.Counter(); ex = {}
for dq, reason, state, q, rd in waits:
    if state != 5: continue
    k = site(tr.stacks.get((q, G), []))
    by[k] += dq; n[k] += 1
print("\n## où le thread GameCore attend (pile à la reprise)"); [print(f"{ms(v):8.0f} ms  {n[k]:5} fois  {k}") for k, v in by.most_common(12)]
by = collections.Counter(); n = collections.Counter()
for dq, reason, state, q, rd in waits:
    if state != 5: continue
    if rd and rd[1] is not None:
        name = tr.thread_name.get(rd[1], "") ; p = tr.thread_pid.get(rd[1])
        k = f"thread {rd[1]}{' ' + name if name else ''}{'' if p == pid else f' (pid {p})'} : {site(tr.stacks.get((rd[0], rd[1]), []), 4)}"
    else: k = "réveilleur inconnu"
    by[k] += dq; n[k] += 1
print("\n## qui le réveille"); [print(f"{ms(v):8.0f} ms  {n[k]:5} fois  {k}") for k, v in by.most_common(12)]
big = sorted(waits, reverse=True)[:15]
print("\n## plus longues attentes")
for dq, reason, state, q, rd in big:
    who = f"réveil par {rd[1]}" if rd else ""
    print(f"{ms(dq):7.1f} ms  fin {wall(q):%S.%f}  {STATES.get(state,state)}/{REASONS.get(reason,reason)}  {who}  attend: {site(tr.stacks.get((q,G),[]),3)}")

# Que fait le thread qui réveille pendant les attentes longues (> 20 ms) ?
M = int(sys.argv[5]) if len(sys.argv) > 5 else 8724
iv = [(q - dq, q) for dq, reason, state, q, rd in waits if state == 5 and ms(dq) > 20]
ivs = sorted(iv); lo = [a for a, b in ivs]
def inside(q):
    i = bisect.bisect_right(lo, q) - 1
    return i >= 0 and q < ivs[i][1]
mod = collections.Counter(); leaf = collections.Counter(); chain = collections.Counter(); k = 0
for q, tid, ip in tr.samples:
    if tid != M or not inside(q): continue
    k += 1
    n, f = where(ip); mod[n] += 1
    fr = tr.stacks.get((q, tid), [])
    names = [where(x) for x in fr if x < 0xFFFF800000000000]
    exe = [f"{n}{f}" for n, f in names if n not in SYS][:6]
    chain[" < ".join(exe[-4:])] += 1
print(f"\n## thread {M} pendant les {len(iv)} attentes > 20 ms ({sum(ms(b-a) for a,b in iv):.0f} ms) : {k} échantillons")
for n, v in mod.most_common(10): print(f"{v:6} ms  {n}")
print("\n## bas de pile (4 cadres hors système les plus externes)")
for c, v in chain.most_common(12): print(f"{v:6} ms  {c}")

# Branche de la boucle principale : les cadres sous exe+0x6ff769, du plus externe au plus interne.
def off(ip):
    n, f = where(ip); return (n, f)
br = collections.Counter(); brmod = collections.defaultdict(collections.Counter)
for q, tid, ip in tr.samples:
    if tid != M or not inside(q): continue
    fr = [off(x) for x in reversed(tr.stacks.get((q, tid), [])) if x < 0xFFFF800000000000]
    keys = [f for n, f in fr]
    if "+0x6ff769" in keys:
        i = keys.index("+0x6ff769")
        k = " > ".join(f"{n.split('.')[0][:12]}{f if n != analyze.GAMECORE else ''}" for n, f in fr[i + 1:i + 4])
    else:
        k = "hors boucle : " + " > ".join(f"{n.split('.')[0][:12]}{f or ''}" for n, f in fr[:4])
    br[k] += 1; brmod[k][where(ip)[0]] += 1
print("\n## thread", M, "pendant les attentes : branches sous exe+0x6ff769")
for k, v in br.most_common(14):
    print(f"{v:6} ms  {k}   [{', '.join(f'{m} {c}' for m, c in brmod[k].most_common(3))}]")
