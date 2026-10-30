"""Ce que font les threads GameCore et principal pendant chaque visualisation longue d'un WMD.

Les fenêtres viennent du journal de hook_nuke.py : chaque paire exe_visBegin → exe_visEnd de plus
de 2 s. Le thread GameCore est celui du dernier gc_*Results qui précède. Pour chaque fenêtre :
- temps CPU échantillonné de chaque thread ;
- ses attentes qui chevauchent la fenêtre, même commencées avant ou finies après, avec la pile à la
  reprise et la pile du thread qui réveille ;
- pour le thread principal, les branches sous la boucle principale (exe+0x6ff769).

Usage :
    python -I nuke/attentes.py <journal frida.log> <dossier de capture>
"""
import bisect
import collections
import datetime as dt
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "rollover"))
import analyze  # noqa: E402

SYS = {"noyau", "ntdll.dll", "kernelbase.dll", "kernel32.dll", "?", "win32u.dll", "ucrtbase.dll",
       "vcruntime140.dll", "msvcp140.dll"}
REASONS = {0: "Executive", 6: "UserRequest", 7: "WrExecutive", 13: "WrUserRequest", 15: "WrQueue",
           16: "WrLpcReceive", 23: "WrPreempted", 31: "WrAlertByThreadId", 32: "WrDeferredPreempt"}


def windows(log):
    rows = []
    for line in open(log, encoding="utf-8"):
        t, tid, kind, data = line.rstrip("\n").split("\t", 3)
        rows.append((dt.datetime.fromisoformat(t), int(tid) if tid != "-" else None, kind, data))
    out, gc_tid, begin = [], None, None
    for t, tid, kind, data in rows:
        if kind in ("gc_icbmResults", "gc_wmdResults"):
            gc_tid = tid
        elif kind == "exe_visBegin":
            begin = (t, tid, json.loads(data), gc_tid)
        elif kind == "exe_visEnd" and begin:
            if (t - begin[0]).total_seconds() > 2:
                out.append((begin[0], t, begin[1], begin[3], begin[2]))
            begin = None
    return out


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    log, cap = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    wins = windows(log)
    tr = analyze.load_trace(cap / "trace.etl")
    start = dt.datetime(1601, 1, 1, tzinfo=dt.timezone.utc) + dt.timedelta(microseconds=tr.start_filetime / 10)

    def wall(q):
        return (start + dt.timedelta(seconds=(q - tr.first_qpc) / tr.perf_freq)).astimezone().replace(tzinfo=None)

    def ms(dq):
        return dq * 1000 / tr.perf_freq

    pid = [i for i in tr.images if i[3].startswith("civilizationvi")][0][0]
    mods = sorted({(b, s, n) for p, b, s, n in tr.images if p == pid})
    if any(n == analyze.GAMECORE_ORIG for _, _, n in mods):
        ren = {analyze.GAMECORE: analyze.PROXY, analyze.GAMECORE_ORIG: analyze.GAMECORE}
        mods = [(b, s, ren.get(n, n)) for b, s, n in mods]
    starts = [m[0] for m in mods]
    funcs = analyze.Functions(analyze.FUNCTIONS_TSV)

    def where(ip):
        if ip >= 0xFFFF800000000000:
            return "noyau", ""
        i = bisect.bisect_right(starts, ip) - 1
        if i >= 0 and ip < mods[i][0] + mods[i][1]:
            b, s, n = mods[i]
            if n == analyze.GAMECORE:
                r = funcs.find(ip - b + analyze.GAMECORE_BASE)
                return n, " " + (funcs.label(r["adresse"]) if r else f"+{ip - b:#x}")
            return n, f"+{ip - b:#x}"
        return "?", ""

    def site(frames, depth=5):
        out = []
        for ip in frames:
            n, f = where(ip)
            if n in SYS and not out:
                continue
            out.append(f"{n}{f}")
            if len(out) >= depth:
                break
        return " < ".join(out) or "?"

    cs = sorted(tr.cswitches, key=lambda c: c[0])
    readies = sorted(tr.readies, key=lambda r: r[0])
    samples = sorted(tr.samples, key=lambda s: s[0])
    sq = [s[0] for s in samples]

    def offcpu(tid, q0, q1):
        """Intervalles hors CPU de tid qui chevauchent [q0, q1)."""
        res, out_at = [], None
        for q, new, old, reason, state, wait in cs:
            if old == tid:
                out_at = (q, reason, state)
            elif new == tid and out_at:
                if q > q0 and out_at[0] < q1:
                    res.append((out_at[0], q, out_at[1], out_at[2]))
                out_at = None
                if q >= q1:
                    break
        if out_at and out_at[0] < q1:
            res.append((out_at[0], None, out_at[1], out_at[2]))
        return res

    by_target = collections.defaultdict(list)
    for r in readies:
        by_target[r[2]].append(r)

    def waker(tid, q_resume):
        rs = by_target[tid]
        i = bisect.bisect_right([r[0] for r in rs], q_resume) - 1
        return rs[i] if i >= 0 else None

    def to_q(t):
        lo, hi = tr.first_qpc, tr.first_qpc + int(3600 * tr.perf_freq)
        while hi - lo > 1000:
            mid = (lo + hi) // 2
            lo, hi = (mid, hi) if wall(mid) < t else (lo, mid)
        return lo

    for b, e, main_tid, gc_tid, data in wins:
        q0, q1 = to_q(b), to_q(e)
        print(f"\n# visualisation {b:%H:%M:%S.%f}"[:-3] + f" → {e:%H:%M:%S.%f}"[:-3]
              + f" ({(e - b).total_seconds():.1f} s), membres {data.get('membres')}, case {data.get('x')},{data.get('y')}")
        i0, i1 = bisect.bisect_left(sq, q0), bisect.bisect_left(sq, q1)
        cpu = collections.Counter(s[1] for s in samples[i0:i1])
        for name, tid in (("GameCore", gc_tid), ("principal", main_tid)):
            print(f"\n## thread {name} {tid} : {cpu[tid]} ms de CPU échantillonné")
            for a, z, reason, state in offcpu(tid, q0, q1):
                if z is not None and ms(z - a) < 20:
                    continue
                fin = f"{wall(z):%H:%M:%S.%f}"[:-3] if z else "après la capture"
                dur = f"{ms(z - a):8.0f} ms" if z else "       ? ms"
                print(f"{dur}  {wall(a):%H:%M:%S.%f}"[:-3] + f" → {fin}  {REASONS.get(reason, reason)}")
                if z:
                    print(f"           attend : {site(tr.stacks.get((z, tid), []))}")
                    w = waker(tid, z)
                    if w and w[1] is not None:
                        print(f"           réveillé par {w[1]} à {wall(w[0]):%H:%M:%S.%f}"[:-3]
                              + f" : {site(tr.stacks.get((w[0], w[1]), []), 8)}")
        # Le thread principal : branches sous la boucle principale.
        br = collections.Counter()
        for q, tid, ip in samples[i0:i1]:
            if tid != main_tid:
                continue
            fr = [where(x) for x in reversed(tr.stacks.get((q, tid), [])) if x < 0xFFFF800000000000]
            keys = [f for n, f in fr]
            if "+0x6ff769" in keys:
                i = keys.index("+0x6ff769")
                br[" > ".join(f"{n.split('.')[0][:12]}{f}" for n, f in fr[i + 1:i + 4])] += 1
            else:
                br["hors boucle"] += 1
        print("\n## thread principal : branches sous exe+0x6ff769")
        for k, v in br.most_common(10):
            print(f"{v:6} ms  {k}")


if __name__ == "__main__":
    main()
