"""Choix des cases travaillées : le sac à dos du jeu (18010f050) contre le choix exact.

    python -I citoyens/simulation.py

- `jeu` réimplémente 18010f050 : programmation dynamique ligne par ligne, où une combinaison
  remplace l'autre si elle franchit le seuil de nourriture T par le bas, sinon si son score
  (favori, autre) est plus grand. Cette comparaison n'est pas un ordre : le choix dépend de l'ordre
  des cases et peut laisser une 2/2 libre pour une 2/1.
- `exact` est l'algorithme du proxy (proxy/src/citoyens.rs) : au plus K emplacements ; parmi ceux
  qui atteignent T s'il y en a, le meilleur (favori, autre), sinon le meilleur tout court ; à
  égalité, le moins d'emplacements.
- On compare `exact` à une recherche exhaustive, et on compte les cas où le jeu fait moins bien.
"""
import itertools
import random


def jeu(items, K, T):
    """items : (nourriture, favori, autre). Rend les indices choisis par l'original."""
    prev = [((0, 0, 0), ()) for _ in range(K + 1)]
    for idx, (f, fa, o) in enumerate(items):
        cur = [prev[0]]
        for k in range(1, K + 1):
            old, base = prev[k], prev[k - 1]
            new = ((base[0][0] + f, base[0][1] + fa, base[0][2] + o), base[1] + (idx,))
            if old[0][0] < T <= new[0][0]:
                take = True
            else:
                take = (new[0][1], new[0][2]) > (old[0][1], old[0][2])
            cur.append(new if take else old)
        prev = cur
    return list(prev[K][1])


def exact(items, K, T):
    n = len(items)
    K, T = min(K, n), max(T, 0)
    best = [[None] * (T + 1) for _ in range(K + 1)]
    best[0][0] = (0, 0)
    took = []
    for f, fa, o in items:
        t, new = {}, [row[:] for row in best]
        for k in range(1, K + 1):
            for c0 in range(T + 1):
                b = best[k - 1][c0]
                if b is None:
                    continue
                c = min(T, c0 + f)
                cand = (b[0] + fa, b[1] + o)
                if new[k][c] is None or cand > new[k][c]:
                    new[k][c] = cand
                    t[(k, c)] = c0
        took.append(t)
        best = new

    def pick(cs):
        r = None
        for k in range(K + 1):
            for c in cs:
                v = best[k][c]
                if v is not None and (r is None or v > r[0]):
                    r = (v, k, c)
        return r

    _, k, c = pick([T]) or pick(range(T + 1))
    sel = []
    for i in range(n - 1, -1, -1):
        if (k, c) in took[i]:
            c = took[i][(k, c)]
            k -= 1
            sel.append(i)
    return sorted(sel)


def cle(items, sel, T):
    f = sum(items[i][0] for i in sel)
    return (f >= T, sum(items[i][1] for i in sel), sum(items[i][2] for i in sel), -len(sel))


def exhaustif(items, K, T):
    return max(cle(items, s, T) for k in range(min(K, len(items)) + 1) for s in itertools.combinations(range(len(items)), k))


def case(F, P, G, fav_f, fav_p):
    """Une case (nourriture, production, or) notée comme 180110ba0 : rendements doublés sauf l'or."""
    fa = 2 * F * fav_f + 2 * P * fav_p
    o = 2 * F * (not fav_f) + 2 * P * (not fav_p) + G
    return (F, fa, o)


if __name__ == "__main__":
    random.seed(7)
    ecarts = moins_bien = 0
    essais = 30000
    for _ in range(essais):
        n = random.randint(1, 9)
        ff, fp = random.random() < 0.6, random.random() < 0.6
        items = [case(random.randint(0, 5), random.randint(0, 5), random.randint(0, 3), ff, fp) for _ in range(n)]
        K = random.randint(0, n + 1)
        T = random.choice([0, 0, random.randint(0, 14)])
        sel = exact(items, K, T)
        if cle(items, sel, T) != exhaustif(items, K, T) or len(sel) > K:
            ecarts += 1
        if cle(items, jeu(items, K, T), T)[:3] < cle(items, sel, T)[:3]:
            moins_bien += 1
    print(f"exact contre exhaustif : {ecarts} écart(s) sur {essais}")
    print(f"le jeu fait moins bien dans {moins_bien} cas sur {essais}")
    cases = [(2, 2), (2, 2), (3, 2), (1, 4), (2, 1)]
    items = [case(f, p, 0, True, True) for f, p in cases]
    print("deux citoyens, déficit 5, focus nourriture et production, cases", cases)
    print("  jeu   :", [cases[i] for i in jeu(items, 2, 5)])
    print("  exact :", [cases[i] for i in exact(items, 2, 5)])
